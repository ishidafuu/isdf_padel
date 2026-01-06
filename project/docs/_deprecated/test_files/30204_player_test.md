# Player Test Cases

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

プレイヤー移動、ジャンプ、ふっとばしに関するテストケースを定義します。

## テストケース

### 移動テスト（30201_movement_spec.md）

#### TST-30204-001: 左右移動（X軸）
**対応**: REQ-30201-001
**Given**: プレイヤーがコート中央（X=0）にいる
**When**: 右入力（D キー）を 1秒間行う
**Then**:
- プレイヤーのX座標が +5.0m 増加する
- PlayerMoveEvent が発行される
- X座標が境界を超えない

---

#### TST-30204-002: 前後移動（Z軸）
**対応**: REQ-30201-002
**Given**: プレイヤーがコート中央（Z=-1.5）にいる
**When**: 前入力（W キー）を 1秒間行う
**Then**:
- プレイヤーのZ座標が -4.0m 移動する（前方向）
- PlayerMoveEvent が発行される
- Z座標が自コート範囲内に留まる

---

#### TST-30204-003: 斜め移動の速度制限
**対応**: REQ-30201-003
**Given**: プレイヤーがコート中央にいる
**When**: 右入力（D キー）と前入力（W キー）を同時に 1秒間行う
**Then**:
- プレイヤーが右前方向に移動する
- 移動速度が max(5.0, 4.0) = 5.0 m/s 以下である

---

#### TST-30204-004: 境界での停止
**対応**: REQ-30201-004
**Given**: プレイヤーが右境界（X=+5.0）にいる
**When**: 右入力（D キー）を 1秒間行う
**Then**:
- プレイヤーのX座標が +5.0 に留まる
- 前後移動は通常通り可能である

---

#### TST-30204-005: ふっとばし中の移動制限
**対応**: REQ-30201-005
**Given**: プレイヤーがふっとばし状態（Knockback.IsActive == true）
**When**: 左右・前後入力を行う
**Then**:
- プレイヤーの入力が無視される
- ふっとばしベクトルに従った移動のみが行われる

---

#### TST-30204-006: PlayerMoveEvent の発行
**対応**: REQ-30201-006
**Given**: プレイヤーがコート中央にいる、EventBus がリスニング中
**When**: 任意の方向に移動する
**Then**:
- PlayerMoveEvent が発行される
- イベントデータに `PlayerId`, `Position`, `Velocity` が含まれる

---

### ジャンプテスト（30202_jump_spec.md）

#### TST-30204-007: ジャンプ開始
**対応**: REQ-30202-001
**Given**: プレイヤーが地面（Y=0）にいる、IsGrounded == true
**When**: ジャンプボタン（Space キー）を押す
**Then**:
- プレイヤーのY軸速度が +8.0 m/s になる
- IsGrounded が false になる
- PlayerJumpEvent が発行される

---

#### TST-30204-008: 重力適用
**対応**: REQ-30202-002
**Given**: プレイヤーが空中（Y=2.0）にいる、IsGrounded == false、Y軸速度が 5.0 m/s
**When**: 1フレーム（deltaTime=0.016s）経過する
**Then**:
- Y軸速度が `5.0 + (-9.8) * 0.016` = 4.84 m/s になる
- Y座標が減少する

---

#### TST-30204-009: 着地判定
**対応**: REQ-30202-003
**Given**: プレイヤーが空中（Y=0.5）にいる、IsGrounded == false、Y軸速度が -5.0 m/s（下向き）
**When**: 0.1秒経過する（Y座標が0以下になる）
**Then**:
- プレイヤーのY座標が 0 に補正される
- Y軸速度が 0 にリセットされる
- IsGrounded が true になる
- PlayerLandEvent が発行される

---

#### TST-30204-010: 空中ジャンプの禁止
**対応**: REQ-30202-004
**Given**: プレイヤーが空中（Y=2.0）にいる、IsGrounded == false
**When**: ジャンプボタン（Space キー）を押す
**Then**:
- ジャンプ入力が無視される
- Y軸速度が変化しない
- PlayerJumpEvent が発行されない

---

#### TST-30204-011: 天井衝突時のY軸速度リセット
**対応**: REQ-30202-005
**Given**: プレイヤーが空中（Y=4.9）にいる、Y軸速度が 5.0 m/s（上向き）
**When**: 0.1秒経過する（Y座標が 5.0 を超える）
**Then**:
- プレイヤーのY座標が 5.0 に補正される
- Y軸速度が 0 にリセットされる

---

#### TST-30204-012: ふっとばし中のジャンプ禁止
**対応**: REQ-30202-006
**Given**: プレイヤーが地面にいる、Knockback.IsActive == true
**When**: ジャンプボタン（Space キー）を押す
**Then**:
- ジャンプ入力が無視される
- PlayerJumpEvent が発行されない

---

#### TST-30204-013: PlayerJumpEvent の発行
**対応**: REQ-30202-007
**Given**: プレイヤーが地面にいる、EventBus がリスニング中
**When**: ジャンプボタンを押す
**Then**:
- PlayerJumpEvent が発行される
- イベントデータに `PlayerId`, `JumpVelocity` が含まれる

---

#### TST-30204-014: PlayerLandEvent の発行
**対応**: REQ-30202-008
**Given**: プレイヤーが空中にいる、EventBus がリスニング中
**When**: プレイヤーが着地する
**Then**:
- PlayerLandEvent が発行される
- イベントデータに `PlayerId`, `LandPosition` が含まれる

---

### ふっとばしテスト（30203_knockback_spec.md）

#### TST-30204-015: ふっとばし開始
**対応**: REQ-30203-001
**Given**: プレイヤーがコート中央にいる、ボールがプレイヤーに衝突する（BallHitEvent を受信）
**When**: ふっとばし処理が実行される
**Then**:
- ふっとばしベクトルが計算される（方向：ボール→プレイヤー）
- ふっとばし速度 = ボール速度 * 0.5
- Knockback.IsActive が true になる
- Knockback.Duration が 0.5秒に設定される
- Knockback.InvincibilityTime が 1.0秒に設定される
- PlayerKnockbackEvent が発行される

---

#### TST-30204-016: ふっとばし移動
**対応**: REQ-30203-002
**Given**: プレイヤーがコート中央にいる、Knockback.IsActive == true、ふっとばしベクトルが (5.0, 0, 0)
**When**: 1秒経過する
**Then**:
- プレイヤーが右方向に移動する

---

#### TST-30204-017: ふっとばし中の境界制限
**対応**: REQ-30203-003
**Given**: プレイヤーが右境界近く（X=4.9）にいる、Knockback.IsActive == true、ふっとばしベクトルが (5.0, 0, 0)
**When**: ふっとばし移動が実行される
**Then**:
- プレイヤーのX座標が +5.0 で停止する
- ふっとばしベクトルのX成分が 0 にリセットされる

---

#### TST-30204-018: ふっとばし終了
**対応**: REQ-30203-004
**Given**: プレイヤーがふっとばし中、Knockback.Duration が 0.01秒
**When**: 0.02秒経過する
**Then**:
- Knockback.IsActive が false になる
- ふっとばしベクトルが (0, 0, 0) にリセットされる

---

#### TST-30204-019: 無敵時間の管理
**対応**: REQ-30203-005
**Given**: プレイヤーがふっとばし中、Knockback.InvincibilityTime が 0.5秒
**When**: 0.5秒経過する
**Then**:
- InvincibilityTime が 0 以下になる
- ボールの衝突判定が有効になる

---

#### TST-30204-020: 操作不能時間の管理
**対応**: REQ-30203-006
**Given**: プレイヤーがふっとばし中、Knockback.Duration が 0.3秒
**When**: プレイヤーが移動入力を行う
**Then**: 入力が無視される
**When**: 0.3秒経過する
**Then**: 入力が再び受け付けられる

---

#### TST-30204-021: PlayerKnockbackEvent の発行
**対応**: REQ-30203-007
**Given**: プレイヤーがコート中央にいる、EventBus がリスニング中
**When**: ふっとばしが開始される
**Then**:
- PlayerKnockbackEvent が発行される
- イベントデータに `PlayerId`, `KnockbackVelocity`, `Duration`, `InvincibilityTime` が含まれる

---

#### TST-30204-022: ふっとばし中の重力適用
**対応**: REQ-30203-008
**Given**: プレイヤーが空中（Y=2.0）でふっとばし中、IsGrounded == false、Y軸速度が 5.0 m/s
**When**: 1フレーム（deltaTime=0.016s）経過する
**Then**:
- Y軸速度が `5.0 + (-9.8) * 0.016` = 4.84 m/s になる
- Y座標が減少する

---

## テスト実行

- 全テストは自動テストで実行可能にする
- 境界値テスト、異常系テストを含む
- イベント発行のテストは EventBus のモックを使用
