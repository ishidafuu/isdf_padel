# Shot System Test Cases

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ショットシステム（入力、方向計算、ジャンプショット）のテストケースを定義します。

## テストケース

### ショット入力テスト（30601_shot_input_spec.md）

#### TST-30604-001: ショット入力
**対応**: REQ-30601-001
**Given**: プレイヤーがボールの近く（距離 1.0m）にいる、クールダウン完了
**When**: Bボタンを押す
**Then**: ShotEvent が発行される

---

#### TST-30604-002: タイミング判定（距離）
**対応**: REQ-30601-002
**Given**: プレイヤーとボールの距離が 2.0m（MaxDistance 1.5m超過）
**When**: Bボタンを押す
**Then**: ショット入力が無視される（ShotEvent 発行されず）

---

#### TST-30604-003: タイミング判定（高さ差）
**対応**: REQ-30601-003
**Given**: プレイヤーとボールの高さ差が 2.5m（MaxHeightDiff 2.0m超過）
**When**: Bボタンを押す
**Then**: ショット入力が無視される（ShotEvent 発行されず）

---

#### TST-30604-004: クールダウン管理
**対応**: REQ-30601-004
**Given**: ショットを実行した直後、CooldownTimer = 0.5秒
**When**: 0.3秒経過後、Bボタンを押す
**Then**: ショット入力が無視される（クールダウン中）
**When**: 0.5秒経過後、Bボタンを押す
**Then**: ShotEvent が発行される（クールダウン完了）

---

#### TST-30604-005: ふっとばし中のショット禁止
**対応**: REQ-30601-005
**Given**: プレイヤーがふっとばし状態（Knockback.IsActive == true）
**When**: Bボタンを押す
**Then**: ショット入力が無視される

---

#### TST-30604-006: ShotEvent の発行
**対応**: REQ-30601-006
**Given**: ショット条件を満たす、EventBus がリスニング中
**When**: Bボタンを押す
**Then**:
- ShotEvent が発行される
- イベントデータに PlayerId, Direction, JumpHeight が含まれる

---

### ショット方向テスト（30602_shot_direction_spec.md）

#### TST-30604-007: 水平方向の計算
**対応**: REQ-30602-001
**Given**: 入力方向が右前（Direction = (1, 1)）
**When**: ShotEvent を受信
**Then**: 水平方向ベクトルが正規化される（(0.707, 0.707)）

---

#### TST-30604-008: 通常ショットの速度
**対応**: REQ-30602-002
**Given**: プレイヤーが地上にいる（Position.Y = 0）
**When**: ショットを実行
**Then**:
- 速度: 10.0 m/s
- 打球角度: 45度

---

#### TST-30604-009: ジャンプショットの速度
**対応**: REQ-30602-003
**Given**: プレイヤーがジャンプ中（Position.Y = 1.5m）
**When**: ショットを実行
**Then**:
- 速度: 15.0 m/s
- 打球角度: 30度

---

#### TST-30604-010: 打球ベクトルの計算
**対応**: REQ-30602-004
**Given**: 水平方向 = (1, 0)、速度 = 10.0 m/s、角度 = 45度
**When**: 打球ベクトルを計算
**Then**:
- Velocity.X = 10.0 * cos(45°) = 7.07 m/s
- Velocity.Y = 10.0 * sin(45°) = 7.07 m/s
- Velocity.Z = 0

---

#### TST-30604-011: ボール速度の設定
**対応**: REQ-30602-005
**Given**: 打球ベクトルが計算された
**When**: ショットを実行
**Then**:
- Ball.Velocity が設定される
- Ball.State = Flying

---

#### TST-30604-012: クールダウンの開始
**対応**: REQ-30602-006
**Given**: ショットを実行した
**When**: ショット処理完了
**Then**: ShotState.CooldownTimer = 0.5秒

---

#### TST-30604-013: ShotExecutedEvent の発行
**対応**: REQ-30602-007
**Given**: ショットが実行された、EventBus がリスニング中
**When**: ショット処理完了
**Then**:
- ShotExecutedEvent が発行される
- イベントデータに PlayerId, ShotVelocity, IsJumpShot が含まれる

---

### ジャンプショットテスト（30603_jump_shot_spec.md）

#### TST-30604-014: ジャンプ判定
**対応**: REQ-30603-001
**Given**: プレイヤー Position.Y = 1.0m（JumpThreshold 0.5m超過）
**When**: ショットを実行
**Then**: ジャンプショットと判定される

---

#### TST-30604-015: ジャンプショットの速度増加
**対応**: REQ-30603-002
**Given**: プレイヤーがジャンプ中（Position.Y = 1.5m）
**When**: ショットを実行
**Then**: ボール速度 = 15.0 m/s（通常 10.0 m/sより高速）

---

#### TST-30604-016: ジャンプショットの角度変化
**対応**: REQ-30603-003
**Given**: プレイヤーがジャンプ中（Position.Y = 1.5m）
**When**: ショットを実行
**Then**: 打球角度 = 30度（通常 45度より急）

---

#### TST-30604-017: ジャンプショット中の空中制御
**対応**: REQ-30603-004
**Given**: プレイヤーがジャンプショット中
**When**: 左右移動入力を行う
**Then**: プレイヤーが空中で移動できる

---

#### TST-30604-018: ジャンプショットの視覚フィードバック
**対応**: REQ-30603-005
**Given**: ジャンプショットを実行
**When**: ショット処理完了
**Then**: 特別なエフェクトが表示される（UI担当）

---

#### TST-30604-019: JumpShotEvent の発行
**対応**: REQ-30603-006
**Given**: ジャンプショットが実行された、EventBus がリスニング中
**When**: ショット処理完了
**Then**:
- JumpShotEvent が発行される
- イベントデータに PlayerId, JumpHeight, ShotVelocity が含まれる

---

## エッジケース

### EC-30604-001: 境界ギリギリのタイミング
**Given**: プレイヤーとボールの距離が 1.5m（MaxDistance境界）
**When**: Bボタンを押す
**Then**: ShotEvent が発行される（境界値を含む）

---

### EC-30604-002: クールダウン終了直後
**Given**: CooldownTimer = 0.001秒
**When**: 0.001秒経過後、Bボタンを押す
**Then**: ShotEvent が発行される

---

### EC-30604-003: ジャンプショット閾値ギリギリ
**Given**: プレイヤー Position.Y = 0.5m（JumpThreshold境界）
**When**: ショットを実行
**Then**: 通常ショットと判定される（閾値を超えていない）

---

### EC-30604-004: 方向入力なし
**Given**: 方向入力がない（Direction = (0, 0)）
**When**: ショットを実行
**Then**: 相手コート方向（Z軸正方向）に打つ

---

## データ参照

全テストケースは以下のデータを参照します：
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Shot パラメータ

---

## 依存関係

### 依存先
- [30601_shot_input_spec.md](30601_shot_input_spec.md) - ショット入力仕様
- [30602_shot_direction_spec.md](30602_shot_direction_spec.md) - ショット方向仕様
- [30603_jump_shot_spec.md](30603_jump_shot_spec.md) - ジャンプショット仕様
