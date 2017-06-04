use super::position::Position;
use rendering::BezierSubrect;
use super::tree::{TreeBranch, Tree};
use super::movable::Movable;
use na::Vector4;
use gg::debug::*;

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

    pub fn get_render_parts<T: Tree> (&self, tree: &T) -> Vec<BezierSubrect> {
        let current_branch = tree.get_branches().values().find(|b| {b.get_id() == self.pos.get_branch_index() });

        if let Some(branch) = current_branch {
            let single_part = BezierSubrect {
                bezier: branch.get_visual().into(),
                logic: branch.get_logical().into(),
                length: self.length,
                height: self.height,
                sub_pos: self.pos.get_branch_position(),
                color: Vector4::new(1.0, 1.0, 1.0, 1.0)
            };

            vec![single_part]
        }
        else {
            panic!("Could not find current branch for player with id {:?}", self.pos.get_branch_index());
        }
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
