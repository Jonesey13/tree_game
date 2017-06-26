use na::Vector2;
use gg::geometry::{Interval, Line, line_line_intersect_2d, DualSoln};
//use gg::debug::*;
use super::{BranchId, Connection, Boundary};
pub mod logical;
pub mod visual;
pub use self::logical::*;
pub use self::visual::*;

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
            BranchType::Trunk => LogicalSpec::new_logical_rect(1.0, 0.5),
            BranchType::BranchTop | BranchType::BranchBottom => LogicalSpec::new_logical_trapezoid(0.5, 1.0, 1.0)
        };

        let visual_spec = VisualSpec::new(id.layer, pos, branch_type, vertical_fill, horizontal_fill);

        TreeBranch {
            id: id,
            logical: logical_spec,
            visual: visual_spec,
            connections: Vec::new()
        }
    }

    pub fn add_connection(&mut self, f_bound: Boundary, second: &mut TreeBranch, s_bound: Boundary) {
        let f_connect = Connection::new(second.id, f_bound);
        let s_connect = Connection::new(self.id, s_bound);
        self.connections.push(f_connect);
        second.connections.push(s_connect);
    }

    pub fn get_visual(&self) -> VisualSpec {
        self.visual.clone()
    }

    pub fn get_logical(&self) -> LogicalSpec {
        self.logical.clone()
    }

    pub fn get_id(&self) -> BranchId {
        self.id
    }

    pub fn get_connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn get_left_connections(&self) -> Vec<&Connection> {
        self.get_connections().iter().filter_map(|c| {
            match c.get_boundary() {
                Boundary::Left(_, _) => Some(c),
                _ => None
            }
        }).collect()
    }

    pub fn get_right_connections(&self) -> Vec<&Connection> {
        self.get_connections().iter().filter_map(|c| {
            match c.get_boundary() {
                Boundary::Right(_, _) => Some(c),
                _ => None
            }
        }).collect()
    }

    pub fn get_connection(&self, pos: f64, side: BranchSide) -> Option<&Connection> {
        let possible_connections = match side {
            BranchSide::Left => self.get_left_connections(),
            BranchSide::Right => self.get_right_connections()
        };

        let valid_connections: Vec<&Connection> = possible_connections
            .into_iter()
            .filter( |c| { c.get_interval().contains(pos) })
            .collect();

        match valid_connections.len() {
            0 => None,
            1 => Some(valid_connections.first().unwrap()),
            _ => panic!("Overlapping Connections on Branch with Id: {:?}", self.id)
        }
    }

    pub fn get_logical_boundary_interval(&self, boundary: Boundary) -> Interval {
        let scaling = match boundary {
            Boundary::Left(_, _) => self.get_logical().left_width / 2.0,
            Boundary::Right(_, _) => self.get_logical().right_width / 2.0
        };
        boundary.get_interval() * scaling
    }

    pub fn get_logical_boundary_line(&self, boundary: Boundary) -> Line {
        let scaling = match boundary {
            Boundary::Left(_, _) => self.get_logical().left_width / 2.0,
            Boundary::Right(_, _) => self.get_logical().right_width / 2.0
        };
        let x_pos = match boundary {
            Boundary::Left(_, _) => 0.0,
            Boundary::Right(_, _) => self.get_logical().length
        };
        let boundary_interval = boundary.get_interval() * scaling;
        Line::new(Vector2::new(x_pos, boundary_interval.get_start()),
                  Vector2::new(x_pos, boundary_interval.get_end()))
    }

    pub fn get_new_logical_position(&self, pos: Vector2<f64>, change_vec: Vector2<f64>) -> Vector2<f64>{
        let mut new_logical_position = self.get_logical().shift_along_tracking_line(pos, change_vec.x);
        new_logical_position += Vector2::new(0.0, change_vec.y);
        new_logical_position
    }

    fn get_single_boundary_intersect(&self, line: Line, connection: &Connection) -> Option<ConnectionIntersect> {
        let boundary_line = self.get_logical_boundary_line(connection.get_boundary());

        let line_line_intersect = line_line_intersect_2d(&line, &boundary_line);

        let zero_one = Interval::new(0.0, 1.0);
        
        let intersect_time = match line_line_intersect {
            DualSoln::None => return None,
            DualSoln::Two(first, second) => {
                if zero_one.contains(first) && zero_one.contains(second) {
                    Some(first)
                }
                else {
                    None
                }
            }
        };

        match intersect_time {
            Some(time) => Some(
                ConnectionIntersect{
                    connection: connection.clone(),
                    overlap: (line.get_point(1.0) - line.get_point(time)).x.abs()
                }
            ),
            None => None
        }
    }

    pub fn get_boundary_intersect(&self, line: Line) -> Option<ConnectionIntersect> {
        for connection in self.get_connections() {
            if let Some(connection_intersect) = self.get_single_boundary_intersect(line, connection) {
                return Some(connection_intersect);
            }
        }
        None
    }
}

pub enum BranchSide {
    Left,
    Right
}

#[derive(Copy, Clone, Debug)]
pub enum BranchType {
    Trunk,
    BranchTop,
    BranchBottom
}

pub struct ConnectionIntersect {
    pub connection: Connection,
    pub overlap: f64
}
