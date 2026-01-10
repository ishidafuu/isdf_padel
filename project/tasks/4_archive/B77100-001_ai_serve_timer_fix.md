# B77100-001: AIサーブタイマーの FixedDeltaTime 対応

## 概要

ヘッドレスシミュレーションでAIサーブタイマーが実時間を使用しているため、高速シミュレーション時にタイマーが進まずサーブが開始されない問題を修正する。

## 背景

- R77100-001 で導入した FixedDeltaTime による高速シミュレーションは動作している
- しかし `ai_serve_toss_system` が `time.delta()` を使用しているため、タイマーが進まない
- 結果: Serve状態で60秒間stuck → StateStuck異常として検出

## 根本原因

`src/systems/ai/serve.rs` L111:
```rust
timer.tick(time.delta());  // ← 実時間ベース（高速シミュレーションではほぼ0）
```

## 修正内容

### 対象ファイル

- `src/systems/ai/serve.rs`

### 変更箇所

`ai_serve_toss_system` のタイマー更新を FixedDeltaTime 対応に:

```rust
// Before
timer.tick(time.delta());

// After
timer.tick(Duration::from_secs_f32(fixed_dt.delta_secs()));
```

システムシグネチャに `fixed_dt: Res<FixedDeltaTime>` を追加。

## 検証方法

```bash
cargo run --release --bin headless_sim -- -c debug
```

確認項目:
- サーブが正常に開始されること
- StateStuck異常が発生しないこと
- 試合が完了すること

## 関連

- R77100-001: ヘッドレスシミュレーション高速化（FixedDeltaTime導入）
- @spec 77100_headless_sim.md
