use na::Vector2;
use gg::rendering::BezierLogic;
use gg::geometry::interpolate;
use gg::geometry::ConPoly;

#[derive(Copy, Clone, Debug)]
pub struct LogicalSpec {
    pub left_width: f64,
    pub right_width: f64,
    pub length: f64,
}

impl LogicalSpec {
    pub fn new_logical_rect(width: f64, length: f64) -> LogicalSpec {
        LogicalSpec {
            left_width: width,
            right_width: width,
            length: length,
        }
    }

    pub fn new_logical_trapezoid(left_width: f64, right_width: f64, left_length: f64) -> LogicalSpec {
        LogicalSpec {
            left_width: left_width,
            right_width: right_width,
            length: left_length,
        }
    }

    pub fn shift_along_tracking_line(&self, point: Vector2<f64>, shift: f64) -> Vector2<f64> {
        let reg_horizontal = point.x / self.length;
        let reg_vertical = point.y / interpolate(self.left_width, self.right_width, reg_horizontal);
        let shifted_reg_horizontal = shift / self.length + reg_horizontal;
        let new_vertical = reg_vertical * interpolate(self.left_width, self.right_width, shifted_reg_horizontal);
        Vector2::new(point.x + shift, new_vertical)
    }

    fn get_logical_con_poly(&self) -> ConPoly {
        let bottom_left = Vector2::new(0.0, - self.left_width / 2.0);
        let bottom_right = Vector2::new(self.length, - self.right_width / 2.0);
        let top_right = Vector2::new(self.length, self.right_width / 2.0);
        let top_left = Vector2::new(0.0, self.left_width / 2.0);

        let corners = vec![
            bottom_left,
            bottom_right,
            top_right,
            top_left,
        ];

        ConPoly::new(corners)
    }
}

impl From<LogicalSpec> for BezierLogic {
    fn from (spec: LogicalSpec) -> Self {
        BezierLogic {
            length: spec.length,
            width_left: spec.left_width,
            width_right: spec.right_width
        }
    }
}
