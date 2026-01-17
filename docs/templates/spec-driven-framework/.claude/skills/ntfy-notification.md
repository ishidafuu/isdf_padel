# ntfy 通知ガイド

Claude Code の承認待ち・実行完了をリモート通知するためのガイド。

## 概要

### 解決する問題
- 承認待ちに気づかず、タイムアウトで作業が止まる
- ターミナルから離れられない

### 解決策
- [ntfy](https://ntfy.sh) によるプッシュ通知
- スマホ/PC でどこからでも受信

---

## セットアップ

### 1. 購読（受信側）

**スマホ**: ntfy アプリをインストール → トピック名を購読
**ブラウザ**: `https://ntfy.sh/your-topic-name` にアクセス

### 2. Hook スクリプト（送信側）

```bash
# ~/.claude/hooks/permission-requested.sh
#!/bin/bash
osascript -e 'display notification "権限の承認が必要です" with title "Claude Code"'
curl -s -d "権限の承認が必要です" ntfy.sh/your-topic-name >/dev/null 2>&1 &
exit 0
```

```bash
# ~/.claude/hooks/execution-complete.sh
#!/bin/bash
osascript -e 'display notification "実行完了。入力可能です" with title "Claude Code"'
curl -s -d "実行完了。入力可能です" ntfy.sh/your-topic-name >/dev/null 2>&1 &
exit 0
```

### 3. トピック名

- 推測されにくいランダムな名前を推奨
- 例: `claude-abc123`, `myproject-notify-xyz`

---

## 使用タイミング

### 使うべき場合
- 長時間タスクを実行中で席を離れたい
- 外出先から承認待ち状況を確認したい
- 承認待ちタイムアウトを防ぎたい

### 使わなくてよい場合
- ターミナルに集中している短時間作業
- ネットワーク接続なしのオフライン作業

---

## セキュリティ

- トピック名は秘密にする（URLを知っている人は誰でも購読可能）
- 機密情報を通知に含めない
- 必要に応じて [ntfy セルフホスト](https://docs.ntfy.sh/install/) を検討

---

## 関連リンク

- [ntfy 公式サイト](https://ntfy.sh)
- [ntfy ドキュメント](https://docs.ntfy.sh)
