# ナムコファミリーテニス Reference

## 基本情報

- **プラットフォーム**: ファミコン (FC)
- **ジャンル**: スポーツ/テニスゲーム
- **発売年**: 1987年
- **開発**: ナムコ

## ゲーム概要

ファミリーコンピュータで発売されたテニスゲーム。シンプルな操作で本格的なテニスの試合を楽しめる。1人プレイと2人対戦が可能。

## コアループ

1. **サーブ** - ボールを打ち上げてサーブを打つ
2. **移動** - ボールの落下地点に移動
3. **返球** - タイミングを合わせてショットを打つ
4. **ラリー継続** - 相手のショットに対応
5. **ポイント獲得** - 相手がミスするまで続ける

## 画面構成

| 画面 | 説明 | 詳細 |
|------|------|------|
| タイトル | ゲーム開始画面 | [screens/90101_title.md](screens/90101_title.md) |
| モード選択 | 1P/2Pモード選択 | [screens/90102_mode_select.md](screens/90102_mode_select.md) |
| 試合画面 | メインゲーム画面 | [screens/90103_match.md](screens/90103_match.md) |
| リザルト | 試合結果表示 | [screens/90104_result.md](screens/90104_result.md) |

## 主要メカニクス

| メカニクス | 説明 | 詳細 |
|-----------|------|------|
| プレイヤー移動 | コート内の移動操作 | [mechanics/90111_player_movement.md](mechanics/90111_player_movement.md) |
| ショット操作 | 各種ショットの打ち分け | [mechanics/90112_shot_system.md](mechanics/90112_shot_system.md) |
| ボール物理 | ボールの挙動と軌道 | [mechanics/90113_ball_physics.md](mechanics/90113_ball_physics.md) |
| スコアリング | 得点システム | [mechanics/90114_scoring.md](mechanics/90114_scoring.md) |
| AI挙動 | コンピュータの動き | [mechanics/90115_ai_behavior.md](mechanics/90115_ai_behavior.md) |

## ゲームモード

1. **1人用モード**
   - プレイヤー vs コンピュータ
   - 難易度選択可能

2. **2人用モード**
   - プレイヤー vs プレイヤー
   - 対戦プレイ

## 特徴的な要素

- **視点**: トップビュー（俯瞰視点）
- **コート**: シングルスコート
- **キャラクター**: デフォルメされた人型キャラクター
- **操作性**: シンプルな十字キー+2ボタン操作

## 確度について

このドキュメントの情報は、主にWeb上の攻略情報やレトロゲーム情報から収集しています。具体的な数値や詳細な挙動については、各メカニクスファイルで個別に確度を明記しています。

## 検索インデックス

### 機能別
| 機能 | ファイル | タグ |
|------|---------|------|
| プレイヤー移動 | mechanics/90111_player_movement.md | player, movement |
| ショット操作 | mechanics/90112_shot_system.md | player, shot, interaction |
| ボール物理 | mechanics/90113_ball_physics.md | ball, physics |
| スコアリング | mechanics/90114_scoring.md | scoring, system |
| AI挙動 | mechanics/90115_ai_behavior.md | ai, npc |

### Entity別
| Entity | 関連ファイル |
|--------|-------------|
| Player | 90111_player_movement.md, 90112_shot_system.md |
| Ball | 90113_ball_physics.md |
| AI | 90115_ai_behavior.md |

---

## 変更履歴

| 日付 | 変更内容 |
|------|---------|
| 2025-12-23 | 初版作成 |
