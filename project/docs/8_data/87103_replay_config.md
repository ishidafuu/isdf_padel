# Replay Configuration

## 概要

リプレイ機能の設定データ。ファイル管理、保存制限に関するパラメータを定義。

## データ定義

### file_management

ファイル管理設定。

| パラメータ | 型 | デフォルト値 | 説明 |
|-----------|-----|-------------|------|
| save_directory | `String` | "assets/replays" | リプレイ保存先ディレクトリ |
| file_prefix | `String` | "replay_" | ファイル名プレフィックス |
| max_replay_count | `u32` | 100 | 保存上限件数 |

**参照元**: REQ-77103-003, REQ-77103-005

### cleanup_policy

クリーンアップポリシー。

| パラメータ | 型 | デフォルト値 | 説明 |
|-----------|-----|-------------|------|
| delete_on_version_mismatch | `bool` | true | バージョン不一致時に削除 |
| delete_oldest_on_limit | `bool` | true | 上限超過時に古いものを削除 |

**参照元**: REQ-77103-004, REQ-77103-005

---

## RON設定例

```ron
// assets/config/replay_config.ron
(
    file_management: (
        save_directory: "assets/replays",
        file_prefix: "replay_",
        max_replay_count: 100,
    ),
    cleanup_policy: (
        delete_on_version_mismatch: true,
        delete_oldest_on_limit: true,
    ),
)
```

## 依存関係

- `77103_replay_spec.md`: リプレイ仕様
