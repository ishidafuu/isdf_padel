//! イベント定義
//! @spec 20005_event_system.md
//! @spec 30503_boundary_behavior.md
//! @spec 30201_movement_spec.md
//! @spec 30202_jump_spec.md
//! @spec 30401_trajectory_spec.md
//! @spec 30601_shot_input_spec.md
//! @spec 30602_shot_direction_spec.md

use bevy::prelude::*;

/// プレイヤー移動イベント
/// @spec 30201_movement_spec.md#req-30201-006
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerMoveEvent {
    /// プレイヤーID
    #[allow(dead_code)]
    pub player_id: u8,
    /// 新しい位置
    #[allow(dead_code)]
    pub position: Vec3,
    /// 移動速度ベクトル
    #[allow(dead_code)]
    pub velocity: Vec3,
}

/// プレイヤージャンプイベント
/// @spec 30202_jump_spec.md#req-30202-007
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerJumpEvent {
    /// プレイヤーID
    #[allow(dead_code)]
    pub player_id: u8,
    /// ジャンプ初速度
    #[allow(dead_code)]
    pub jump_velocity: f32,
}

/// プレイヤー着地イベント
/// @spec 30202_jump_spec.md#req-30202-008
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerLandEvent {
    /// プレイヤーID
    #[allow(dead_code)]
    pub player_id: u8,
    /// 着地位置
    #[allow(dead_code)]
    pub land_position: Vec3,
}

/// ボールヒットイベント（プレイヤーにボールが当たった）
/// @spec 30203_knockback_spec.md#req-30203-001
#[derive(Event, Message, Debug, Clone)]
pub struct BallHitEvent {
    /// ボールのEntity
    #[allow(dead_code)]
    pub ball_id: Entity,
    /// 被弾したプレイヤーのEntity
    pub target_id: Entity,
    /// ボールの速度
    pub ball_velocity: Vec3,
    /// 衝突位置
    pub hit_point: Vec3,
}

/// プレイヤーふっとばしイベント
/// @spec 30203_knockback_spec.md#req-30203-007
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerKnockbackEvent {
    /// プレイヤーID
    #[allow(dead_code)]
    pub player_id: u8,
    /// ふっとばし速度ベクトル
    #[allow(dead_code)]
    pub knockback_velocity: Vec3,
    /// ふっとばし時間（秒）
    #[allow(dead_code)]
    pub duration: f32,
    /// 無敵時間（秒）
    #[allow(dead_code)]
    pub invincibility_time: f32,
}

/// 壁の種類
/// @spec 30503_boundary_behavior.md#beh-30503-004
/// 新座標系: LeftWall/RightWall = Z方向（コート幅）, BackWall = X方向（打ち合い方向）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallType {
    /// 左壁（Z = -Court.Width/2、コート幅方向）
    LeftWall,
    /// 右壁（Z = +Court.Width/2、コート幅方向）
    RightWall,
    /// 後壁（Leftコート側、X = -Court.Depth/2、打ち合い方向）
    BackWallLeft,
    /// 後壁（Rightコート側、X = +Court.Depth/2、打ち合い方向）
    BackWallRight,
    /// 天井（Y = Court.CeilingHeight）
    Ceiling,
}

impl WallType {
    /// 壁の法線ベクトルを返す
    /// @spec 30503_boundary_behavior.md#beh-30503-004
    /// 新座標系: LeftWall/RightWall = Z方向, BackWall = X方向
    #[allow(dead_code)]
    #[inline]
    pub fn normal(&self) -> Vec3 {
        match self {
            WallType::LeftWall => Vec3::Z,          // +Z方向（Z負側の壁）
            WallType::RightWall => Vec3::NEG_Z,     // -Z方向（Z正側の壁）
            WallType::BackWallLeft => Vec3::X,      // +X方向（X負側の壁、Leftバックライン）
            WallType::BackWallRight => Vec3::NEG_X, // -X方向（X正側の壁、Rightバックライン）
            WallType::Ceiling => Vec3::NEG_Y,       // -Y方向（天井）
        }
    }

    /// 左右壁かどうか
    #[allow(dead_code)]
    #[inline]
    pub fn is_side_wall(&self) -> bool {
        matches!(self, WallType::LeftWall | WallType::RightWall)
    }

    /// 前後壁かどうか
    #[allow(dead_code)]
    #[inline]
    pub fn is_back_wall(&self) -> bool {
        matches!(self, WallType::BackWallLeft | WallType::BackWallRight)
    }
}

/// 壁反射イベント
/// @spec 30503_boundary_behavior.md#beh-30503-004
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Message, Debug, Clone)]
pub struct WallReflectionEvent {
    /// 反射したボールのEntity
    #[allow(dead_code)]
    pub ball: Entity,
    /// 反射した壁の種類
    #[allow(dead_code)]
    pub wall_type: WallType,
    /// 接触点の座標
    #[allow(dead_code)]
    pub contact_point: Vec3,
    /// 反射前の速度
    #[allow(dead_code)]
    pub incident_velocity: Vec3,
    /// 反射後の速度
    #[allow(dead_code)]
    pub reflected_velocity: Vec3,
}

/// ネット接触イベント
/// @spec 30503_boundary_behavior.md#beh-30503-005
#[derive(Message, Debug, Clone)]
pub struct NetHitEvent {
    /// ネットに当たったボールのEntity
    pub ball: Entity,
    /// 接触点の座標
    #[allow(dead_code)]
    pub contact_point: Vec3,
}

/// 地面バウンドイベント
/// @spec 30503_boundary_behavior.md#beh-30503-006
#[derive(Message, Debug, Clone)]
pub struct GroundBounceEvent {
    /// バウンドしたボールのEntity
    pub ball: Entity,
    /// バウンド位置
    #[allow(dead_code)]
    pub bounce_point: Vec3,
    /// バウンドしたコート側
    pub court_side: super::court::CourtSide,
}

/// ボールアウトオブバウンズイベント
/// @spec 30401_trajectory_spec.md#req-30401-006
#[derive(Event, Message, Debug, Clone)]
pub struct BallOutOfBoundsEvent {
    /// アウトになったボールのEntity
    #[allow(dead_code)]
    pub ball: Entity,
    /// 最終位置
    pub final_position: Vec3,
}

/// ショットイベント
/// @spec 30601_shot_input_spec.md#req-30601-001
/// @spec 30601_shot_input_spec.md#req-30601-006
/// @spec 30602_shot_direction_spec.md#req-30602-030
#[derive(Event, Message, Debug, Clone)]
pub struct ShotEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// プレイヤーの所属コートサイド（ショット方向の決定に使用）
    pub court_side: super::court::CourtSide,
    /// 入力方向（十字キー、正規化済み）
    pub direction: Vec2,
    /// ジャンプ中の高さ（Position.Y）
    pub jump_height: f32,
    /// サーブかどうか
    /// @spec 30602_shot_direction_spec.md#req-30602-030
    pub is_serve: bool,
    /// 打点位置（サーブ時のみ使用）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-053
    pub hit_position: Option<Vec3>,
    /// サーブトス上向き初速度（サーブ時のみ使用）
    pub serve_toss_velocity_y: Option<f32>,
}

/// ショット実行完了イベント
/// @spec 30602_shot_direction_spec.md#req-30602-007
#[derive(Event, Message, Debug, Clone)]
pub struct ShotExecutedEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// 打球速度ベクトル
    #[allow(dead_code)]
    pub shot_velocity: Vec3,
    /// ジャンプショットかどうか
    #[allow(dead_code)]
    pub is_jump_shot: bool,
}

/// ショット属性計算完了イベント（トレース用）
/// @spec 77200_telemetry_spec.md#req-77200-001
#[derive(Event, Message, Debug, Clone)]
pub struct ShotAttributesCalculatedEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// 入力方式
    pub input_mode: String,
    /// 打点高さ
    pub hit_height: f32,
    /// バウンド経過時間
    pub bounce_elapsed: Option<f32>,
    /// 入り方内積
    pub approach_dot: f32,
    /// ボール距離
    pub ball_distance: f32,
    /// 高さ係数 (power, stability, angle)
    pub height_factors: (f32, f32, f32),
    /// タイミング係数 (power, stability, angle)
    pub timing_factors: (f32, f32, f32),
    /// 入り方係数 (power, angle)
    pub approach_factors: (f32, f32),
    /// 距離係数 (power, stability, accuracy)
    pub distance_factors: (f32, f32, f32),
    /// 最終威力
    pub final_power: f32,
    /// 最終安定性
    pub final_stability: f32,
    /// 最終角度
    pub final_angle: f32,
    /// 最終スピン
    pub final_spin: f32,
    /// 最終精度
    pub final_accuracy: f32,
}

/// ラリー終了イベント（ポイント獲得のトリガー）
/// @spec 30701_point_spec.md#req-30701-002
#[derive(Event, Message, Debug, Clone)]
pub struct RallyEndEvent {
    /// ポイントを獲得したプレイヤー側
    pub winner: super::court::CourtSide,
    /// 終了理由
    pub reason: RallyEndReason,
}

/// ラリー終了理由
/// @spec 30701_point_spec.md
/// @spec 30103_point_end_spec.md
/// @spec 30902_fault_spec.md
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RallyEndReason {
    /// ツーバウンド
    /// @spec 30103_point_end_spec.md#req-30103-001
    DoubleBounce,
    /// アウト（パデルでは通常発生しない、安全弁）
    Out,
    /// ネットタッチ（ボールがネットに当たり相手コートに届かなかった）
    /// @spec 30103_point_end_spec.md#req-30103-002
    NetFault,
    /// 自コート打球（打った打球が自コートに落ちた）
    /// @spec 30103_point_end_spec.md#req-30103-003
    OwnCourtHit,
    /// ダブルフォルト（サーブを2回連続でミス）
    /// @spec 30902_fault_spec.md#req-30902-002
    DoubleFault,
}

/// ポイント獲得イベント
/// @spec 30701_point_spec.md#req-30701-002
#[derive(Event, Message, Debug, Clone)]
pub struct PointScoredEvent {
    /// ポイントを獲得したプレイヤー側
    pub scorer: super::court::CourtSide,
    /// 獲得後のポイント値（表示用）
    pub new_point_value: u32,
}

/// ゲーム勝利イベント
/// @spec 30701_point_spec.md#req-30701-003
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct GameWonEvent {
    /// 勝利したプレイヤー側
    #[allow(dead_code)]
    pub winner: super::court::CourtSide,
    /// 勝利後のゲーム数
    #[allow(dead_code)]
    pub games_won: u32,
}

/// セット勝利イベント
/// @spec 30701_point_spec.md
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct SetWonEvent {
    /// 勝利したプレイヤー側
    #[allow(dead_code)]
    pub winner: super::court::CourtSide,
    /// 勝利後のセット数
    #[allow(dead_code)]
    pub sets_won: u32,
}

/// マッチ勝利イベント（試合終了イベント）
/// @spec 30701_point_spec.md
/// @spec 30101_flow_spec.md#req-30101-005
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct MatchWonEvent {
    /// 勝利したプレイヤー側
    #[allow(dead_code)]
    pub winner: super::court::CourtSide,
}

/// 試合開始イベント
/// @spec 30101_flow_spec.md#req-30101-005
/// NOTE: デバッグ・ログ出力用のイベント。読み取りハンドラは将来実装予定。
#[derive(Event, Message, Debug, Clone)]
pub struct MatchStartEvent {
    /// サーブ権を持つプレイヤー側
    #[allow(dead_code)]
    pub first_server: super::court::CourtSide,
}

/// フォールトイベント
/// @spec 30902_fault_spec.md#req-30902-001
/// @spec 30902_fault_spec.md#req-30902-003
#[derive(Event, Message, Debug, Clone)]
pub struct FaultEvent {
    /// サーバー側
    pub server: super::court::CourtSide,
    /// 現在のフォールトカウント（このフォールト後の値）
    #[allow(dead_code)]
    pub fault_count: u32,
    /// フォールト理由
    #[allow(dead_code)]
    pub reason: FaultReason,
}

/// フォールト理由
/// @spec 30902_fault_spec.md#req-30902-001
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultReason {
    /// サービスボックス外への着地
    OutOfServiceBox,
    /// ネットフォールト（サーブがネットに当たりサービスボックスに入らなかった）
    /// NOTE: 仕様書で定義済み、将来実装予定
    #[allow(dead_code)]
    NetFault,
}

/// ダブルフォールトイベント
/// @spec 30902_fault_spec.md#req-30902-002
#[derive(Event, Message, Debug, Clone)]
pub struct DoubleFaultEvent {
    /// サーバー側（失点するプレイヤー）
    pub server: super::court::CourtSide,
}
