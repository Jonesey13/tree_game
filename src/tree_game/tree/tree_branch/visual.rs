use na::{Vector2, Vector4};
use gg::geometry::{BezierPatch, BezierQuad};
use gg::rendering::BezierRect;
use super::BranchType;

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
