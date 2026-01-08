//! コートファクトリ: CourtConfigからCore層オブジェクトを生成
//! @spec 20001_layers.md#layer-4-systems
//!
//! このモジュールはSystems層に配置し、Resource層のCourtConfigを
//! Core層のCourtBounds/NetInfoに変換する役割を持つ。
//! これによりCore層のResource層への依存を解消する。
//!
//! ## 座標系
//! - X軸: 打ち合い方向（depth）
//! - Y軸: 高さ方向
//! - Z軸: コート幅方向（width）

use crate::core::court::{CourtBounds, NetInfo};
use crate::resource::CourtConfig;

/// CourtConfigからCourtBoundsを生成
/// @spec 30501_court_spec.md#req-30501-002
/// @spec 30501_court_spec.md#req-30501-003
/// @spec 30501_court_spec.md#req-30501-004
///
/// # 座標マッピング
/// - left/right (Z軸): -width/2 〜 +width/2（コート幅方向）
/// - back_1p/back_2p (X軸): -depth/2 〜 +depth/2（打ち合い方向）
#[inline]
pub fn create_court_bounds(config: &CourtConfig) -> CourtBounds {
    CourtBounds::new(
        -config.width / 2.0,  // left (Z方向、コート幅)
        config.width / 2.0,   // right (Z方向)
        -config.depth / 2.0,  // back_1p (X方向、1Pバックライン)
        config.depth / 2.0,   // back_2p (X方向、2Pバックライン)
        0.0,
        config.ceiling_height,
    )
}

/// CourtConfigからNetInfoを生成
/// @spec 30501_court_spec.md#req-30501-005
#[inline]
pub fn create_net_info(config: &CourtConfig) -> NetInfo {
    NetInfo::new(config.net_x, config.net_height)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CourtConfig {
        CourtConfig {
            width: 10.0,   // Z方向（コート幅）
            depth: 6.0,    // X方向（打ち合い方向）
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_x: 0.0,    // ネットのX座標
            service_box_depth: 1.5,
        }
    }

    #[test]
    fn test_create_court_bounds() {
        let config = test_config();
        let bounds = create_court_bounds(&config);

        // Z方向（コート幅）: -5 〜 +5
        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
        // X方向（打ち合い方向）: -3 〜 +3
        assert_eq!(bounds.back_1p, -3.0);
        assert_eq!(bounds.back_2p, 3.0);
        assert_eq!(bounds.ground, 0.0);
        assert_eq!(bounds.ceiling, 5.0);
    }

    #[test]
    fn test_create_net_info() {
        let config = test_config();
        let net = create_net_info(&config);

        assert_eq!(net.x, 0.0);
        assert_eq!(net.height, 1.0);
    }
}
