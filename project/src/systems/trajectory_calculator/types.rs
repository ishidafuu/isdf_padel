//! 弾道計算の型定義
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::ServeSide;

/// 弾道計算結果
/// @spec 30605_trajectory_calculation_spec.md
#[derive(Debug, Clone)]
pub struct TrajectoryResult {
    /// 発射角度（度）
    pub launch_angle: f32,
    /// 最終初速
    pub final_speed: f32,
    /// 発射方向ベクトル（正規化済み）
    pub direction: Vec3,
    /// 着地予定地点
    pub landing_position: Vec3,
}

/// 弾道計算コンテキスト
/// 計算に必要な入力パラメータをまとめる
#[derive(Debug, Clone)]
pub struct TrajectoryContext {
    /// 入力方向（X=左右, Y=前後）
    pub input: Vec2,
    /// コートサイド
    pub court_side: CourtSide,
    /// ボールの現在位置
    pub ball_position: Vec3,
    /// スピン値（-1.0〜+1.0）
    pub spin: f32,
    /// 基準初速（ショット属性から）
    pub base_speed: f32,
    /// 精度（ショット属性から）
    pub accuracy: f32,
}

/// サーブ用弾道計算コンテキスト
#[derive(Debug, Clone)]
pub struct ServeTrajectoryContext {
    /// 入力方向（Y=コース左右。Xはサーブでは未使用）
    pub input: Vec2,
    /// サーバーのコートサイド
    pub server: CourtSide,
    /// サーブサイド（デュース/アド）
    pub serve_side: ServeSide,
    /// 打点位置（トスボールの位置）
    pub hit_position: Vec3,
    /// 基準初速
    pub base_speed: f32,
    /// サーブトスの上向き初速度
    pub toss_velocity_y: f32,
}
