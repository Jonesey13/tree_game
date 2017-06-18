use super::position::Position;
use rendering::BezierSubrect;
use super::tree::{TreeBranch, Tree, BranchRect, BranchObject, ConnectionIntersect, Boundary};
use super::movable::Movable;
use na::{Vector2, Vector4};
use gg::geometry::Line;
use gg::rendering::Renderable;
use gg::debug::*;

lazy_static! {
    static ref PLAYER_COLOR: Vector4<f64> = Vector4::new(1.0, 1.0, 1.0, 1.0);
}

pub struct Player {
    pos: Position,
    length: f64,
    height: f64
}

impl Player {
    pub fn new(pos: Position, length: f64, height: f64) -> Player {
        Player {
            pos,
            length,
            height,
        }
    }

    pub fn get_branch_objects(&self, tree: &Tree) -> Vec<Box<BranchObject>> {
        let current_branch = tree.get_branch(self.pos.get_branch_id());

        let mut output: Vec<Box<BranchObject>> = Vec::new();

        let current_branch_pos = self.pos.get_branch_position();
        
        let centre_part = BranchRect {
            branch_id: self.pos.get_branch_id(),
            length: self.length,
            height: self.height,
            pos: current_branch_pos,
            color: *PLAYER_COLOR
        };

        output.push(Box::new(centre_part));

        
        let center_right_line = Line::new(
            current_branch_pos,
            current_branch_pos + Vector2::new(self.length / 2.0, 0.0)
        );

        if let Some(intersect) = current_branch.get_boundary_intersect(center_right_line) {
            let right_part = self.get_side_object(intersect, tree);

            output.push(Box::new(right_part));
        }
        

        let center_left_line = Line::new(
            self.pos.get_branch_position(),
            self.pos.get_branch_position() - Vector2::new(self.length / 2.0, 0.0)
        );

        if let Some(intersect) = current_branch.get_boundary_intersect(center_left_line) {
            let left_part = self.get_side_object(intersect, tree);

            output.push(Box::new(left_part));
        }
        
        output
    }

    fn get_side_object(&self, intersect: ConnectionIntersect, tree: &Tree) -> BranchRect {
        let current_branch = tree.get_branch(self.pos.get_branch_id());
        let current_branch_pos = self.pos.get_branch_position();
        
        let side_length = intersect.overlap;
        let current_branch_interval = current_branch.get_logical_boundary_interval(intersect.connection.get_boundary());
        
        let side_branch = tree.get_branch(intersect.connection.get_branch_id());
        let side_connection = tree.get_matching_connection(current_branch.get_id(), &intersect.connection);
        let side_branch_interval = side_branch.get_logical_boundary_interval(side_connection.get_boundary());
        
        let new_y_pos = current_branch_interval.fit_point_to(current_branch_pos.y, side_branch_interval);
        let new_x_pos = match intersect.connection.get_boundary(){
            Boundary::Left(_, _) => side_branch.get_logical().length - intersect.overlap / 2.0,
            Boundary::Right(_, _) => intersect.overlap / 2.0
        };
        let side_position = Vector2::new(new_x_pos, new_y_pos);
        
        BranchRect {
            branch_id: side_branch.get_id(),
            length: side_length,
            height: self.height,
            pos: side_position,
            color: *PLAYER_COLOR
        }
    }

    pub fn get_render_parts<T: Tree> (&self, tree: &T) -> Vec<Box<Renderable>> {
        self.get_branch_objects(tree)
            .into_iter()
            .flat_map(|p| {p.get_renderables(tree)})
            .collect()
    }
}

impl Movable for Player {
    fn get_position(&self) -> Position {
        self.pos
    }

    fn set_position(&mut self, new_pos: Position) {
        self.pos = new_pos;
    }
}
