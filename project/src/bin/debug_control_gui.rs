//! Debug Control GUI
//! @spec 77210_debug_control.md

use eframe::egui;
use ron::{extensions::Extensions, Options as RonOptions};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use padel_game::resource::{
    compose_effective_config, get_config_value, get_override_value, load_env_profile,
    load_game_config, load_runtime_overrides, override_field_type, save_env_profile,
    save_runtime_overrides, set_override_value, DebugEnvProfile, DebugOverrideType,
    DebugOverrideValue, DebugRuntimeOverrides, DEBUG_ENV_CONFIG_PATH, DEBUG_RUNTIME_CONFIG_PATH,
};

const DEBUG_FIELDS_CONFIG_PATH: &str = "assets/config/debug_fields.ron";
const GAME_CONFIG_PATH: &str = "assets/config/game_config.ron";

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Padel デバッグ制御 GUI")
            .with_inner_size([1040.0, 820.0]),
        ..Default::default()
    };

    let app = DebugControlGuiApp::load().unwrap_or_else(DebugControlGuiApp::with_error);
    let app_name = "Padel デバッグ制御 GUI";
    let run = eframe::run_native(
        app_name,
        options,
        Box::new(|cc| {
            configure_japanese_font(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    if let Err(err) = run {
        eprintln!("Failed to launch {}: {}", app_name, err);
        std::process::exit(1);
    }
}

fn configure_japanese_font(ctx: &egui::Context) {
    let Some(font_bytes) = load_japanese_font_bytes() else {
        return;
    };

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "jp_font".to_string(),
        egui::FontData::from_owned(font_bytes).into(),
    );

    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        family.insert(0, "jp_font".to_string());
    }
    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        family.insert(0, "jp_font".to_string());
    }

    ctx.set_fonts(fonts);
}

fn load_japanese_font_bytes() -> Option<Vec<u8>> {
    let candidates = [
        "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
        "/System/Library/Fonts/ヒラギノ角ゴシック W6.ttc",
        "/System/Library/Fonts/ヒラギノ丸ゴ ProN W4.ttc",
        "/System/Library/Fonts/ヒラギノ明朝 ProN.ttc",
        "/System/Library/Fonts/Language Support/PingFang.ttc",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
    ];

    candidates.iter().find_map(|path| fs::read(path).ok())
}

#[derive(Clone, Copy, Default)]
struct FloatState {
    enabled: bool,
    value: f32,
}

#[derive(Clone)]
struct EnvRow {
    key: String,
    value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct DebugFieldCatalog {
    sections: Vec<DebugFieldSection>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct DebugFieldSection {
    title: String,
    fields: Vec<DebugFieldDef>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
struct DebugFieldDef {
    kind: String,
    key: String,
    label: String,
    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,
}

impl DebugFieldDef {
    fn is_flag(&self) -> bool {
        self.kind.eq_ignore_ascii_case("flag")
    }

    fn is_float(&self) -> bool {
        self.kind.eq_ignore_ascii_case("float")
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn min(&self) -> f32 {
        self.min.unwrap_or(-1000.0)
    }

    fn max(&self) -> f32 {
        self.max.unwrap_or(1000.0)
    }

    fn step(&self) -> f32 {
        self.step.unwrap_or(0.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MainTab {
    Runtime,
    Env,
    Effective,
}

impl MainTab {
    fn title(self) -> &'static str {
        match self {
            Self::Runtime => "実行中上書き",
            Self::Env => "起動時環境変数",
            Self::Effective => "実効値プレビュー",
        }
    }
}

struct DebugControlGuiApp {
    base_config: Option<padel_game::resource::GameConfig>,
    startup_env: DebugRuntimeOverrides,

    catalog: DebugFieldCatalog,
    runtime_enabled: bool,
    runtime_seed: DebugRuntimeOverrides,
    flag_states: BTreeMap<String, bool>,
    float_states: BTreeMap<String, FloatState>,
    active_tab: MainTab,
    runtime_section_index: usize,
    runtime_filter: String,

    env_rows: Vec<EnvRow>,
    new_env_key: String,
    new_env_value: String,

    launch_release: bool,
    status_message: String,
}

impl DebugControlGuiApp {
    fn with_error(error: String) -> Self {
        Self {
            base_config: None,
            startup_env: DebugRuntimeOverrides::from_env(),
            catalog: default_field_catalog(),
            runtime_enabled: false,
            runtime_seed: DebugRuntimeOverrides::default(),
            flag_states: BTreeMap::new(),
            float_states: BTreeMap::new(),
            active_tab: MainTab::Runtime,
            runtime_section_index: 0,
            runtime_filter: String::new(),
            env_rows: Vec::new(),
            new_env_key: String::new(),
            new_env_value: String::new(),
            launch_release: false,
            status_message: error,
        }
    }

    fn load() -> Result<Self, String> {
        let base_path = resolve_project_path(GAME_CONFIG_PATH);
        let base_path = base_path.to_string_lossy().into_owned();
        let runtime_path = resolve_project_path(DEBUG_RUNTIME_CONFIG_PATH);
        let runtime_path = runtime_path.to_string_lossy().into_owned();
        let env_path = resolve_project_path(DEBUG_ENV_CONFIG_PATH);
        let env_path = env_path.to_string_lossy().into_owned();

        let base = load_game_config(&base_path)?;
        let startup_env = DebugRuntimeOverrides::from_env();
        let runtime = load_runtime_overrides(&runtime_path)?;
        let env_profile = load_env_profile(&env_path)?;
        let catalog = load_or_create_field_catalog(DEBUG_FIELDS_CONFIG_PATH)?;

        let (flag_states, float_states) = build_editor_states(&catalog, &base, &runtime);

        let env_rows = env_profile
            .vars
            .into_iter()
            .map(|(key, value)| EnvRow { key, value })
            .collect();

        Ok(Self {
            base_config: Some(base),
            startup_env,
            catalog,
            runtime_enabled: runtime.enabled,
            runtime_seed: runtime,
            flag_states,
            float_states,
            active_tab: MainTab::Runtime,
            runtime_section_index: 0,
            runtime_filter: String::new(),
            env_rows,
            new_env_key: String::new(),
            new_env_value: String::new(),
            launch_release: false,
            status_message: "読み込み完了".to_string(),
        })
    }

    fn env_profile(&self) -> DebugEnvProfile {
        let mut vars = BTreeMap::new();
        for row in &self.env_rows {
            let key = row.key.trim();
            if key.is_empty() {
                continue;
            }
            vars.insert(key.to_string(), row.value.clone());
        }
        DebugEnvProfile { vars }
    }

    fn build_runtime_from_ui(&self) -> Result<DebugRuntimeOverrides, String> {
        let mut overrides = self.runtime_seed.clone();

        for section in &self.catalog.sections {
            for field in &section.fields {
                if field.is_flag() {
                    let checked = self.flag_states.get(field.key()).copied().unwrap_or(false);
                    let value = if checked {
                        Some(DebugOverrideValue::Bool(true))
                    } else {
                        None
                    };
                    set_override_value(&mut overrides, field.key(), value)?;
                } else if field.is_float() {
                    let state = self
                        .float_states
                        .get(field.key())
                        .copied()
                        .unwrap_or_default();
                    let value = if state.enabled {
                        Some(DebugOverrideValue::Float(state.value))
                    } else {
                        None
                    };
                    set_override_value(&mut overrides, field.key(), value)?;
                } else {
                    return Err(format!(
                        "不明なkindです: key='{}', kind='{}'",
                        field.key(),
                        field.kind
                    ));
                }
            }
        }

        overrides.enabled = self.runtime_enabled;
        Ok(overrides)
    }

    fn reload_files(&mut self) {
        match Self::load() {
            Ok(new_state) => *self = new_state,
            Err(err) => self.status_message = format!("再読み込み失敗: {}", err),
        }
    }

    fn save_runtime(&mut self) {
        let mut overrides = match self.build_runtime_from_ui() {
            Ok(v) => v,
            Err(err) => {
                self.status_message = format!("実行中上書きの保存失敗: {}", err);
                return;
            }
        };

        if overrides.has_any_value() && !overrides.enabled {
            overrides.enabled = true;
            self.runtime_enabled = true;
        }

        let runtime_path = resolve_project_path(DEBUG_RUNTIME_CONFIG_PATH);
        let runtime_path = runtime_path.to_string_lossy().into_owned();
        match save_runtime_overrides(&runtime_path, &overrides) {
            Ok(()) => {
                self.runtime_seed = overrides;
                self.status_message = "実行中上書きの保存完了".to_string();
            }
            Err(err) => {
                self.status_message = format!("実行中上書きの保存失敗: {}", err);
            }
        }
    }

    fn save_env(&mut self) {
        let env_path = resolve_project_path(DEBUG_ENV_CONFIG_PATH);
        let env_path = env_path.to_string_lossy().into_owned();
        match save_env_profile(&env_path, &self.env_profile()) {
            Ok(()) => self.status_message = "起動時環境変数の保存完了".to_string(),
            Err(err) => self.status_message = format!("起動時環境変数の保存失敗: {}", err),
        }
    }

    fn save_all(&mut self) {
        self.save_runtime();
        if self.status_message.contains("失敗") {
            return;
        }
        self.save_env();
        if !self.status_message.contains("失敗") {
            self.status_message = "実行中上書きと起動時環境変数を保存しました".to_string();
        }
    }

    fn launch_game(&mut self) {
        let mut cmd = Command::new("cargo");
        cmd.arg("run");
        if self.launch_release {
            cmd.arg("--release");
        }
        cmd.arg("--bin").arg("padel_game");

        let profile = self.env_profile();
        for (k, v) in profile.vars {
            cmd.env(k, v);
        }

        match cmd.spawn() {
            Ok(child) => {
                self.status_message = format!("ゲーム起動プロセス開始 pid={}", child.id());
            }
            Err(err) => {
                self.status_message = format!("ゲーム起動失敗: {}", err);
            }
        }
    }

    fn open_file_default(path: &str) -> Result<String, String> {
        let resolved = resolve_project_path(path);
        if !resolved.exists() {
            return Err(format!("ファイルが存在しません: {}", resolved.display()));
        }

        #[cfg(target_os = "macos")]
        {
            let status = Command::new("open")
                .arg("-t")
                .arg(&resolved)
                .status()
                .map_err(|e| format!("open failed: {}", e))?;
            if !status.success() {
                return Err(format!("open failed with status: {}", status));
            }
            return Ok(resolved.display().to_string());
        }
        #[cfg(not(target_os = "macos"))]
        {
            let status = Command::new("xdg-open")
                .arg(&resolved)
                .status()
                .map_err(|e| format!("xdg-open failed: {}", e))?;
            if !status.success() {
                return Err(format!("xdg-open failed with status: {}", status));
            }
            Ok(resolved.display().to_string())
        }
    }

    fn draw_runtime_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("実行中上書き (debug_runtime.ron)");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.runtime_enabled, "上書きを有効化");
            if ui.button("実行中上書きを保存").clicked() {
                self.save_runtime();
            }
            if ui.button("選択中タブをリセット").clicked() {
                self.reset_runtime_current_section();
            }
            if ui.button("実行中上書きを全リセット").clicked() {
                self.reset_runtime_all();
            }
            if ui.button("実行中上書きファイルを開く").clicked() {
                match Self::open_file_default(DEBUG_RUNTIME_CONFIG_PATH) {
                    Ok(path) => {
                        self.status_message = format!("runtimeファイルを開きました: {}", path);
                    }
                    Err(err) => self.status_message = err,
                }
            }
            if ui.button("項目定義ファイルを開く").clicked() {
                match Self::open_file_default(DEBUG_FIELDS_CONFIG_PATH) {
                    Ok(path) => {
                        self.status_message = format!("項目定義ファイルを開きました: {}", path);
                    }
                    Err(err) => self.status_message = err,
                }
            }
        });
        ui.separator();

        self.ensure_runtime_section_index();

        let section_tabs: Vec<(usize, String, usize, usize)> = self
            .catalog
            .sections
            .iter()
            .enumerate()
            .map(|(index, section)| {
                (
                    index,
                    section.title.clone(),
                    section.fields.len(),
                    self.runtime_section_enabled_count(section),
                )
            })
            .collect();
        ui.horizontal_wrapped(|ui| {
            for (index, title, field_count, enabled_count) in &section_tabs {
                let selected = *index == self.runtime_section_index;
                let marker = if *enabled_count > 0 { "●" } else { "○" };
                let label = format!("{} {} {}/{}", marker, title, enabled_count, field_count);
                if ui.selectable_label(selected, label).clicked() {
                    self.runtime_section_index = *index;
                }
            }
        });
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("絞り込み:");
            ui.add(
                egui::TextEdit::singleline(&mut self.runtime_filter)
                    .desired_width(260.0)
                    .hint_text("ラベル or キー"),
            );
            if !self.runtime_filter.is_empty() && ui.button("クリア").clicked() {
                self.runtime_filter.clear();
            }
        });
        ui.separator();

        let selected_section = self
            .catalog
            .sections
            .get(self.runtime_section_index)
            .cloned();
        if let Some(section) = selected_section {
            let mut shown = 0usize;
            for field in &section.fields {
                if !field_matches_filter(field, &self.runtime_filter) {
                    continue;
                }
                shown += 1;
                self.draw_runtime_field(ui, field);
            }

            if shown == 0 {
                ui.label("該当する項目はありません。");
            }
        } else {
            ui.label("表示可能な項目セクションがありません。");
        }
    }

    fn draw_runtime_field(&mut self, ui: &mut egui::Ui, field: &DebugFieldDef) {
        if field.is_flag() {
            let state = self.flag_states.entry(field.key.clone()).or_insert(false);
            ui.checkbox(state, field.label());
            return;
        }

        if field.is_float() {
            let base_value = self.base_float_value(field);
            let state = self.float_states.entry(field.key.clone()).or_insert_with(|| {
                FloatState {
                    enabled: false,
                    value: base_value.unwrap_or(0.0),
                }
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut state.enabled, field.label());
                ui.add_enabled_ui(state.enabled, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut state.value)
                            .speed(field.step() as f64)
                            .range(field.min()..=field.max()),
                    );
                });
                if let Some(base_value) = base_value {
                    let hint = format!("({:.3})", base_value);
                    ui.label(egui::RichText::new(hint).weak());
                }
            });
            return;
        }

        ui.colored_label(
            egui::Color32::YELLOW,
            format!("未対応kind: {} ({})", field.kind, field.key),
        );
    }

    fn draw_env_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("起動時環境変数 (debug_env.ron)");
        ui.horizontal(|ui| {
            if ui.button("起動時環境変数を保存").clicked() {
                self.save_env();
            }
            if ui.button("このタブをリセット").clicked() {
                self.reset_env_tab();
            }
            if ui.button("起動時環境変数ファイルを開く").clicked() {
                match Self::open_file_default(DEBUG_ENV_CONFIG_PATH) {
                    Ok(path) => {
                        self.status_message = format!("envファイルを開きました: {}", path);
                    }
                    Err(err) => self.status_message = err,
                }
            }
        });
        ui.separator();

        let mut remove_index = None;
        for (index, row) in self.env_rows.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("#{}", index + 1));
                ui.text_edit_singleline(&mut row.key);
                ui.text_edit_singleline(&mut row.value);
                if ui.button("削除").clicked() {
                    remove_index = Some(index);
                }
            });
        }
        if let Some(index) = remove_index {
            self.env_rows.remove(index);
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("追加:");
            ui.text_edit_singleline(&mut self.new_env_key);
            ui.text_edit_singleline(&mut self.new_env_value);
            if ui.button("追加").clicked() {
                let key = self.new_env_key.trim();
                if !key.is_empty() {
                    self.env_rows.push(EnvRow {
                        key: key.to_string(),
                        value: self.new_env_value.clone(),
                    });
                    self.new_env_key.clear();
                    self.new_env_value.clear();
                }
            }
        });
    }

    fn draw_effective_preview(&self, ui: &mut egui::Ui) {
        ui.heading("実効値プレビュー");

        let Some(base) = &self.base_config else {
            ui.label("base game_config の読み込みに失敗しています。");
            return;
        };

        let mut runtime = match self.build_runtime_from_ui() {
            Ok(v) => v,
            Err(err) => {
                ui.label(format!("プレビュー生成失敗: {}", err));
                return;
            }
        };
        if runtime.has_any_value() && !runtime.enabled {
            runtime.enabled = true;
        }

        let effective = compose_effective_config(base, &self.startup_env, &runtime);

        for section in &self.catalog.sections {
            ui.label(format!("[{}]", section.title));
            let mut lines = Vec::new();
            for field in &section.fields {
                let line = match get_config_value(&effective, field.key()) {
                    Some(DebugOverrideValue::Bool(v)) => {
                        format!("{}: {}", field.label(), if v { "ON" } else { "OFF" })
                    }
                    Some(DebugOverrideValue::Float(v)) => format!("{}: {:.3}", field.label(), v),
                    None => format!("{}: (未対応キー)", field.label()),
                };
                lines.push(line);
            }
            ui.monospace(lines.join("\n"));
            ui.separator();
        }
    }

    fn ensure_runtime_section_index(&mut self) {
        if self.catalog.sections.is_empty() {
            self.runtime_section_index = 0;
            return;
        }
        let max = self.catalog.sections.len() - 1;
        if self.runtime_section_index > max {
            self.runtime_section_index = max;
        }
    }

    fn draw_main_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for tab in [MainTab::Runtime, MainTab::Env, MainTab::Effective] {
                let selected = self.active_tab == tab;
                let label = self.main_tab_label(tab);
                if ui.selectable_label(selected, label).clicked() {
                    self.active_tab = tab;
                }
            }
        });
    }

    fn main_tab_label(&self, tab: MainTab) -> String {
        match tab {
            MainTab::Runtime => {
                let enabled = self.runtime_enabled_total_count();
                if enabled > 0 {
                    format!("{} ●{}", tab.title(), enabled)
                } else {
                    tab.title().to_string()
                }
            }
            MainTab::Env => {
                let count = self.env_active_count();
                if count > 0 {
                    format!("{} ●{}", tab.title(), count)
                } else {
                    tab.title().to_string()
                }
            }
            MainTab::Effective => tab.title().to_string(),
        }
    }

    fn is_field_enabled(&self, field: &DebugFieldDef) -> bool {
        if field.is_flag() {
            return self.flag_states.get(field.key()).copied().unwrap_or(false);
        }
        if field.is_float() {
            return self
                .float_states
                .get(field.key())
                .map(|state| state.enabled)
                .unwrap_or(false);
        }
        false
    }

    fn runtime_section_enabled_count(&self, section: &DebugFieldSection) -> usize {
        section
            .fields
            .iter()
            .filter(|field| self.is_field_enabled(field))
            .count()
    }

    fn runtime_enabled_total_count(&self) -> usize {
        self.catalog
            .sections
            .iter()
            .map(|section| self.runtime_section_enabled_count(section))
            .sum()
    }

    fn base_float_value(&self, field: &DebugFieldDef) -> Option<f32> {
        self.base_config
            .as_ref()
            .and_then(|base| match get_config_value(base, field.key()) {
                Some(DebugOverrideValue::Float(v)) => Some(v),
                _ => None,
            })
    }

    fn env_active_count(&self) -> usize {
        self.env_rows
            .iter()
            .filter(|row| !row.key.trim().is_empty())
            .count()
    }

    fn reset_runtime_current_section(&mut self) {
        let section = self
            .catalog
            .sections
            .get(self.runtime_section_index)
            .cloned();
        let Some(section) = section else {
            self.status_message = "リセット対象のタブがありません".to_string();
            return;
        };
        for field in &section.fields {
            self.reset_runtime_field(field);
        }
        if self.runtime_enabled_total_count() == 0 {
            self.runtime_enabled = false;
        }
        self.status_message = format!("タブ「{}」をリセットしました", section.title);
    }

    fn reset_runtime_all(&mut self) {
        let sections = self.catalog.sections.clone();
        for section in &sections {
            for field in &section.fields {
                self.reset_runtime_field(field);
            }
        }
        self.runtime_enabled = false;
        self.status_message = "実行中上書きを全リセットしました".to_string();
    }

    fn reset_runtime_field(&mut self, field: &DebugFieldDef) {
        if field.is_flag() {
            self.flag_states.insert(field.key.clone(), false);
            return;
        }
        if field.is_float() {
            let base_value = self
                .base_config
                .as_ref()
                .and_then(|base| match get_config_value(base, field.key()) {
                    Some(DebugOverrideValue::Float(v)) => Some(v),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    self.float_states
                        .get(field.key())
                        .map(|state| state.value)
                        .unwrap_or(0.0)
                });
            self.float_states.insert(
                field.key.clone(),
                FloatState {
                    enabled: false,
                    value: base_value,
                },
            );
        }
    }

    fn reset_env_tab(&mut self) {
        self.env_rows.clear();
        self.new_env_key.clear();
        self.new_env_value.clear();
        self.status_message = "起動時環境変数タブをリセットしました".to_string();
    }
}

impl eframe::App for DebugControlGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.command) {
            self.save_all();
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("再読み込み").clicked() {
                    self.reload_files();
                }
                if ui.button("まとめて保存 (Cmd+S)").clicked() {
                    self.save_all();
                }
                ui.checkbox(&mut self.launch_release, "リリースビルドで起動");
                if ui.button("ゲーム起動").clicked() {
                    self.launch_game();
                }
            });
            ui.separator();
            ui.label(format!("状態: {}", self.status_message));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_main_tabs(ui);
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                MainTab::Runtime => self.draw_runtime_editor(ui),
                MainTab::Env => self.draw_env_editor(ui),
                MainTab::Effective => self.draw_effective_preview(ui),
            });
        });
    }
}

fn field_matches_filter(field: &DebugFieldDef, filter: &str) -> bool {
    let filter = filter.trim();
    if filter.is_empty() {
        return true;
    }

    let needle = filter.to_lowercase();
    field.label().to_lowercase().contains(&needle) || field.key().to_lowercase().contains(&needle)
}

fn load_or_create_field_catalog(path: &str) -> Result<DebugFieldCatalog, String> {
    let target = resolve_project_path(path);

    if target.exists() {
        let content = fs::read_to_string(&target)
            .map_err(|e| format!("Failed to read field catalog {}: {}", target.display(), e))?;
        let catalog = parse_field_catalog_with_legacy_support(&content, &target)?;
        validate_catalog(&catalog)?;
        return Ok(catalog);
    }

    let catalog = default_field_catalog();
    validate_catalog(&catalog)?;

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create {}: {}", parent.display(), e))?;
    }

    let pretty = ron::ser::PrettyConfig::new()
        .depth_limit(5)
        .separate_tuple_members(true);
    let payload = ron::ser::to_string_pretty(&catalog, pretty)
        .map_err(|e| format!("Failed to serialize field catalog: {}", e))?;
    fs::write(&target, payload)
        .map_err(|e| format!("Failed to write field catalog {}: {}", target.display(), e))?;

    Ok(catalog)
}

fn build_editor_states(
    catalog: &DebugFieldCatalog,
    base: &padel_game::resource::GameConfig,
    runtime: &DebugRuntimeOverrides,
) -> (BTreeMap<String, bool>, BTreeMap<String, FloatState>) {
    let mut flag_states = BTreeMap::new();
    let mut float_states = BTreeMap::new();

    for section in &catalog.sections {
        for field in &section.fields {
            if field.is_flag() {
                let checked = matches!(
                    get_override_value(runtime, field.key()),
                    Some(DebugOverrideValue::Bool(true))
                );
                flag_states.insert(field.key.clone(), checked);
                continue;
            }

            if field.is_float() {
                let runtime_value = match get_override_value(runtime, field.key()) {
                    Some(DebugOverrideValue::Float(v)) => Some(v),
                    _ => None,
                };
                let base_value = match get_config_value(base, field.key()) {
                    Some(DebugOverrideValue::Float(v)) => Some(v),
                    _ => None,
                };

                let state = if let Some(v) = runtime_value {
                    FloatState {
                        enabled: true,
                        value: v,
                    }
                } else {
                    FloatState {
                        enabled: false,
                        value: base_value.unwrap_or(0.0),
                    }
                };
                float_states.insert(field.key.clone(), state);
            }
        }
    }

    (flag_states, float_states)
}

fn default_field_catalog() -> DebugFieldCatalog {
    DebugFieldCatalog {
        sections: vec![
            DebugFieldSection {
                title: "デバッグ".to_string(),
                fields: vec![flag_field(
                    "serve.practice_infinite_mode",
                    "練習サーブ無限化",
                )],
            },
            DebugFieldSection {
                title: "物理".to_string(),
                fields: vec![
                    float_field("physics.gravity", "重力加速度", -50.0, 0.0, 0.1),
                    float_field("physics.max_fall_speed", "最大落下速度", -100.0, 0.0, 0.1),
                ],
            },
            DebugFieldSection {
                title: "プレイヤー".to_string(),
                fields: vec![
                    float_field("player.move_speed", "移動速度(X)", 0.0, 30.0, 0.1),
                    float_field("player.move_speed_z", "移動速度(Z)", 0.0, 30.0, 0.1),
                    float_field("player.max_speed", "最大速度", 0.0, 50.0, 0.1),
                    float_field("player.jump_force", "ジャンプ力", 0.0, 30.0, 0.1),
                ],
            },
            DebugFieldSection {
                title: "ボール".to_string(),
                fields: vec![
                    float_field("ball.normal_shot_speed", "通常ショット速度", 0.0, 40.0, 0.1),
                    float_field("ball.power_shot_speed", "強打ショット速度", 0.0, 50.0, 0.1),
                    float_field("ball.bounce_factor", "バウンド減衰", 0.0, 2.0, 0.01),
                    float_field("ball.radius", "ボール半径", 0.01, 2.0, 0.01),
                    float_field(
                        "ball.min_bounce_velocity",
                        "最小バウンド速度",
                        0.0,
                        20.0,
                        0.05,
                    ),
                    float_field("ball.wall_bounce_factor", "壁バウンド減衰", 0.0, 2.0, 0.01),
                ],
            },
            DebugFieldSection {
                title: "サーブ".to_string(),
                fields: vec![
                    float_field("serve.serve_speed", "サーブ速度", 0.0, 40.0, 0.1),
                    float_field("serve.serve_angle", "サーブ角度", -90.0, 90.0, 0.1),
                    float_field("serve.toss_velocity_y", "トス速度", 0.0, 20.0, 0.05),
                    float_field(
                        "serve.toss_velocity_min_y",
                        "トス速度(最小)",
                        0.0,
                        20.0,
                        0.05,
                    ),
                    float_field(
                        "serve.toss_velocity_max_y",
                        "トス速度(最大)",
                        0.0,
                        20.0,
                        0.05,
                    ),
                    float_field(
                        "serve.toss_hold_max_secs",
                        "トス長押し最大秒",
                        0.0,
                        2.0,
                        0.01,
                    ),
                    float_field("serve.toss_depth_shift", "トス奥行き補正", 0.0, 3.0, 0.01),
                    float_field(
                        "serve.toss_launch_angle_bonus_deg",
                        "トス角度ボーナス",
                        0.0,
                        30.0,
                        0.1,
                    ),
                    float_field("serve.toss_timeout", "トス失敗タイムアウト", 0.0, 20.0, 0.1),
                    float_field("serve.hit_height_min", "ヒット高さ(最小)", 0.0, 10.0, 0.01),
                    float_field("serve.hit_height_max", "ヒット高さ(最大)", 0.0, 10.0, 0.01),
                    float_field(
                        "serve.hit_height_optimal",
                        "ヒット高さ(最適)",
                        0.0,
                        10.0,
                        0.01,
                    ),
                    float_field("serve.ai_hit_tolerance", "AIヒット許容", 0.0, 2.0, 0.01),
                ],
            },
            DebugFieldSection {
                title: "ショット".to_string(),
                fields: vec![
                    float_field("shot.max_distance", "最大ショット距離", 0.0, 10.0, 0.01),
                    float_field("shot.cooldown_time", "ショットクールダウン", 0.0, 5.0, 0.01),
                    float_field("shot.jump_threshold", "ジャンプ閾値", 0.0, 5.0, 0.01),
                    float_field(
                        "shot.racket_swing.duration_seconds",
                        "ラケットスイング時間",
                        0.05,
                        1.5,
                        0.01,
                    ),
                    float_field(
                        "shot.racket_swing.contact_time_seconds",
                        "接触目標時刻",
                        0.0,
                        1.0,
                        0.01,
                    ),
                    float_field(
                        "shot.racket_swing.contact_window_seconds",
                        "接触判定ウィンドウ",
                        0.01,
                        0.5,
                        0.005,
                    ),
                    float_field(
                        "shot.racket_swing.min_prediction_time",
                        "最小予測時間",
                        0.0,
                        0.5,
                        0.005,
                    ),
                    float_field(
                        "shot.racket_swing.max_prediction_time",
                        "最大予測時間",
                        0.05,
                        1.2,
                        0.01,
                    ),
                    float_field(
                        "shot.racket_swing.prediction_step",
                        "予測刻み幅",
                        0.001,
                        0.05,
                        0.001,
                    ),
                    float_field(
                        "shot.racket_swing.reach_distance",
                        "ラケット到達距離",
                        0.5,
                        3.0,
                        0.01,
                    ),
                    float_field(
                        "shot.racket_swing.max_hit_height_diff",
                        "最大打点高低差",
                        0.1,
                        5.0,
                        0.01,
                    ),
                    float_field(
                        "shot.racket_swing.contact_radius",
                        "接触半径",
                        0.05,
                        1.0,
                        0.01,
                    ),
                ],
            },
            DebugFieldSection {
                title: "AI".to_string(),
                fields: vec![
                    float_field("ai.move_speed", "AI移動速度", 0.0, 30.0, 0.1),
                    float_field("ai.shot_cooldown", "AIショットクールダウン", 0.0, 5.0, 0.01),
                    float_field("ai.prediction_accuracy", "AI予測精度", 0.0, 1.0, 0.01),
                    float_field("ai.prediction_error", "AI予測誤差", 0.0, 5.0, 0.01),
                    float_field("ai.direction_variance", "AI方向ブレ", 0.0, 45.0, 0.1),
                    float_field("ai.reaction_delay", "AI反応遅延", 0.0, 2.0, 0.01),
                    float_field("ai.offensive_probability", "AI攻め確率", 0.0, 1.0, 0.01),
                    float_field(
                        "ai.serve_offensive_probability",
                        "AIサーブ攻め確率",
                        0.0,
                        1.0,
                        0.01,
                    ),
                ],
            },
        ],
    }
}

fn flag_field(key: &str, label: &str) -> DebugFieldDef {
    DebugFieldDef {
        kind: "flag".to_string(),
        key: key.to_string(),
        label: label.to_string(),
        min: None,
        max: None,
        step: None,
    }
}

fn float_field(key: &str, label: &str, min: f32, max: f32, step: f32) -> DebugFieldDef {
    DebugFieldDef {
        kind: "float".to_string(),
        key: key.to_string(),
        label: label.to_string(),
        min: Some(min),
        max: Some(max),
        step: Some(step),
    }
}

fn validate_catalog(catalog: &DebugFieldCatalog) -> Result<(), String> {
    for section in &catalog.sections {
        for field in &section.fields {
            let key = field.key();
            let Some(expected_kind) = override_field_type(key) else {
                return Err(format!("Unknown override key in catalog: {}", key));
            };

            let actual_kind = if field.is_flag() {
                Some(DebugOverrideType::Bool)
            } else if field.is_float() {
                Some(DebugOverrideType::Float)
            } else {
                None
            };

            if actual_kind != Some(expected_kind) {
                return Err(format!(
                    "Type mismatch in catalog for key '{}': expected={:?}, actual_kind={}",
                    key, expected_kind, field.kind
                ));
            }
        }
    }

    Ok(())
}

fn resolve_path(path: &str) -> PathBuf {
    let p = Path::new(path);
    if p.is_absolute() {
        return p.to_path_buf();
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let candidate = cwd.join(p);
    if candidate.exists() {
        return candidate;
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join(p)
}

fn resolve_project_path(path: &str) -> PathBuf {
    resolve_path(path)
}

fn parse_field_catalog_with_legacy_support(
    content: &str,
    path: &Path,
) -> Result<DebugFieldCatalog, String> {
    if let Ok(catalog) = ron::from_str::<DebugFieldCatalog>(content) {
        return Ok(catalog);
    }

    let migrated = migrate_legacy_kind_literals(content);

    if let Ok(catalog) = ron::from_str::<DebugFieldCatalog>(&migrated) {
        return Ok(catalog);
    }

    if let Ok(catalog) = parse_field_catalog_implicit_some(content) {
        return Ok(catalog);
    }

    if let Ok(catalog) = parse_field_catalog_implicit_some(&migrated) {
        return Ok(catalog);
    }

    let primary_err = ron::from_str::<DebugFieldCatalog>(content).unwrap_err();
    let fallback_err = if migrated == content {
        "no legacy migration applied".to_string()
    } else {
        ron::from_str::<DebugFieldCatalog>(&migrated)
            .unwrap_err()
            .to_string()
    };
    Err(format!(
        "Failed to parse field catalog {}: {} (fallback: {})",
        path.display(),
        primary_err,
        fallback_err
    ))
}

fn parse_field_catalog_implicit_some(
    content: &str,
) -> Result<DebugFieldCatalog, ron::error::SpannedError> {
    RonOptions::default()
        .with_default_extension(Extensions::IMPLICIT_SOME)
        .from_str(content)
}

fn migrate_legacy_kind_literals(content: &str) -> String {
    let mut changed = false;
    let mut migrated = Vec::new();

    for line in content.lines() {
        if let Some(kind_pos) = line.find("kind:") {
            let prefix = &line[..kind_pos + "kind:".len()];
            let rest = &line[kind_pos + "kind:".len()..];
            let ws_len = rest.len() - rest.trim_start().len();
            let ws = &rest[..ws_len];
            let value = &rest[ws_len..];
            if value.starts_with('"') {
                migrated.push(line.to_string());
                continue;
            }
            if let Some(tail) = value.strip_prefix("flag") {
                migrated.push(format!("{}{}\"flag\"{}", prefix, ws, tail));
                changed = true;
                continue;
            }
            if let Some(tail) = value.strip_prefix("float") {
                migrated.push(format!("{}{}\"float\"{}", prefix, ws, tail));
                changed = true;
                continue;
            }
        }
        migrated.push(line.to_string());
    }

    if changed {
        migrated.join("\n")
    } else {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn フィールド種別は文字列形式を受け入れる() {
        let ron = r#"
(
    sections: [
        (
            title: "t",
            fields: [
                (kind: "flag", key: "serve.practice_infinite_mode", label: "x"),
            ],
        ),
    ],
)
"#;
        let catalog: DebugFieldCatalog = ron::from_str(ron).expect("string kind should parse");
        assert!(catalog.sections[0].fields[0].is_flag());
    }

    #[test]
    fn 旧kind記法は読み込み時に移行して扱える() {
        let ron = r#"
(
    sections: [
        (
            title: "t",
            fields: [
                (kind: flag, key: "serve.practice_infinite_mode", label: "x"),
            ],
        ),
    ],
)
"#;
        let migrated = migrate_legacy_kind_literals(ron);
        let catalog: DebugFieldCatalog =
            ron::from_str(&migrated).expect("migrated legacy kind should parse");
        assert!(catalog.sections[0].fields[0].is_flag());
    }

    #[test]
    fn implicit_some形式のmin_max_stepを読み込める() {
        let ron = r#"
(
    sections: [
        (
            title: "t",
            fields: [
                (
                    kind: "float",
                    key: "physics.gravity",
                    label: "x",
                    min: -50.0,
                    max: 0.0,
                    step: 0.1,
                ),
            ],
        ),
    ],
)
"#;
        let catalog =
            parse_field_catalog_with_legacy_support(ron, Path::new("dummy.ron"))
                .expect("implicit_some should parse");
        let field = &catalog.sections[0].fields[0];
        assert_eq!(field.min, Some(-50.0));
        assert_eq!(field.max, Some(0.0));
        assert_eq!(field.step, Some(0.1));
    }
}
