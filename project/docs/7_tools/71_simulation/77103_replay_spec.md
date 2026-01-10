# 77103: Replay System Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-10

## 概要

手元で発生した不具合をAIに伝えて再現・調査させるためのデバッグ用リプレイ機能。入力を記録し、同一シード・同一入力で再現可能にする。

## 目的

- 不具合発生時の入力シーケンスを正確に記録
- AIによる自律的な不具合再現・調査を支援
- ヘッドレス再生と画面表示再生の両方に対応

## データ構造

### ReplayData

```rust
pub struct ReplayData {
    pub metadata: ReplayMetadata,
    pub frames: Vec<FrameInput>,
}
```

### ReplayMetadata

```rust
pub struct ReplayMetadata {
    pub game_version: String,       // バージョン不一致時は削除対象
    pub recorded_at: String,        // ISO 8601形式
    pub seed: u64,                  // 乱数シード
    pub initial_serve_side: TeamSide,
}
```

### FrameInput

```rust
pub struct FrameInput {
    pub frame: u32,
    pub p1: InputSnapshot,
    pub p2: InputSnapshot,
}
```

### InputSnapshot

```rust
pub struct InputSnapshot {
    pub movement: Vec2,
    pub shot_pressed: bool,
    pub jump_pressed: bool,
    pub holding: bool,
    pub hold_time: f32,
}
```

## Core Requirements (MVP)

### 記録機能

#### REQ-77103-001: フレーム入力記録
**WHILE** 試合中（Rally状態または Serve状態）
**THE SYSTEM SHALL** 毎フレームの入力状態を記録する
- P1、P2両方の InputState をキャプチャ
- フレーム番号と共に FrameInput として保存
**テスト**: 試合プレイ後にリプレイファイルが生成されることを確認

#### REQ-77103-002: メタデータ記録
**WHEN** 試合が開始される
**THE SYSTEM SHALL** メタデータを記録する
- game_version: 現在のゲームバージョン
- recorded_at: 記録開始時刻（ISO 8601）
- seed: 乱数シード値
- initial_serve_side: 最初のサーブ側
**テスト**: メタデータが正しく記録されることを確認

#### REQ-77103-003: 自動保存
**WHEN** 試合が終了する（MatchEnd状態に遷移）
**THE SYSTEM SHALL** リプレイデータを自動保存する
- 保存先: `assets/replays/`
- ファイル名: `replay_{timestamp}.ron`
- フォーマット: RON（Rusty Object Notation）
**テスト**: 試合終了後にRONファイルが生成されることを確認

### ファイル管理

#### REQ-77103-004: バージョン不一致削除
**WHEN** ゲームが起動する
**THE SYSTEM SHALL** バージョン不一致のリプレイを削除する
- metadata.game_version が現在のバージョンと不一致 → 削除
**テスト**: バージョン変更後に古いリプレイが削除されることを確認

#### REQ-77103-005: 保存数上限管理
**WHEN** リプレイ保存数が上限を超える
**THE SYSTEM SHALL** 古いリプレイを削除する
- 上限: 100件
- 削除順: recorded_at の古い順
**WITH** 上限値は外部設定ファイルで指定
**テスト**: 101件目の保存時に最古のファイルが削除されることを確認

### 再生機能（ヘッドレス）

#### REQ-77103-006: リプレイ読み込み
**WHEN** リプレイファイルを指定して再生を開始する
**THE SYSTEM SHALL** リプレイデータを読み込む
- RONファイルをデシリアライズ
- バージョン不一致の場合はエラー終了
**テスト**: 正常なリプレイファイルが読み込めることを確認

#### REQ-77103-007: シード復元
**WHEN** リプレイ再生が開始される
**THE SYSTEM SHALL** 記録されたシードで乱数を初期化する
- metadata.seed を使用
- 確定的な再現を保証
**テスト**: 同一リプレイを複数回再生して同一結果を確認

#### REQ-77103-008: 入力注入
**WHILE** リプレイ再生中
**THE SYSTEM SHALL** 記録された入力を各フレームで注入する
- 通常の入力システムを無効化
- FrameInput の内容を InputState に上書き
**テスト**: 記録時と同じ動作が再現されることを確認

#### REQ-77103-009: ヘッドレス再生CLI
**THE SYSTEM SHALL** コマンドラインからリプレイを再生できる
```bash
cargo run --bin replay_player -- <replay_file>
```
- HeadlessPlugins基盤を使用
- トレース出力対応（headless_sim と同様）
**テスト**: CLIでリプレイが再生できることを確認

## Extended Requirements (v0.2+)

### 画面表示再生

#### REQ-77103-050: リプレイモード状態
**WHEN** リプレイモードを選択する
**THE SYSTEM SHALL** AppState::ReplayPlayback に遷移する
- 入力ソースをリプレイに切り替え
- UI表示を「Replay」モードに変更
**テスト**: リプレイモードで画面表示されることを確認

#### REQ-77103-051: リプレイ選択UI
**WHEN** リプレイ選択画面を開く
**THE SYSTEM SHALL** 保存済みリプレイ一覧を表示する
- 記録日時
- 対戦結果（勝者）
- 総フレーム数
**テスト**: UIからリプレイを選択して再生できることを確認

### 再生コントロール（Future）

#### REQ-77103-100: 一時停止/再開
**WHEN** 一時停止ボタンを押す
**THE SYSTEM SHALL** 再生を一時停止する
**WHEN** 再開ボタンを押す
**THE SYSTEM SHALL** 再生を再開する
**テスト**: 一時停止と再開が正しく動作することを確認

#### REQ-77103-101: 再生速度変更
**WHEN** 再生速度を変更する
**THE SYSTEM SHALL** 指定された速度で再生する
- WITH 速度オプション: 0.5x, 1x, 2x, 4x
**テスト**: 各速度で正しく再生されることを確認

#### REQ-77103-102: フレーム送り
**WHEN** フレーム送りボタンを押す
**THE SYSTEM SHALL** 1フレーム進める
**テスト**: フレーム単位で進められることを確認

## アーキテクチャ

### ファイル構成

```
project/
├── assets/
│   ├── config/
│   │   └── replay_config.ron   # リプレイ設定
│   └── replays/                # リプレイ保存先
│       └── replay_{timestamp}.ron
└── src/
    ├── lib.rs                     # replay モジュール公開
    ├── bin/
    │   └── replay_player.rs       # CLIエントリポイント
    └── replay/
        ├── mod.rs                 # モジュール定義
        ├── data.rs                # データ構造
        ├── recorder.rs            # 記録システム
        ├── loader.rs              # 読み込み
        ├── player.rs              # 再生システム
        └── manager.rs             # ファイル管理
```

### シミュレータとの統合

- 共通基盤: `HeadlessPlugins` を再利用
- トレース出力: 77100_headless_sim.md のトレース機能と共有
- 入力注入: `ReplayInputProvider` リソースで InputState を上書き

## 使用方法

### 記録

通常のゲームプレイで自動的に記録される。特別な操作は不要。

### ヘッドレス再生

```bash
# project/ ディレクトリで実行
cargo run --bin replay_player -- assets/replays/replay_20260110_123456.ron

# 詳細出力付き
cargo run --bin replay_player -- -v assets/replays/replay_20260110_123456.ron
```

### 画面表示再生（v0.2+）

ゲーム内メニューからリプレイ選択。

## 依存関係

- [77100_headless_sim.md](77100_headless_sim.md) - HeadlessPlugins、トレース出力
- [20006_input_system.md](../../2_architecture/20006_input_system.md) - InputState コンポーネント

## 制限事項

- 人間 vs AI 対戦のみ対応（MVP）
- 人間 vs 人間は v0.2+ で対応予定
- ネットワーク対戦のリプレイは非対応
