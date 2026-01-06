# Height Component

## Document Info
- ID: COMP-20903
- Version: 1.0.0
- Last Updated: 2025-12-23
- Related: 20901_position.md, 20902_velocity.md

## 概要
地面からの高さを表す Component。ボールが持つ。

## 使用機能
- Ball（テニスボール）

## 定義

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| value | f32 | 0.0 | 地面からの高さ（ピクセル） |

## C# 実装例

```csharp
public struct Height
{
    public float Value;

    public Height(float value = 0.0f)
    {
        Value = value;
    }
}
```

## 高さの単位

- **単位**: ピクセル（仮想的な高さ）
- **範囲**: 0.0 以上（負の値は地面以下なので不正）
- **最大高さ**: 制限なし（物理演算による自然な制約）

## 物理演算との関係

Height は重力の影響を受ける：

```csharp
public void BallPhysicsSystem(Query<Velocity, Height, Ball> query, float deltaTime)
{
    const float GRAVITY = -980.0f; // px/s^2

    foreach (var (vel, height, _) in query)
    {
        // 重力を適用（Y 方向の速度は高さ方向）
        vel.Y += GRAVITY * deltaTime;

        // 高さを更新
        height.Value += vel.Y * deltaTime;

        // 地面との衝突
        if (height.Value <= 0.0f)
        {
            height.Value = 0.0f;
            vel.Y = -vel.Y * 0.7f; // バウンド（反発係数 0.7）
        }
    }
}
```

## 影のスケールとの関係

Height が高いほど、影が小さくなる：

```csharp
public void ShadowSystem(
    Query<Position, Height, Ball> ballQuery,
    Query<Position, Scale, ShadowOwner, Shadow> shadowQuery)
{
    foreach (var (shadowPos, shadowScale, owner, _) in shadowQuery)
    {
        var ballEntity = owner.Owner;
        if (ballQuery.TryGet(ballEntity, out var ballPos, out var ballHeight))
        {
            // 影の位置はボールの真下
            shadowPos.X = ballPos.X;
            shadowPos.Y = ballPos.Y;

            // 影のスケールは高さに反比例
            shadowScale.Value = 1.0f / (1.0f + ballHeight.Value / 100.0f);
        }
    }
}
```

## 注意事項
- Height は物理的な高さを表すが、描画時は影のスケールで表現する
- Position とは独立（Position は 2D 座標、Height は 3D 的な高さ）
- 負の値は不正なので、物理演算で 0.0 にクランプする
