# Character System Overview

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

ジョイメカファイト風のパーツ分離キャラクターシステム。頭・胴体・手・膝・足が分離したロボットキャラクターを表現し、パラメトリックアニメーションにより滑らかな動きを実現する。

## 仕様書一覧

| ID | ファイル | 内容 |
|----|---------|------|
| 31001 | [31001_parts_spec.md](31001_parts_spec.md) | パーツ構成・Component設計 |
| 31002 | [31002_animation_spec.md](31002_animation_spec.md) | アニメーションシステム |

## アーキテクチャ

```
ArticulatedCharacter (親エンティティ)
├── Head (子エンティティ)
├── Torso (子エンティティ)
├── LeftHand (子エンティティ)
├── RightHand (子エンティティ)
├── LeftKnee (子エンティティ)
├── RightKnee (子エンティティ)
├── LeftFoot (子エンティティ)
└── RightFoot (子エンティティ)
```

## 設計原則

1. **1パーツ=1エンティティ**: 各パーツは独立したエンティティとしてスプライト・Transformを持つ
2. **Parent/Children階層**: Bevy標準の親子関係（1階層のみ）を使用
3. **パラメトリック補間**: キーフレーム間の位置・回転を線形/イージング補間
4. **データ外部化**: パーツ定義・アニメーションは`.ron`ファイルで外部化

## 関連ドキュメント

- [30200_overview.md](../302_player/30200_overview.md) - プレイヤーシステム概要
- [30801_shadow_spec.md](../308_presentation/30801_shadow_spec.md) - 影システム
