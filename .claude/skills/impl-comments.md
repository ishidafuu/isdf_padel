# 実装コメント規約

仕様書と実装コードの対応関係を明示するためのコメント規約。

## 参照元ガイドライン

- 💻 impl-agent.md - 実装時のコメント付与
- ✅ review-agent.md - コメント整合性の検証

---

## タグ一覧

| タグ | 用途 | 例 |
|---|---|---|
| `@spec` | 要件との対応 | `// @spec REQ-30101-001` |
| `@test` | テストとの対応 | `// @test TST-30105-001` |
| `@data` | データ定義との対応 | `// @data 80101_enemy_params.md#enemy_slime` |

---

## ルール

1. 要件を実装するメソッドやクラスに `@spec` コメントを付与
2. テストメソッドに `@test` コメントを付与
3. データ定義を参照する箇所に `@data` コメントを付与
4. 1つの実装が複数の要件に対応する場合は複数行で記載
5. コメントは実装の直前に配置

---

## 使用例

### 要件との対応（@spec）

```csharp
// @spec REQ-30101-001
// @spec REQ-30101-002
public class PlayerJumpSystem : ISystem
{
    public void Execute()
    {
        // ジャンプ処理
    }
}
```

### テストとの対応（@test）

```csharp
// @test TST-30105-001
[Test]
public void Jump_OnGround_ShouldApplyUpwardVelocity()
{
    // Given: プレイヤーが地上にいる
    // When: ジャンプボタンを押す
    // Then: プレイヤーが上方向に移動する
}
```

### データ定義との対応（@data）

```csharp
// @data 80101_enemy_params.md#enemy_slime
public static readonly EnemyParam Slime = new(hp: 10, attack: 2, speed: 1.0f);

// @data 80101_enemy_params.md#enemy_goblin
public static readonly EnemyParam Goblin = new(hp: 25, attack: 5, speed: 1.5f);
```

### 複数の要件に対応する場合

```csharp
// @spec REQ-30101-001 ジャンプ開始
// @spec REQ-30101-003 ジャンプバッファ
// @spec REQ-30101-005 着地判定
public class PlayerMovementSystem : ISystem
{
    // ...
}
```

---

## 検証

`/docs-validate` コマンドで以下を検証できます：

- 実装コードに `@spec` コメントが存在するか
- `@spec` が参照する REQ-ID が仕様書に存在するか
- `@test` が参照する TST-ID が仕様書に存在するか
- `@data` が参照するデータ定義が 8_data/ に存在するか

---

## テストメソッド命名

```csharp
// @test TST-30105-001
[Test]
public void [動作]_[条件]_[期待結果]()
{
    // Given/When/Then 形式のコメントで説明
}
```

例:
```csharp
// @test TST-30105-001
[Test]
public void Jump_OnGround_ShouldApplyUpwardVelocity() { }

// @test TST-30105-002
[Test]
public void Jump_InAir_ShouldNotJump() { }

// @test TST-30105-003
[Test]
public void Jump_WithBuffer_ShouldJumpAfterLanding() { }
```

---

## 多言語対応例

### TypeScript

```typescript
// @spec REQ-30101-001
// @spec REQ-30101-002
export class PlayerJumpSystem implements ISystem {
    execute(): void {
        // ジャンプ処理
    }
}

// @data 80101_enemy_params.md#enemy_slime
export const SLIME_PARAMS = { hp: 10, attack: 2, speed: 1.0 };
```

### Python

```python
# @spec REQ-30101-001
# @spec REQ-30101-002
class PlayerJumpSystem:
    def execute(self) -> None:
        # ジャンプ処理
        pass

# @data 80101_enemy_params.md#enemy_slime
SLIME_PARAMS = {"hp": 10, "attack": 2, "speed": 1.0}
```

### Rust (Bevy)

```rust
// @spec REQ-30101-001
// @spec REQ-30101-002
impl System for PlayerJumpSystem {
    fn execute(&mut self) {
        // ジャンプ処理
    }
}

// @data 80101_enemy_params.md#enemy_slime
pub const SLIME_PARAMS: EnemyParam = EnemyParam { hp: 10, attack: 2, speed: 1.0 };
```
