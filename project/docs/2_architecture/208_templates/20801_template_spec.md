# [番号] [機能名] 仕様書

<!-- セッション情報（並列実行時に自動記入） -->
<!-- Session: -->
<!-- ID Range: -->
<!-- Locked at: -->

---

## 1. Functional Requirements（EARS記法）

機能的要件。AIが実装時に従うべき明確な仕様。

### REQ-xxxxx-001: [要件名]

[EARS pattern を使用]

例:
- WHEN [event]
- THE SYSTEM SHALL [action]
- WITH [parameter]

**Precondition**: [事前条件]
**Postcondition**: [事後条件]

- **Test**: TST-xxxxx-001
- **Issue**: #xx

---

### REQ-xxxxx-002: [要件名]

...

---

## 2. Game Feel Requirements（感覚的品質）

EARS記法で表現できない、ゲーム特有の品質要件。
実装後のプレイテストで調整する。

### FEEL-xxxxx-001: [感覚的品質名]

**目標**: [プレイヤーがどう感じるべきか]

例: 「プレイヤーがジャンプ操作に応答性を感じること」

**測定方法**:
- [ ] [具体的な測定基準1]
- [ ] [具体的な測定基準2]

例:
- [ ] Coyote time: 0.1s（崖から落ちた直後もジャンプ可）
- [ ] Jump buffer: 0.15s（着地前の入力を受け付ける）
- [ ] Jump arc: 放物線（apex at 60% of duration）

**参照ゲーム**: [既存ゲームの例]

例: Celeste, Hollow Knight

**調整パラメータ**:

| Parameter | Initial | Min | Max | Notes |
|-----------|---------|-----|-----|-------|
| [パラメータ名] | [初期値] | [最小値] | [最大値] | [備考] |

例:
| Parameter | Initial | Min | Max | Notes |
|-----------|---------|-----|-----|-------|
| JumpVelocity | 12 | 8 | 15 | プレイテストで調整 |
| CoyoteTime | 0.1 | 0 | 0.2 | 崖落下後の猶予 |
| JumpBuffer | 0.15 | 0 | 0.3 | 着地前入力受付 |

**実装優先度**: EARS要件の実装後

---

### FEEL-xxxxx-002: [感覚的品質名]

...

---

## 3. Extensibility Requirements（拡張性）

将来の拡張を見越した設計要件。

### EXT-xxxxx-001: [拡張シナリオ]

WHERE [拡張条件]
THE SYSTEM SHALL [拡張方法]
BY [実現手段]

例:
- WHERE new enemy type is added
- THE SYSTEM SHALL support without code changes
- BY data file + new AIStrategy only

**Expected expansion**: [予想される拡張数]

例: Enemy types: 3 → 20+

**Current**: [現在の実装]

例: 3 enemy types (slime, goblin, bat)

**Design pattern**: [使用するパターン]

例: Factory + Strategy pattern (see 30104_enemy_module.md)

**Impact analysis**:

| Component | Change required | Impact |
|-----------|----------------|--------|
| [コンポーネント名] | [必要な変更] | [影響度] |

例:
| Component | Change required | Impact |
|-----------|----------------|--------|
| EnemyFactory | None | Low |
| AIStrategy | New class | Low |
| EnemyData | New entry | Low |
| EnemySystem | None | None |

---

### EXT-xxxxx-002: [拡張シナリオ]

...

---

## 4. Dependencies（依存関係）

この機能が依存する他の機能。

- [ ] 20902_health.md（HealthComponent）
- [ ] 20005_event_system.md（DamageEvent）
- [ ] 30103_player_behavior.md（PlayerMovementSystem）

**禁止依存**:
- ❌ [他機能への直接参照] → [代替手段]

例:
- ❌ Player → Enemy（直接参照禁止） → EventSystemを使用

---

## 5. Constraints（制約）

実装時の制約条件。

### Performance

- [ ] Update()は毎フレーム実行される（60fps想定）
- [ ] Entity数: 最大 [数値]
- [ ] メモリ使用量: [制限]

例:
- [ ] Update()は毎フレーム実行される（60fps想定）
- [ ] Entity数: 最大1000
- [ ] メモリ使用量: 1MB以内

### Platform

- [ ] Unity 2022.3 LTS / Godot 4.x
- [ ] Windows / macOS / Linux
- [ ] [その他のプラットフォーム要件]

### Technical

- [ ] ECSアーキテクチャに準拠
- [ ] [その他の技術的制約]

---

## 6. Non-Goals（対象外）

この機能では**実装しない**ことを明示。

- ❌ [実装しない機能1]
- ❌ [実装しない機能2]

例:
- ❌ 二段ジャンプ（将来の拡張で対応）
- ❌ 壁ジャンプ（別機能として実装）

---

## 7. References（参考資料）

- 9_reference/901_celeste/90101_movement.md
- 9_reference/902_hollow_knight/90201_jump.md
- [外部ドキュメントURL]

---

## 8. Change History（変更履歴）

| Date | Version | Author | Changes |
|------|---------|--------|---------|
| 2025-XX-XX | 1.0 | [名前] | 初版作成 |

---

## 使い方

### 1. Functional Requirements（必須）

すべての機能は **必ず** Functional Requirements から始める。
EARS記法を使って、AIが実装可能な明確な仕様を書く。

**チェックポイント**:
- [ ] すべての要件がEARSパターンに従っている
- [ ] 測定可能な基準が含まれている
- [ ] 曖昧な表現（「すぐに」「適切に」）がない
- [ ] Precondition/Postconditionが明記されている

### 2. Game Feel Requirements（オプション）

ゲーム特有の感覚的品質を記述する。
実装後のプレイテストで調整するパラメータを明記。

**使用タイミング**:
- プレイヤーの感覚が重要な機能（ジャンプ、攻撃、移動）
- 調整が必要なパラメータがある場合

**スキップ可能な場合**:
- UI機能（タイトル画面、設定画面）
- データ管理機能（セーブ/ロード）

### 3. Extensibility Requirements（条件付き）

将来の拡張が見込まれる機能にのみ記述。

**使用タイミング**:
- EXT-ID が仕様書に含まれている場合
- 「種類が増える」「バリエーションが増える」機能

**スキップ可能な場合**:
- 一度作ったら変更しない機能
- プロジェクト固有の機能

---

## 記入例

実際の記入例は以下を参照：

- `docs/examples/30101_player_spec_example.md`（完全版）
- `docs/examples/40101_title_spec_example.md`（Game Feel省略版）
