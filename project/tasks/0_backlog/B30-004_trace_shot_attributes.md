# B30-004: トレース設定のshot_attributes有効化

## 概要

トレース記録設定で `shot_attributes` がデフォルトで無効になっており、ナラティブ生成時にショットカウントが0になる問題を修正する。

## 問題詳細

- **タイプ**: バグ修正（設定）
- **優先度**: Major
- **原因特定済み**: Yes
- **QAレポート**: `project/qa_reports/2026-01-19_072511/`

## 症状

- `trace_narrator` でTotal Shots = 0
- ショット属性情報がトレースに記録されない

## 根本原因

`tracer.config.shot_attributes = false`（デフォルト）のため、`ShotAttributesCalculatedEvent` がトレースに記録されない。

## 修正内容

シミュレーション設定ファイルに `shot_attributes: true` を追加する。

## 修正対象ファイル

- `project/assets/config/simulation_debug.ron`
- `project/assets/config/simulation_stress.ron`

## 修正例

```ron
// simulation_debug.ron
(
    tracer: (
        enabled: true,
        shot_attributes: true,  // 追加
        // ... 他の設定
    ),
    // ...
)
```

## 検証方法

1. `/qa-cycle` でQAサイクル再実行
2. ナラティブレポートでショットカウントが正しく表示されることを確認
3. トレースファイルに `ShotAttributesCalculated` イベントが含まれることを確認

## 関連

- 関連イベント: ShotAttributesCalculatedEvent
- 関連コンポーネント: Tracer, SimulationConfig
