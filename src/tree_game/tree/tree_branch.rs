use na::{Vector2, Matrix2, Vector4, Rotation2};
use gg::rendering::{BezierQuadControl, BezierRect, BezierLogic};
use gg::geometry::bezier_2d::BezierQuad;
use gg::geometry::bezier_patch::BezierPatch;
use gg::debug::*;
use super::BranchId;
use std::collections::HashMap;

pub struct TreeBranch {
    id: BranchId,
    visual: VisualSpec,
    logical: LogicalSpec,
    connections: Vec<Connection>
}

impl TreeBranch {
    pub fn new(
        id: BranchId,
        pos: Vector2<f64>,
        branch_type: BranchType,
        vertical_fill: f64,
        horizontal_fill: f64
    ) -> TreeBranch {
        let logical_spec = match branch_type {
            BranchType::Trunk => LogicalSpec::new_trunk(vertical_fill, horizontal_fill),
            BranchType::BranchTop | BranchType::BranchBottom => LogicalSpec::new_branch(vertical_fill, horizontal_fill)
        };

        let visual_spec = VisualSpec::new(id.layer, pos, branch_type, vertical_fill, horizontal_fill);

        TreeBranch {
            id: id,
            logical: logical_spec,
            visual: visual_spec,
            connections: Vec::new()
        }
    }

    pub fn add_connection(first: &mut TreeBranch, f_bound: Boundary, second: &mut TreeBranch, s_bound: Boundary) {
        let f_connect = Connection {
            id: second.id,
            boundary: f_bound
        };
        let s_connect = Connection {
            id: first.id,
            boundary: s_bound
        };
        first.connections.push(f_connect);
        second.connections.push(s_connect);
    }

    pub fn get_visual(&self) -> VisualSpec {
        self.visual.clone()
    }

    pub fn get_logical(&self) -> LogicalSpec {
        self.logical.clone()
    }

    pub fn get_id(&self) -> BranchId {
        self.id.clone()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LogicalSpec {
    left_width: f64,
    right_width: f64,
    length_left: f64,
    length_right: f64
}

impl LogicalSpec {
    pub fn new_trunk(vert_fill: f64, hori_fill: f64) -> LogicalSpec {
        LogicalSpec {
            left_width: vert_fill,
            right_width: vert_fill,
            length_left: hori_fill / 2.0,
            length_right: hori_fill / 2.0
        }
    }

    pub fn new_branch(vert_fill: f64, hori_fill: f64) -> LogicalSpec {
        LogicalSpec {
            left_width: vert_fill / 2.0,
            right_width: vert_fill,
            length_left: hori_fill / 2.0,
            length_right: hori_fill * (1.0 - 1.0 / hori_fill) / 2.0 ,
        }
    }
}

impl From<LogicalSpec> for BezierLogic {
    fn from (spec: LogicalSpec) -> Self {
        BezierLogic {
            length_left: spec.length_left,
            length_right: spec.length_right,
            height_left: spec.left_width,
            height_right: spec.right_width
        }
    }
}

#[derive(Clone)]
pub struct VisualSpec {
    pub patch: BezierPatch,
    pub color: Vector4<f64>
}

impl VisualSpec {
    pub fn new(
        depth: usize,
        pos: Vector2<f64>,
        branch_type: BranchType,
        vertical_fill: f64,
        horizontal_fill: f64
    ) -> VisualSpec {
        //debug(&format!("depth: {:?}, pos: {:?}, branch_type: {:?}, vert_fill, {:?}, hori_fill: {:?}",
        //               depth,
        //               pos,
        //               branch_type,
        //               vertical_fill,
        //               horizontal_fill,
        //));
        let trunk_width = (1.0 - (1.0 - 2.0 * vertical_fill).powi((depth as i32) + 1)) / 2f64.powi((depth as i32) + 1);
        let trunk_width_next =  (1.0 - vertical_fill.powi((depth as i32) + 2)) / 2f64.powi((depth as i32) + 2);
        let trunk_length = horizontal_fill * (1.0 - horizontal_fill).powi(depth as i32) / 2.0;
        let vert_dir = Vector2::<f64>::new(0.0, 1.0);
        let width = match branch_type {
            BranchType::Trunk => trunk_width * 2.0,
            BranchType::BranchTop | BranchType::BranchBottom => trunk_width
        };
        let control = match branch_type {
            BranchType::Trunk => BezierQuad::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(trunk_length, 0.0),
                Vector2::new(trunk_length * 2.0, 0.0)
            ),
            BranchType::BranchTop => BezierQuad::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(trunk_length, trunk_width / 4.0),
                Vector2::new(trunk_length * 2.0, trunk_width / 2.0)
            ),
            BranchType::BranchBottom => BezierQuad::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(trunk_length, -trunk_width / 4.0),
                Vector2::new(trunk_length * 2.0, -trunk_width / 2.0)
            )
        };

        let patch = BezierPatch {
            control: control,
            vert_dir: vert_dir,
            width: width,
            pos: pos
        };

        VisualSpec {
            patch: patch,
            color: Vector4::new(0.1, 0.1, 1.0, 1.0),
        }
    }
}

impl From<VisualSpec> for BezierRect {
    fn from (spec: VisualSpec) -> Self {
        BezierRect {
            control: spec.patch.control.into(),
            vert_dir: spec.patch.vert_dir,
            width: spec.patch.width,
            pos: spec.patch.pos,
            color: spec.color
        }
    }
}

pub struct Connection {
    id: BranchId,
    boundary: Boundary
}

pub enum Boundary {
    Left(f64, f64),
    Right(f64, f64)
}

#[derive(Copy, Clone, Debug)]
pub enum BranchType {
    Trunk,
    BranchTop,
    BranchBottom
}
