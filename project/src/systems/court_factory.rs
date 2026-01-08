//! コートファクトリ: CourtConfigからCore層オブジェクトを生成
//! @spec 20001_layers.md#layer-4-systems
//!
//! このモジュールはSystems層に配置し、Resource層のCourtConfigを
//! Core層のCourtBounds/NetInfo/Courtに変換する役割を持つ。
//! これによりCore層のResource層への依存を解消する。

use crate::core::court::{Court, CourtBounds, NetInfo};
use crate::resource::CourtConfig;

/// CourtConfigからCourtBoundsを生成
/// @spec 30501_court_spec.md#req-30501-002
/// @spec 30501_court_spec.md#req-30501-003
/// @spec 30501_court_spec.md#req-30501-004
#[inline]
pub fn create_court_bounds(config: &CourtConfig) -> CourtBounds {
    CourtBounds::new(
        -config.width / 2.0,
        config.width / 2.0,
        -config.depth / 2.0,
        config.depth / 2.0,
        0.0,
        config.ceiling_height,
    )
}

/// CourtConfigからNetInfoを生成
/// @spec 30501_court_spec.md#req-30501-005
#[inline]
pub fn create_net_info(config: &CourtConfig) -> NetInfo {
    NetInfo::new(config.net_z, config.net_height)
}

/// CourtConfigからCourtを生成
/// @spec 30501_court_spec.md
#[inline]
pub fn create_court(config: &CourtConfig) -> Court {
    Court::new(create_court_bounds(config), create_net_info(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CourtConfig {
        CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
            service_box_depth: 1.5,
        }
    }

    #[test]
    fn test_create_court_bounds() {
        let config = test_config();
        let bounds = create_court_bounds(&config);

        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
        assert_eq!(bounds.back_1p, -3.0);
        assert_eq!(bounds.back_2p, 3.0);
        assert_eq!(bounds.ground, 0.0);
        assert_eq!(bounds.ceiling, 5.0);
    }

    #[test]
    fn test_create_net_info() {
        let config = test_config();
        let net = create_net_info(&config);

        assert_eq!(net.z, 0.0);
        assert_eq!(net.height, 1.0);
    }

    #[test]
    fn test_create_court() {
        let config = test_config();
        let court = create_court(&config);

        assert_eq!(court.bounds.left, -5.0);
        assert_eq!(court.net.height, 1.0);
    }
}
