---
description: 次の連番タスクIDを取得
---

# /id-next コマンド

次のタスクIDを取得します。

## 使用方法

```
/id-next
```

## 処理手順

1. 既存タスクファイルを検索

```bash
ls tasks/*/T*.md 2>/dev/null
```

2. 既存IDを抽出

```
T001, T002, T003, ...
```

3. 最大番号 + 1 を計算

4. 結果を出力

```
次のID: T004

既存タスクID:
- T001
- T002
- T003
```

## ID形式

```
T{3桁の連番}

例: T001, T002, T003, ...
```

**設定**: `.taskrc.yaml` で prefix と format を変更可能

```yaml
id_numbering:
  prefix: "T"
  start: 1
  format: "T{:03d}"
```

## 注意事項

- 欠番は埋めない（常に最大値 + 1）
- 連番は 001 から開始
- 並行作業時はID衝突の可能性あり（手動で振り直し）
