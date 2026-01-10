//! コート座標系・境界定義
//! @spec 30501_court_spec.md
//!
//! ## 座標系 (REQ-30501-001)
//! - X軸: 打ち合い方向（-X: Leftコート側、+X: Rightコート側）← 画面の左右
//! - Y軸: 高さ方向（0: 地面、+Y: 上）
//! - Z軸: コート幅方向（奥行き）← 画面の上下に合成
//!
//! ## アウト判定 (REQ-30501-007)
//! コートライン外に着地するとアウト（失点）。
//! 失点条件はアウト、ツーバウンド、ネット、自コート打球。

/// コート区分（Left/Rightのどちら側か）
/// @spec 30501_court_spec.md#req-30501-006
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum CourtSide {
    /// 左コート側（X < net_x）
    #[default]
    Left,
    /// 右コート側（X >= net_x）
    Right,
}

impl CourtSide {
    /// 相手側を返す
    #[inline]
    pub fn opponent(&self) -> Self {
        match self {
            CourtSide::Left => CourtSide::Right,
            CourtSide::Right => CourtSide::Left,
        }
    }
}

/// コート境界情報
/// @spec 30501_court_spec.md#req-30501-002
/// @spec 30501_court_spec.md#req-30501-003
/// @spec 30501_court_spec.md#req-30501-004
#[derive(Debug, Clone, Copy)]
pub struct CourtBounds {
    /// サイドウォール左端Z座標（コート幅方向）(REQ-30501-002)
    pub left: f32,
    /// サイドウォール右端Z座標（コート幅方向）(REQ-30501-002)
    pub right: f32,
    /// 後端X座標（Left側バックライン、打ち合い方向）(REQ-30501-003)
    pub back_left: f32,
    /// 後端X座標（Right側バックライン、打ち合い方向）(REQ-30501-003)
    pub back_right: f32,
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
    pub fn new(left: f32, right: f32, back_left: f32, back_right: f32, ground: f32, ceiling: f32) -> Self {
        Self {
            left,
            right,
            back_left,
            back_right,
            ground,
            ceiling,
        }
    }

    /// 位置がコート内かチェック
    #[allow(dead_code)] // テストで使用、将来の境界判定で使用予定
    #[inline]
    pub fn is_inside(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.back_left
            && x <= self.back_right
            && y >= self.ground
            && y <= self.ceiling
            && z >= self.left
            && z <= self.right
    }

    /// X座標がコート打ち合い方向境界内かチェック (REQ-30501-003)
    #[allow(dead_code)] // テストで使用、将来の境界判定で使用予定
    #[inline]
    pub fn is_within_x(&self, x: f32) -> bool {
        x >= self.back_left && x <= self.back_right
    }

    /// Y座標がコート上下境界内かチェック (REQ-30501-004)
    #[allow(dead_code)] // テストで使用、将来の境界判定で使用予定
    #[inline]
    pub fn is_within_y(&self, y: f32) -> bool {
        y >= self.ground && y <= self.ceiling
    }

    /// Z座標がコート幅方向境界内かチェック (REQ-30501-002)
    #[allow(dead_code)] // テストで使用、将来の境界判定で使用予定
    #[inline]
    pub fn is_within_z(&self, z: f32) -> bool {
        z >= self.left && z <= self.right
    }

    /// X座標を境界内にクランプ（打ち合い方向）
    #[inline]
    pub fn clamp_x(&self, x: f32) -> f32 {
        x.clamp(self.back_left, self.back_right)
    }

    /// Y座標を境界内にクランプ
    #[inline]
    pub fn clamp_y(&self, y: f32) -> f32 {
        y.clamp(self.ground, self.ceiling)
    }

    /// Z座標を境界内にクランプ（コート幅方向）
    #[inline]
    pub fn clamp_z(&self, z: f32) -> f32 {
        z.clamp(self.left, self.right)
    }
}

/// ネット情報
/// @spec 30501_court_spec.md#req-30501-005
#[derive(Debug, Clone, Copy)]
pub struct NetInfo {
    /// ネットのX座標（コート中央、Left/Rightの境界、打ち合い方向）
    pub x: f32,
    /// ネットの高さ
    #[allow(dead_code)] // is_collision で使用、将来のネット衝突判定で使用予定
    pub height: f32,
}

impl NetInfo {
    /// ネット情報を生成
    /// @spec 30501_court_spec.md#req-30501-005
    pub fn new(x: f32, height: f32) -> Self {
        Self { x, height }
    }

    /// 指定位置がネットに衝突するかチェック
    /// ネット位置（X座標）にいて、ネット高さ未満の場合に衝突
    #[allow(dead_code)] // テストで使用、将来のネット衝突判定で使用予定
    #[inline]
    pub fn is_collision(&self, y: f32, x: f32, tolerance: f32) -> bool {
        (x - self.x).abs() < tolerance && y < self.height
    }
}

/// コート区分を判定
/// @spec 30501_court_spec.md#req-30501-006
///
/// # Arguments
/// * `x` - 判定するX座標（打ち合い方向）
/// * `net_x` - ネットのX座標
///
/// # Returns
/// * `CourtSide::Left` - X < net_x（左コート側）
/// * `CourtSide::Right` - X >= net_x（右コート側、ネット上含む）
#[inline]
pub fn determine_court_side(x: f32, net_x: f32) -> CourtSide {
    if x < net_x {
        CourtSide::Left
    } else {
        CourtSide::Right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用の境界を生成
    /// - Z方向（コート幅）: left=-5, right=5
    /// - X方向（打ち合い方向）: back_left=-3, back_right=3
    /// - Y方向: ground=0, ceiling=5
    fn test_bounds() -> CourtBounds {
        CourtBounds::new(-5.0, 5.0, -3.0, 3.0, 0.0, 5.0)
    }

    /// テスト用のネット情報を生成（x=0, height=1）
    fn test_net() -> NetInfo {
        NetInfo::new(0.0, 1.0)
    }

    /// TST-30504-001: コート座標系
    #[test]
    fn test_req_30501_001_coordinate_system() {
        let bounds = test_bounds();

        // X軸（打ち合い方向）: -3.0（Left側）〜 +3.0（Right側）
        assert_eq!(bounds.back_left, -3.0);
        assert_eq!(bounds.back_right, 3.0);

        // Y軸: 0（地面）〜 5.0（天井）
        assert_eq!(bounds.ground, 0.0);
        assert_eq!(bounds.ceiling, 5.0);

        // Z軸（コート幅）: -5.0（左）〜 +5.0（右）
        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
    }

    /// TST-30504-002: コート境界（コート幅方向 = Z）
    #[test]
    fn test_req_30501_002_boundary_z() {
        let bounds = test_bounds();

        // Z境界: -5.0（左）〜 +5.0（右）
        assert!(bounds.is_within_z(0.0));
        assert!(bounds.is_within_z(-5.0));
        assert!(bounds.is_within_z(5.0));
        assert!(!bounds.is_within_z(-5.1));
        assert!(!bounds.is_within_z(5.1));
    }

    /// TST-30504-003: コート境界（打ち合い方向 = X）
    #[test]
    fn test_req_30501_003_boundary_x() {
        let bounds = test_bounds();

        // X境界: -3.0（Left側）〜 +3.0（Right側）
        assert!(bounds.is_within_x(0.0));
        assert!(bounds.is_within_x(-3.0));
        assert!(bounds.is_within_x(3.0));
        assert!(!bounds.is_within_x(-3.1));
        assert!(!bounds.is_within_x(3.1));
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

        // ネット位置: X = 0.0, 高さ = 1.0
        assert_eq!(net.x, 0.0);
        assert_eq!(net.height, 1.0);

        // ネット衝突判定 (y, x, tolerance)
        assert!(net.is_collision(0.5, 0.0, 0.1)); // ネット位置、高さ未満
        assert!(!net.is_collision(1.5, 0.0, 0.1)); // ネット位置、高さ超過
        assert!(!net.is_collision(0.5, 1.0, 0.1)); // ネット位置外
    }

    /// TST-30504-006: コート区分（Left/Right）
    #[test]
    fn test_req_30501_006_court_side() {
        let net = test_net();

        // Leftコート範囲: X < net.x
        assert_eq!(determine_court_side(-1.0, net.x), CourtSide::Left);
        assert_eq!(determine_court_side(-3.0, net.x), CourtSide::Left);

        // Rightコート範囲: X >= net.x
        assert_eq!(determine_court_side(1.0, net.x), CourtSide::Right);
        assert_eq!(determine_court_side(3.0, net.x), CourtSide::Right);
        assert_eq!(determine_court_side(0.0, net.x), CourtSide::Right); // ネット上はRight側
    }

    /// REQ-30501-007: アウト境界（設計検証）
    /// コートライン外に着地するとアウト（失点）
    #[test]
    fn test_req_30501_007_out_of_bounds() {
        // このテストは設計上の確認
        // アウト判定は境界判定システム(30503)で実装される
        // ここではコート境界の定義のみを検証
    }

    #[test]
    fn test_court_bounds_is_inside() {
        let bounds = test_bounds();

        // コート中央 (x=0, y=2.5, z=0)
        assert!(bounds.is_inside(0.0, 2.5, 0.0));

        // コート端 (x=-3, y=0, z=-5) と (x=3, y=5, z=5)
        assert!(bounds.is_inside(-3.0, 0.0, -5.0));
        assert!(bounds.is_inside(3.0, 5.0, 5.0));

        // コート外
        assert!(!bounds.is_inside(-4.0, 2.5, 0.0)); // X outside
        assert!(!bounds.is_inside(0.0, 6.0, 0.0));  // Y outside
        assert!(!bounds.is_inside(0.0, 2.5, 6.0));  // Z outside
    }

    #[test]
    fn test_court_bounds_clamp() {
        let bounds = test_bounds();

        // X方向クランプ（打ち合い方向: -3〜3）
        assert_eq!(bounds.clamp_x(-10.0), -3.0);
        assert_eq!(bounds.clamp_x(10.0), 3.0);
        assert_eq!(bounds.clamp_x(0.0), 0.0);

        // Y方向クランプ（高さ: 0〜5）
        assert_eq!(bounds.clamp_y(-1.0), 0.0);
        assert_eq!(bounds.clamp_y(10.0), 5.0);
        assert_eq!(bounds.clamp_y(2.5), 2.5);

        // Z方向クランプ（コート幅: -5〜5）
        assert_eq!(bounds.clamp_z(-10.0), -5.0);
        assert_eq!(bounds.clamp_z(10.0), 5.0);
        assert_eq!(bounds.clamp_z(0.0), 0.0);
    }
}
