//! コート座標系・境界定義
//! @spec 30501_court_spec.md
//!
//! ## 座標系 (REQ-30501-001)
//! - X軸: 左右方向（-X: 左、+X: 右）
//! - Y軸: 高さ方向（0: 地面、+Y: 上）
//! - Z軸: 前後方向（-Z: 1Pコート側、+Z: 2Pコート側）
//!
//! ## アウトの不存在 (REQ-30501-007)
//! コートは壁で完全に囲まれており、アウト判定は存在しない。
//! 失点条件はツーバウンド、ネット、自コート打球のみ。

/// コート区分（1P/2Pのどちら側か）
/// @spec 30501_court_spec.md#req-30501-006
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CourtSide {
    /// 1Pコート側（Z < net_z）
    #[default]
    Player1,
    /// 2Pコート側（Z > net_z）
    Player2,
}

impl CourtSide {
    /// 相手側を返す
    #[inline]
    pub fn opponent(&self) -> Self {
        match self {
            CourtSide::Player1 => CourtSide::Player2,
            CourtSide::Player2 => CourtSide::Player1,
        }
    }
}

/// コート境界情報
/// @spec 30501_court_spec.md#req-30501-002
/// @spec 30501_court_spec.md#req-30501-003
/// @spec 30501_court_spec.md#req-30501-004
#[derive(Debug, Clone, Copy)]
pub struct CourtBounds {
    /// 左端X座標 (REQ-30501-002)
    pub left: f32,
    /// 右端X座標 (REQ-30501-002)
    pub right: f32,
    /// 後端Z座標（1P側）(REQ-30501-003)
    pub back_1p: f32,
    /// 後端Z座標（2P側）(REQ-30501-003)
    pub back_2p: f32,
    /// 地面Y座標
    pub ground: f32,
    /// 天井Y座標 (REQ-30501-004)
    pub ceiling: f32,
}

impl CourtBounds {
    /// 境界情報を生成
    /// @spec 30501_court_spec.md#req-30501-002
    /// @spec 30501_court_spec.md#req-30501-003
    /// @spec 30501_court_spec.md#req-30501-004
    pub fn new(left: f32, right: f32, back_1p: f32, back_2p: f32, ground: f32, ceiling: f32) -> Self {
        Self {
            left,
            right,
            back_1p,
            back_2p,
            ground,
            ceiling,
        }
    }

    /// 位置がコート内かチェック
    #[inline]
    pub fn is_inside(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.left
            && x <= self.right
            && y >= self.ground
            && y <= self.ceiling
            && z >= self.back_1p
            && z <= self.back_2p
    }

    /// X座標がコート左右境界内かチェック (REQ-30501-002)
    #[inline]
    pub fn is_within_x(&self, x: f32) -> bool {
        x >= self.left && x <= self.right
    }

    /// Y座標がコート上下境界内かチェック (REQ-30501-004)
    #[inline]
    pub fn is_within_y(&self, y: f32) -> bool {
        y >= self.ground && y <= self.ceiling
    }

    /// Z座標がコート前後境界内かチェック (REQ-30501-003)
    #[inline]
    pub fn is_within_z(&self, z: f32) -> bool {
        z >= self.back_1p && z <= self.back_2p
    }

    /// X座標を境界内にクランプ
    #[inline]
    pub fn clamp_x(&self, x: f32) -> f32 {
        x.clamp(self.left, self.right)
    }

    /// Y座標を境界内にクランプ
    #[inline]
    pub fn clamp_y(&self, y: f32) -> f32 {
        y.clamp(self.ground, self.ceiling)
    }

    /// Z座標を境界内にクランプ
    #[inline]
    pub fn clamp_z(&self, z: f32) -> f32 {
        z.clamp(self.back_1p, self.back_2p)
    }
}

/// ネット情報
/// @spec 30501_court_spec.md#req-30501-005
#[derive(Debug, Clone, Copy)]
pub struct NetInfo {
    /// ネットのZ座標（コート中央、1P/2Pの境界）
    pub z: f32,
    /// ネットの高さ
    pub height: f32,
}

impl NetInfo {
    /// ネット情報を生成
    /// @spec 30501_court_spec.md#req-30501-005
    pub fn new(z: f32, height: f32) -> Self {
        Self { z, height }
    }

    /// 指定位置がネットに衝突するかチェック
    /// ネット位置（Z座標）にいて、ネット高さ未満の場合に衝突
    #[inline]
    pub fn is_collision(&self, y: f32, z: f32, tolerance: f32) -> bool {
        (z - self.z).abs() < tolerance && y < self.height
    }
}

/// コート区分を判定
/// @spec 30501_court_spec.md#req-30501-006
///
/// # Arguments
/// * `z` - 判定するZ座標
/// * `net_z` - ネットのZ座標
///
/// # Returns
/// * `CourtSide::Player1` - Z < net_z（1Pコート側）
/// * `CourtSide::Player2` - Z >= net_z（2Pコート側、ネット上含む）
#[inline]
pub fn determine_court_side(z: f32, net_z: f32) -> CourtSide {
    if z < net_z {
        CourtSide::Player1
    } else {
        CourtSide::Player2
    }
}

/// コート全体の情報を保持
/// @spec 30501_court_spec.md
#[derive(Debug, Clone, Copy)]
pub struct Court {
    pub bounds: CourtBounds,
    pub net: NetInfo,
}

impl Court {
    /// コート情報を生成
    pub fn new(bounds: CourtBounds, net: NetInfo) -> Self {
        Self { bounds, net }
    }

    /// 指定位置のコート区分を判定
    /// @spec 30501_court_spec.md#req-30501-006
    #[inline]
    pub fn get_court_side(&self, z: f32) -> CourtSide {
        determine_court_side(z, self.net.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用の境界を生成（width=10, depth=6, ceiling=5）
    fn test_bounds() -> CourtBounds {
        // left=-5, right=5, back_1p=-3, back_2p=3, ground=0, ceiling=5
        CourtBounds::new(-5.0, 5.0, -3.0, 3.0, 0.0, 5.0)
    }

    /// テスト用のネット情報を生成（z=0, height=1）
    fn test_net() -> NetInfo {
        NetInfo::new(0.0, 1.0)
    }

    /// テスト用のコート全体を生成
    fn test_court() -> Court {
        Court::new(test_bounds(), test_net())
    }

    /// TST-30504-001: コート座標系
    #[test]
    fn test_req_30501_001_coordinate_system() {
        let court = test_court();

        // X軸: -5.0（左）〜 +5.0（右）
        assert_eq!(court.bounds.left, -5.0);
        assert_eq!(court.bounds.right, 5.0);

        // Y軸: 0（地面）〜 5.0（天井）
        assert_eq!(court.bounds.ground, 0.0);
        assert_eq!(court.bounds.ceiling, 5.0);

        // Z軸: -3.0（1P側）〜 +3.0（2P側）
        assert_eq!(court.bounds.back_1p, -3.0);
        assert_eq!(court.bounds.back_2p, 3.0);
    }

    /// TST-30504-002: コート境界（左右）
    #[test]
    fn test_req_30501_002_boundary_x() {
        let bounds = test_bounds();

        // 左端: -5.0, 右端: +5.0
        assert!(bounds.is_within_x(0.0));
        assert!(bounds.is_within_x(-5.0));
        assert!(bounds.is_within_x(5.0));
        assert!(!bounds.is_within_x(-5.1));
        assert!(!bounds.is_within_x(5.1));
    }

    /// TST-30504-003: コート境界（前後）
    #[test]
    fn test_req_30501_003_boundary_z() {
        let bounds = test_bounds();

        // 後端（1P側）: -3.0, 後端（2P側）: +3.0
        assert!(bounds.is_within_z(0.0));
        assert!(bounds.is_within_z(-3.0));
        assert!(bounds.is_within_z(3.0));
        assert!(!bounds.is_within_z(-3.1));
        assert!(!bounds.is_within_z(3.1));
    }

    /// TST-30504-004: コート境界（天井）
    #[test]
    fn test_req_30501_004_boundary_ceiling() {
        let bounds = test_bounds();

        // 天井: 5.0
        assert!(bounds.is_within_y(0.0));
        assert!(bounds.is_within_y(5.0));
        assert!(!bounds.is_within_y(5.1));
        assert!(!bounds.is_within_y(-0.1));
    }

    /// TST-30504-005: ネット位置
    #[test]
    fn test_req_30501_005_net_position() {
        let net = test_net();

        // ネット位置: Z = 0.0, 高さ = 1.0
        assert_eq!(net.z, 0.0);
        assert_eq!(net.height, 1.0);

        // ネット衝突判定
        assert!(net.is_collision(0.5, 0.0, 0.1)); // ネット位置、高さ未満
        assert!(!net.is_collision(1.5, 0.0, 0.1)); // ネット位置、高さ超過
        assert!(!net.is_collision(0.5, 1.0, 0.1)); // ネット位置外
    }

    /// TST-30504-006: コート区分（1P/2P）
    #[test]
    fn test_req_30501_006_court_side() {
        let court = test_court();

        // 1Pコート範囲: Z < 0
        assert_eq!(court.get_court_side(-1.0), CourtSide::Player1);
        assert_eq!(court.get_court_side(-3.0), CourtSide::Player1);

        // 2Pコート範囲: Z >= 0
        assert_eq!(court.get_court_side(1.0), CourtSide::Player2);
        assert_eq!(court.get_court_side(3.0), CourtSide::Player2);
        assert_eq!(court.get_court_side(0.0), CourtSide::Player2); // ネット上は2P側
    }

    /// REQ-30501-007: アウトの不存在（設計検証）
    /// コートは壁で完全に囲まれているため、アウト判定は存在しない
    #[test]
    fn test_req_30501_007_no_out_of_bounds() {
        // このテストは設計上の確認
        // アウト判定メソッドが存在しないことを確認（コンパイル時チェック）
        // 壁反射は別モジュールで実装される
    }

    #[test]
    fn test_court_bounds_is_inside() {
        let bounds = test_bounds();

        // コート中央
        assert!(bounds.is_inside(0.0, 2.5, 0.0));

        // コート端
        assert!(bounds.is_inside(-5.0, 0.0, -3.0));
        assert!(bounds.is_inside(5.0, 5.0, 3.0));

        // コート外
        assert!(!bounds.is_inside(-6.0, 2.5, 0.0));
        assert!(!bounds.is_inside(0.0, 6.0, 0.0));
        assert!(!bounds.is_inside(0.0, 2.5, 4.0));
    }

    #[test]
    fn test_court_bounds_clamp() {
        let bounds = test_bounds();

        // クランプ動作
        assert_eq!(bounds.clamp_x(-10.0), -5.0);
        assert_eq!(bounds.clamp_x(10.0), 5.0);
        assert_eq!(bounds.clamp_x(0.0), 0.0);

        assert_eq!(bounds.clamp_y(-1.0), 0.0);
        assert_eq!(bounds.clamp_y(10.0), 5.0);
        assert_eq!(bounds.clamp_y(2.5), 2.5);

        assert_eq!(bounds.clamp_z(-5.0), -3.0);
        assert_eq!(bounds.clamp_z(5.0), 3.0);
        assert_eq!(bounds.clamp_z(0.0), 0.0);
    }
}
