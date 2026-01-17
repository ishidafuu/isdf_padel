# EARS記法

要件を明確に記述するための構造化記法。

## 参照元ガイドライン

- 📋 spec-agent.md - 要件仕様の記述
- 💬 requirements-agent.md - 要件の深掘り・明確化
- 🔍 critic-agent.md - 要件の曖昧さ検出

---

## パターン一覧

| パターン | 構文 | 用途 |
|----------|------|------|
| Ubiquitous | THE SYSTEM SHALL [動作] | 常に成り立つ要件 |
| Event-driven | WHEN [イベント], THE SYSTEM SHALL [動作] | イベント起因の要件 |
| State-driven | WHILE [状態], THE SYSTEM SHALL [動作] | 状態依存の要件 |
| Optional | WHERE [条件], THE SYSTEM SHALL [動作] | 条件付きの要件 |
| Unwanted | IF [望ましくない状況], THEN THE SYSTEM SHALL [対応] | 異常系の要件 |

---

## 詳細説明

### Ubiquitous（普遍的要件）

**常に成り立つ要件**。条件やイベントに依存しない。

```
THE SYSTEM SHALL [動作]
```

**例:**
```
THE SYSTEM SHALL 60FPSで動作する
THE SYSTEM SHALL ゲームデータを暗号化して保存する
```

### Event-driven（イベント駆動要件）

**特定のイベント発生時に動作する要件**。最も頻出。

```
WHEN [イベント]
THE SYSTEM SHALL [動作]
WITH [パラメータ]（オプション）
```

**例:**
```
WHEN プレイヤーがジャンプボタンを押す
THE SYSTEM SHALL プレイヤーを上方向に加速させる
WITH 初速度 10m/s
```

```
WHEN 敵がプレイヤーに接触する
THE SYSTEM SHALL プレイヤーにダメージを与える
WITH ダメージ量 = 敵の攻撃力
```

### State-driven（状態駆動要件）

**特定の状態にある間、継続的に動作する要件**。

```
WHILE [状態]
THE SYSTEM SHALL [動作]
WITH [パラメータ]（オプション）
```

**例:**
```
WHILE プレイヤーが空中にいる
THE SYSTEM SHALL 重力を適用する
WITH 加速度 -9.8m/s²
```

```
WHILE プレイヤーが水中にいる
THE SYSTEM SHALL 移動速度を50%に低下させる
```

### Optional（条件付き要件）

**特定の条件下でのみ有効な要件**。

```
WHERE [条件]
THE SYSTEM SHALL [動作]
```

**例:**
```
WHERE プレイヤーがパワーアップアイテムを所持している
THE SYSTEM SHALL 攻撃力を2倍にする
```

```
WHERE デバッグモードが有効
THE SYSTEM SHALL FPSカウンターを表示する
```

### Unwanted（異常系要件）

**望ましくない状況への対処を定義する要件**。

```
IF [望ましくない状況]
THEN THE SYSTEM SHALL [対応]
```

**例:**
```
IF プレイヤーのHPが0以下になった
THEN THE SYSTEM SHALL ゲームオーバー画面を表示する
```

```
IF セーブデータが破損している
THEN THE SYSTEM SHALL 新規ゲームを開始するか確認する
```

---

## 複合パターン

複数の条件を組み合わせることも可能:

```
WHEN プレイヤーが攻撃ボタンを押す
AND プレイヤーが地上にいる
THE SYSTEM SHALL 地上攻撃を実行する
```

```
WHEN プレイヤーがジャンプボタンを押す
AND プレイヤーが空中にいる
AND 二段ジャンプが未使用
THE SYSTEM SHALL 二段ジャンプを実行する
```

---

## WITH句の使い方

`WITH`句でパラメータや詳細条件を指定:

```
WHEN プレイヤーがダッシュを開始する
THE SYSTEM SHALL プレイヤーの移動速度を増加させる
WITH 速度倍率 2.0
WITH 持続時間 3秒
WITH クールダウン 5秒
```

---

## ベストプラクティス

### 1. 1要件1動作

悪い例:
```
WHEN ジャンプボタンを押す
THE SYSTEM SHALL ジャンプし、効果音を再生し、アニメーションを開始する
```

良い例:
```
WHEN ジャンプボタンを押す
THE SYSTEM SHALL ジャンプを開始する

WHEN ジャンプが開始される
THE SYSTEM SHALL ジャンプ効果音を再生する

WHEN ジャンプが開始される
THE SYSTEM SHALL ジャンプアニメーションを開始する
```

### 2. 曖昧な表現を避ける

悪い例:
```
THE SYSTEM SHALL すぐに反応する
```

良い例:
```
THE SYSTEM SHALL 1フレーム以内に反応する
```

### 3. 数値を明示する

悪い例:
```
THE SYSTEM SHALL プレイヤーを速く移動させる
```

良い例:
```
THE SYSTEM SHALL プレイヤーを移動させる
WITH 移動速度 5m/s
```

---

## テンプレート

### spec.md での使用例

```markdown
## 要件

### REQ-30101-001: ジャンプ開始

- WHEN プレイヤーがジャンプボタンを押す
- AND プレイヤーが接地している
- THE SYSTEM SHALL プレイヤーを上方向に加速させる
- WITH 初速度 12m/s

**テスト**: TST-30105-001

### REQ-30101-002: 空中制御

- WHILE プレイヤーが空中にいる
- THE SYSTEM SHALL 左右入力による水平移動を許可する
- WITH 空中制御係数 0.8

**テスト**: TST-30105-002
```

---

## 参考

- [EARS: The Easy Approach to Requirements Syntax (ICCGI 2013 Tutorial)](https://www.iaria.org/conferences2013/filesICCGI13/ICCGI_2013_Tutorial_Terzakis.pdf)
- 原論文: "Easy Approach to Requirements Syntax (EARS)" by Alistair Mavin et al. (IEEE RE'09)
