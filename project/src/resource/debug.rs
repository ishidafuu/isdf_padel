//! デバッグ用リソース
//! 弾道デバッグマーカー機能用

use bevy::prelude::*;

use crate::core::court::CourtSide;

/// 最後のショットのデバッグ情報
/// Xボタンでマーク時にログ出力される
#[derive(Resource, Default)]
pub struct LastShotDebugInfo {
    /// 有効なデータがあるか（バウンド後はfalseになる）
    pub is_valid: bool,
    /// プレイヤーID
    pub player_id: u8,
    /// ショット時のボール位置
    pub ball_pos: Vec3,
    /// 入力方向
    pub input: Vec2,
    /// コートサイド
    pub court_side: Option<CourtSide>,
    /// パワー
    pub power: f32,
    /// スピン
    pub spin: f32,
    /// 精度
    pub accuracy: f32,
    /// 着地予定地点
    pub landing: Vec3,
    /// 打ち上げ角度（度）
    pub launch_angle: f32,
    /// 最終速度
    pub final_speed: f32,
    /// 速度ベクトル
    pub velocity: Vec3,
    /// 判別式
    pub discriminant: f32,
    /// 有効重力
    pub g_eff: f32,
}
