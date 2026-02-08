   Compiling padel_game v0.1.0 (/Users/ishidafuu/Documents/repository/isdf_padel/project)
warning: fields `x`, `y`, and `z` are never read
  --> src/bin/trace_narrator/types.rs:27:9
   |
26 | pub struct Vec3 {
   |            ---- fields in this struct
27 |     pub x: f32,
   |         ^
28 |     pub y: f32,
   |         ^
29 |     pub z: f32,
   |         ^
   |
   = note: `Vec3` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: enum `Severity` is never used
  --> src/bin/trace_narrator/types.rs:48:10
   |
48 | pub enum Severity {
   |          ^^^^^^^^

warning: fields `player` and `shot_type` are never read
  --> src/bin/trace_narrator/types.rs:59:9
   |
58 |     BallHit {
   |     ------- fields in this variant
59 |         player: u8,
   |         ^^^^^^
60 |         shot_type: String,
   |         ^^^^^^^^^
   |
   = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `position` and `court_side` are never read
  --> src/bin/trace_narrator/types.rs:64:9
   |
63 |     Bounce {
   |     ------ fields in this variant
64 |         position: Vec3,
   |         ^^^^^^^^
65 |         court_side: CourtSide,
   |         ^^^^^^^^^^
   |
   = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `position` and `wall_type` are never read
  --> src/bin/trace_narrator/types.rs:69:9
   |
68 |     WallReflect {
   |     ----------- fields in this variant
69 |         position: Vec3,
   |         ^^^^^^^^
70 |         wall_type: String,
   |         ^^^^^^^^^
   |
   = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: field `fault_type` is never read
  --> src/bin/trace_narrator/types.rs:79:9
   |
78 |     Fault {
   |     ----- field in this variant
79 |         fault_type: String,
   |         ^^^^^^^^^^
   |
   = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `from` and `to` are never read
  --> src/bin/trace_narrator/types.rs:83:9
   |
82 |     StateChange {
   |     ----------- fields in this variant
83 |         from: String,
   |         ^^^^
84 |         to: String,
   |         ^^
   |
   = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: multiple fields are never read
   --> src/bin/trace_narrator/types.rs:89:9
    |
 87 |     ShotAttributesCalculated {
    |     ------------------------ fields in this variant
 88 |         player_id: u8,
 89 |         input_mode: String,
    |         ^^^^^^^^^^
 90 |         hit_height: f32,
    |         ^^^^^^^^^^
 91 |         bounce_elapsed: Option<f32>,
    |         ^^^^^^^^^^^^^^
 92 |         approach_dot: f32,
    |         ^^^^^^^^^^^^
 93 |         ball_distance: f32,
    |         ^^^^^^^^^^^^^
 94 |         height_factors: [f32; 3],
    |         ^^^^^^^^^^^^^^
 95 |         timing_factors: [f32; 3],
    |         ^^^^^^^^^^^^^^
 96 |         approach_factors: [f32; 2],
    |         ^^^^^^^^^^^^^^^^
 97 |         distance_factors: [f32; 3],
    |         ^^^^^^^^^^^^^^^^
...
100 |         final_angle: f32,
    |         ^^^^^^^^^^^
    |
    = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: multiple fields are never read
   --> src/bin/trace_narrator/types.rs:106:9
    |
105 |     AiMovementDecision {
    |     ------------------ fields in this variant
106 |         player_id: u8,
    |         ^^^^^^^^^
107 |         movement_state: String,
    |         ^^^^^^^^^^^^^^
108 |         ball_coming_to_me: bool,
    |         ^^^^^^^^^^^^^^^^^
109 |         reaction_timer: f32,
    |         ^^^^^^^^^^^^^^
110 |         landing_time: Option<f32>,
    |         ^^^^^^^^^^^^
111 |         landing_position: Option<Vec3>,
    |         ^^^^^^^^^^^^^^^^
112 |         trajectory_line_z: f32,
    |         ^^^^^^^^^^^^^^^^^
113 |         arrival_distance: f32,
    |         ^^^^^^^^^^^^^^^^
114 |         target_position: Vec3,
    |         ^^^^^^^^^^^^^^^
    |
    = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: fields `position` and `velocity` are never read
   --> src/bin/trace_narrator/types.rs:119:9
    |
117 |     PhysicsAnomaly {
    |     -------------- fields in this variant
118 |         anomaly_type: String,
119 |         position: Vec3,
    |         ^^^^^^^^
120 |         velocity: Vec3,
    |         ^^^^^^^^
    |
    = note: `GameEvent` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: methods `type_name`, `is_point`, and `is_anomaly` are never used
   --> src/bin/trace_narrator/types.rs:129:12
    |
127 | impl GameEvent {
    | -------------- methods in this implementation
128 |     /// イベント種別名を取得
129 |     pub fn type_name(&self) -> &'static str {
    |            ^^^^^^^^^
...
144 |     pub fn is_point(&self) -> bool {
    |            ^^^^^^^^
...
149 |     pub fn is_anomaly(&self) -> bool {
    |            ^^^^^^^^^^

warning: fields `entity_type`, `position`, and `velocity` are never read
   --> src/bin/trace_narrator/types.rs:159:9
    |
156 | pub struct EntityTrace {
    |            ----------- fields in this struct
...
159 |     pub entity_type: EntityType,
    |         ^^^^^^^^^^^
160 |     /// 位置
161 |     pub position: Vec3,
    |         ^^^^^^^^
162 |     /// 速度
163 |     pub velocity: Vec3,
    |         ^^^^^^^^
    |
    = note: `EntityTrace` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: field `entities` is never read
   --> src/bin/trace_narrator/types.rs:175:9
    |
168 | pub struct FrameTrace {
    |            ---------- field in this struct
...
175 |     pub entities: Vec<EntityTrace>,
    |         ^^^^^^^^
    |
    = note: `FrameTrace` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: variant `UnknownFormat` is never constructed
  --> src/bin/trace_narrator/parser.rs:22:5
   |
14 | pub enum ParseError {
   |          ---------- variant in this enum
...
22 |     UnknownFormat,
   |     ^^^^^^^^^^^^^
   |
   = note: `ParseError` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: fields `expected` and `actual` are never read
  --> src/bin/trace_narrator/analyzer.rs:38:9
   |
30 | pub struct Anomaly {
   |            ------- fields in this struct
...
38 |     pub expected: Option<f32>,
   |         ^^^^^^^^
39 |     /// 実際の値
40 |     pub actual: Option<f32>,
   |         ^^^^^^
   |
   = note: `Anomaly` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: field `include_physics` is never read
  --> src/bin/trace_narrator/formatter.rs:40:9
   |
36 | pub struct FormatOptions {
   |            ------------- field in this struct
...
40 |     pub include_physics: bool,
   |         ^^^^^^^^^^^^^^^
   |
   = note: `FormatOptions` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: `padel_game` (bin "trace_narrator") generated 16 warnings
    Finished `dev` profile [optimized + debuginfo] target(s) in 9.69s
     Running `target/debug/trace_narrator qa_reports/2026-01-19_072511/trace.jsonl`
# Match Report

- **Duration**: 0m 59s
- **Frame Range**: 10 - 3600
- **Total Rallies**: 8
- **Points**: P1: 3, P2: 5

## Summary

| Metric | P1 | P2 |
|--------|----|----|
| Total Shots | 0 | 0 |
| Avg Power | 0.00 | 0.00 |
| Avg Accuracy | 0.00 | 0.00 |


## Rallies

### Rally 1 (Frame 0-350)

**Result**: P2 wins (DoubleFault)
**Duration**: 5.83s
**Shots**: 0

- Bounces: 2, Wall reflects: 0

### Rally 2 (Frame 350-690)

**Result**: P2 wins (DoubleFault)
**Duration**: 5.67s
**Shots**: 0

- Bounces: 2, Wall reflects: 0

### Rally 3 (Frame 690-1170)

**Result**: P1 wins (Out)
**Duration**: 8.00s
**Shots**: 0

- Bounces: 4, Wall reflects: 0

### Rally 4 (Frame 1170-1550)

**Result**: P2 wins (NetFault)
**Duration**: 6.33s
**Shots**: 0

- Bounces: 2, Wall reflects: 2

### Rally 5 (Frame 1550-2170)

**Result**: P2 wins (Out)
**Duration**: 10.33s
**Shots**: 0

- Bounces: 4, Wall reflects: 1

### Rally 6 (Frame 2170-2520)

**Result**: P1 wins (DoubleFault)
**Duration**: 5.83s
**Shots**: 0

- Bounces: 3, Wall reflects: 1

### Rally 7 (Frame 2520-2920)

**Result**: P1 wins (NetFault)
**Duration**: 6.67s
**Shots**: 0

- Bounces: 2, Wall reflects: 0

### Rally 8 (Frame 2920-3360)

**Result**: P2 wins (Out)
**Duration**: 7.33s
**Shots**: 0

- Bounces: 4, Wall reflects: 0

