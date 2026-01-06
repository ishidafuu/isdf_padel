# 主要クラス一覧

**解析日**: YYYY-MM-DD
**解析者**: legacy-analyzer-agent
**解析方法**: Serena MCP による自動抽出

---

## クラス一覧（アルファベット順）

| クラス名 | ファイル | 推測される役割 | 重要度 |
|---------|---------|--------------|-------|
| GameState | src/game/GameState.cpp | ゲーム状態管理 | ⭐⭐⭐ |
| Player | src/game/Player.cpp | プレイヤーデータ | ⭐⭐⭐ |
| Enemy | src/game/Enemy.cpp | 敵キャラクター | ⭐⭐⭐ |
| Item | src/data/Item.cpp | アイテムデータ | ⭐⭐ |
| Stage | src/game/Stage.cpp | ステージ管理 | ⭐⭐ |
| Renderer | src/renderer/Renderer.cpp | 描画処理 | ⭐⭐ |
| InputManager | src/input/InputManager.cpp | 入力管理 | ⭐⭐ |
| Menu | src/ui/Menu.cpp | メニュー画面 | ⭐ |

---

## ゲームロジック関連

### GameState

**役割（推測）**:
ゲーム全体の状態を管理。タイトル、ゲーム中、ゲームオーバーなどのステート遷移を制御。

**メンバー変数（Serena MCPで取得）**:
```cpp
class GameState {
private:
    State currentState;  // 現在の状態
    Player* player;      // プレイヤー
    Stage* stage;        // 現在のステージ
    vector<Enemy*> enemies;  // 敵リスト
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `update()`: 毎フレームの更新
- `changeState()`: 状態遷移
- `handleEvent()`: イベント処理

**確認事項**:
- [ ] ステートの種類を教えてください
- [ ] イベントの種類を教えてください

**制作者による補完**:
- （例: ステートは TITLE, GAME, PAUSE, GAMEOVER の4種類）
- （例: イベントは PLAYER_DAMAGED, ENEMY_DEFEATED など）

**Phase 2 での詳細解析**: 必須 ⭐⭐⭐

---

### Player

**役割（推測）**:
プレイヤーキャラクターのデータと振る舞い。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Player {
private:
    int hp;           // HP
    int maxHp;        // 最大HP
    int attack;       // 攻撃力
    int defense;      // 防御力
    Position pos;     // 位置
    vector<Item*> items;  // 所持アイテム
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `update()`: プレイヤーの更新
- `move()`: 移動処理
- `takeDamage()`: ダメージを受ける
- `attack()`: 攻撃処理

**確認事項**:
- [ ] レベルの概念はありましたか？
- [ ] 経験値システムはありましたか？
- [ ] スキルや特殊能力はありましたか？

**制作者による補完**:
- （例: レベルシステムはあった。経験値で成長する）
- （例: スキルは未実装だった）

**Phase 2 での詳細解析**: 必須 ⭐⭐⭐

---

### Enemy

**役割（推測）**:
敵キャラクターのデータと振る舞い。AIも含む。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Enemy {
private:
    int hp;
    int attack;
    int defense;
    Position pos;
    EnemyType type;   // 敵の種類
    AIState aiState;  // AI状態
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `update()`: 敵の更新
- `ai()`: AI処理
- `attack()`: 攻撃処理

**確認事項**:
- [ ] 敵の種類は何種類ありましたか？
- [ ] AIのパターンはどの程度ありましたか？

**制作者による補完**:
- （例: 敵は10種類くらい。ボスを含めると15種類）
- （例: AIはシンプル。追跡、巡回、攻撃の3パターン）

**Phase 2 での詳細解析**: 推奨 ⭐⭐⭐

---

## データ管理関連

### Item

**役割（推測）**:
アイテムのデータ定義。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Item {
private:
    int itemId;
    string name;
    ItemType type;    // アイテム種類
    int value;        // 効果値
    // 他のメンバー変数...
};
```

**確認事項**:
- [ ] アイテムの種類は何種類ありましたか？
- [ ] 消費アイテムと装備アイテムの区別はありましたか？

**制作者による補完**:
- （例: アイテムは20種類程度。全て消費アイテム）

**Phase 2 での詳細解析**: 推奨 ⭐⭐

---

### Stage

**役割（推測）**:
ステージ（レベル、マップ）のデータと管理。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Stage {
private:
    int stageId;
    int width;
    int height;
    Tile** tiles;     // タイルマップ
    vector<Enemy*> enemies;  // 敵配置
    // 他のメンバー変数...
};
```

**確認事項**:
- [ ] ステージは何ステージありましたか？
- [ ] タイルベースのマップでしたか？

**制作者による補完**:
- （例: 全10ステージ。タイルベースで、32x32ピクセルのタイル）

**Phase 2 での詳細解析**: 推奨 ⭐⭐

---

## レンダリング関連

### Renderer

**役割（推測）**:
OpenGLを使った描画処理の中心。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Renderer {
private:
    // OpenGL関連の変数
    GLuint vbo;
    GLuint vao;
    vector<Texture*> textures;
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `initialize()`: 初期化
- `render()`: 描画実行
- `loadTexture()`: テクスチャ読み込み

**確認事項**:
- [ ] OpenGLのバージョンは？
- [ ] シェーダーは使用していましたか？

**制作者による補完**:
- （例: OpenGL 2.1。シェーダーは使っていない）

**Phase 2 での詳細解析**: 任意 ⭐⭐

---

## 入力処理関連

### InputManager

**役割（推測）**:
キーボード・マウスの入力を統合管理。

**メンバー変数（Serena MCPで取得）**:
```cpp
class InputManager {
private:
    bool keys[256];   // キー状態
    int mouseX, mouseY;  // マウス位置
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `update()`: 入力状態の更新
- `isKeyPressed()`: キーが押されているか
- `getMousePosition()`: マウス位置取得

**確認事項**:
- [ ] キーコンフィグは可能でしたか？
- [ ] マウスは使用していましたか？

**制作者による補完**:
- （例: キーコンフィグは未実装。マウスは未使用）

**Phase 2 での詳細解析**: 任意 ⭐⭐

---

## UI関連

### Menu

**役割（推測）**:
タイトル画面、ポーズメニューなどのメニュー表示。

**メンバー変数（Serena MCPで取得）**:
```cpp
class Menu {
private:
    int selectedIndex;  // 選択中の項目
    vector<string> items;  // メニュー項目
    // 他のメンバー変数...
};
```

**主要メソッド**:
- `render()`: メニュー描画
- `handleInput()`: 入力処理
- `selectItem()`: 項目選択

**Phase 2 での詳細解析**: 任意 ⭐

---

## 継承関係（推測）

```
GameObject (基底クラス？)
  ├─ Player
  ├─ Enemy
  └─ Item (?)

Renderable (基底クラス？)
  ├─ Sprite
  └─ TileMap
```

**確認事項**:
- [ ] 基底クラスは存在しましたか？
- [ ] 継承構造はどうなっていましたか？

**制作者による補完**:
- （例: GameObjectという基底クラスがあった）
- （例: 継承はあまり使っていない。ほぼフラットな構造）

---

## Phase 2 での解析優先順位

**最優先（⭐⭐⭐）**:
1. GameState - ゲーム全体の制御
2. Player - プレイヤーの詳細
3. Enemy - 敵とAI

**推奨（⭐⭐）**:
4. Stage - ステージ構造
5. Item - アイテムシステム
6. Renderer - 描画の詳細

**任意（⭐）**:
7. InputManager - 入力処理
8. Menu - UI

**制作者による調整**:
- （例: 「戦闘システムが複雑なので、EnemyとPlayerを優先したい」）

---

## メモ

**気づいた点**:
- （例: クラス間の結合が強い。依存関係が複雑）
- （例: ポインタの生ポインタが多用されている）

**制作者のメモ**:
- （例: 当時はスマートポインタを知らなかった）
- （例: デザインパターンはあまり使っていない）

---

## 参照

- [Overview](./overview.md)
- [ディレクトリ構造](./architecture/directories.md)
