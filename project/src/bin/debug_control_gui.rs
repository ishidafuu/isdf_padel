//! Debug Control GUI
//! @spec 77210_debug_control.md

use eframe::egui;
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

use padel_game::resource::{
    compose_effective_config, load_env_profile, load_game_config, load_runtime_overrides,
    save_env_profile, save_runtime_overrides, DebugEnvProfile, DebugRuntimeOverrides,
    DEBUG_ENV_CONFIG_PATH, DEBUG_RUNTIME_CONFIG_PATH,
};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Padel デバッグ制御 GUI")
            .with_inner_size([960.0, 760.0]),
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
        // フォントが見つからない場合はデフォルトのまま動かす
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
    // macOSで一般的な日本語フォント候補
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

#[derive(Clone, Copy)]
struct OptionalBoolField {
    use_override: bool,
    value: bool,
}

impl OptionalBoolField {
    fn from_option(value: Option<bool>) -> Self {
        Self {
            use_override: value.is_some(),
            value: value.unwrap_or(false),
        }
    }

    fn to_option(self) -> Option<bool> {
        if self.use_override {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct OptionalF32Field {
    use_override: bool,
    value: f32,
}

impl OptionalF32Field {
    fn from_option(value: Option<f32>) -> Self {
        Self {
            use_override: value.is_some(),
            value: value.unwrap_or(0.0),
        }
    }

    fn to_option(self) -> Option<f32> {
        if self.use_override {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct EnvRow {
    key: String,
    value: String,
}

struct DebugControlGuiApp {
    base_config: Option<padel_game::resource::GameConfig>,
    startup_env: DebugRuntimeOverrides,

    runtime_enabled: bool,
    practice_infinite_mode: OptionalBoolField,
    player_move_speed: OptionalF32Field,
    player_move_speed_z: OptionalF32Field,
    ball_normal_shot_speed: OptionalF32Field,
    ball_power_shot_speed: OptionalF32Field,
    serve_speed: OptionalF32Field,
    gravity: OptionalF32Field,

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
            runtime_enabled: false,
            practice_infinite_mode: OptionalBoolField::from_option(None),
            player_move_speed: OptionalF32Field::from_option(None),
            player_move_speed_z: OptionalF32Field::from_option(None),
            ball_normal_shot_speed: OptionalF32Field::from_option(None),
            ball_power_shot_speed: OptionalF32Field::from_option(None),
            serve_speed: OptionalF32Field::from_option(None),
            gravity: OptionalF32Field::from_option(None),
            env_rows: Vec::new(),
            new_env_key: String::new(),
            new_env_value: String::new(),
            launch_release: false,
            status_message: error,
        }
    }

    fn load() -> Result<Self, String> {
        let base = load_game_config("assets/config/game_config.ron")?;
        let startup_env = DebugRuntimeOverrides::from_env();
        let runtime = load_runtime_overrides(DEBUG_RUNTIME_CONFIG_PATH)?;
        let env_profile = load_env_profile(DEBUG_ENV_CONFIG_PATH)?;

        let env_rows = env_profile
            .vars
            .into_iter()
            .map(|(key, value)| EnvRow { key, value })
            .collect();

        Ok(Self {
            base_config: Some(base),
            startup_env,
            runtime_enabled: runtime.enabled,
            practice_infinite_mode: OptionalBoolField::from_option(runtime.practice_infinite_mode),
            player_move_speed: OptionalF32Field::from_option(runtime.player_move_speed),
            player_move_speed_z: OptionalF32Field::from_option(runtime.player_move_speed_z),
            ball_normal_shot_speed: OptionalF32Field::from_option(runtime.ball_normal_shot_speed),
            ball_power_shot_speed: OptionalF32Field::from_option(runtime.ball_power_shot_speed),
            serve_speed: OptionalF32Field::from_option(runtime.serve_speed),
            gravity: OptionalF32Field::from_option(runtime.gravity),
            env_rows,
            new_env_key: String::new(),
            new_env_value: String::new(),
            launch_release: false,
            status_message: "読み込み完了".to_string(),
        })
    }

    fn runtime_overrides(&self) -> DebugRuntimeOverrides {
        DebugRuntimeOverrides {
            enabled: self.runtime_enabled,
            practice_infinite_mode: self.practice_infinite_mode.to_option(),
            player_move_speed: self.player_move_speed.to_option(),
            player_move_speed_z: self.player_move_speed_z.to_option(),
            ball_normal_shot_speed: self.ball_normal_shot_speed.to_option(),
            ball_power_shot_speed: self.ball_power_shot_speed.to_option(),
            serve_speed: self.serve_speed.to_option(),
            gravity: self.gravity.to_option(),
        }
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

    fn reload_files(&mut self) {
        match Self::load() {
            Ok(new_state) => *self = new_state,
            Err(err) => self.status_message = format!("再読み込み失敗: {}", err),
        }
    }

    fn save_runtime(&mut self) {
        let mut overrides = self.runtime_overrides();
        if overrides.has_any_value() && !overrides.enabled {
            // 値があるのに無効だと「保存したのに反映されない」状態になるため自動有効化
            overrides.enabled = true;
            self.runtime_enabled = true;
        }

        match save_runtime_overrides(DEBUG_RUNTIME_CONFIG_PATH, &overrides) {
            Ok(()) => self.status_message = "実行中上書きの保存完了".to_string(),
            Err(err) => self.status_message = format!("実行中上書きの保存失敗: {}", err),
        }
    }

    fn save_env(&mut self) {
        match save_env_profile(DEBUG_ENV_CONFIG_PATH, &self.env_profile()) {
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

    fn open_file_default(path: &str) -> Result<(), String> {
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(path)
                .spawn()
                .map_err(|e| format!("open failed: {}", e))?;
            return Ok(());
        }
        #[cfg(not(target_os = "macos"))]
        {
            Command::new("xdg-open")
                .arg(path)
                .spawn()
                .map_err(|e| format!("xdg-open failed: {}", e))?;
            Ok(())
        }
    }

    fn draw_runtime_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("実行中上書き (debug_runtime.ron)");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.runtime_enabled, "上書きを有効化");
            if ui.button("実行中上書きを保存").clicked() {
                self.save_runtime();
            }
            if ui.button("実行中上書きファイルを開く").clicked() {
                match Self::open_file_default(DEBUG_RUNTIME_CONFIG_PATH) {
                    Ok(()) => self.status_message = "runtimeファイルを開きました".to_string(),
                    Err(err) => self.status_message = err,
                }
            }
        });
        ui.separator();

        draw_optional_bool_row(
            ui,
            "練習サーブ無限化",
            &mut self.practice_infinite_mode,
            "有効",
        );
        draw_optional_f32_row(ui, "プレイヤー移動速度(X)", &mut self.player_move_speed);
        draw_optional_f32_row(ui, "プレイヤー移動速度(Z)", &mut self.player_move_speed_z);
        draw_optional_f32_row(
            ui,
            "ボール通常ショット速度",
            &mut self.ball_normal_shot_speed,
        );
        draw_optional_f32_row(
            ui,
            "ボール強打ショット速度",
            &mut self.ball_power_shot_speed,
        );
        draw_optional_f32_row(ui, "サーブ速度", &mut self.serve_speed);
        draw_optional_f32_row(ui, "重力加速度", &mut self.gravity);
    }

    fn draw_env_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("起動時環境変数 (debug_env.ron)");
        ui.horizontal(|ui| {
            if ui.button("起動時環境変数を保存").clicked() {
                self.save_env();
            }
            if ui.button("起動時環境変数ファイルを開く").clicked() {
                match Self::open_file_default(DEBUG_ENV_CONFIG_PATH) {
                    Ok(()) => self.status_message = "envファイルを開きました".to_string(),
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

        let effective =
            compose_effective_config(base, &self.startup_env, &self.runtime_overrides());
        ui.monospace(format!(
            "練習サーブ無限化: {}\nプレイヤー移動速度(X): {:.3}\nプレイヤー移動速度(Z): {:.3}\nボール通常ショット速度: {:.3}\nボール強打ショット速度: {:.3}\nサーブ速度: {:.3}\n重力加速度: {:.3}",
            effective.serve.practice_infinite_mode,
            effective.player.move_speed,
            effective.player.move_speed_z,
            effective.ball.normal_shot_speed,
            effective.ball.power_shot_speed,
            effective.serve.serve_speed,
            effective.physics.gravity
        ));
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.draw_runtime_editor(ui);
                ui.separator();
                self.draw_env_editor(ui);
                ui.separator();
                self.draw_effective_preview(ui);
            });
        });
    }
}

fn draw_optional_bool_row(
    ui: &mut egui::Ui,
    label: &str,
    field: &mut OptionalBoolField,
    caption: &str,
) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut field.use_override, label);
        ui.add_enabled_ui(field.use_override, |ui| {
            ui.checkbox(&mut field.value, caption);
        });
    });
}

fn draw_optional_f32_row(ui: &mut egui::Ui, label: &str, field: &mut OptionalF32Field) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut field.use_override, label);
        ui.add_enabled_ui(field.use_override, |ui| {
            ui.add(
                egui::DragValue::new(&mut field.value)
                    .speed(0.05)
                    .range(-1000.0..=1000.0),
            );
        });
    });
}
