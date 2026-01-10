# B30902-001: fault_judgmentテスト失敗修正

## 概要

サービスボックス判定テストが3件失敗。テストの期待値が古い座標系ルールを反映しており、実装は正しい。

## 問題

| テスト名 | 実際値 | 期待値（誤り） |
|---------|--------|---------------|
| `test_req_30902_001_service_box_judgment` | z_min=-5.0 | z_min=0.0 |
| `test_req_30902_001_service_box_ad_side` | z_min=0.0 | z_min=-5.0 |
| `test_req_30902_001_service_box_player2_serve` | z_min=0.0 | z_min=-5.0 |

## 根本原因

座標系統一コミット後にテストが更新されていない

## 修正ファイル

- `src/systems/fault_judgment.rs`

## 関連仕様

- `docs/3_ingame/309_rules/30902_fault_spec.md`

## ステータス

- [ ] テスト期待値修正
- [ ] テスト実行確認
