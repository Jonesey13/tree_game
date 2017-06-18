use super::tree::{Tree};
use super::position::Position;
use na::Vector2;
use gg::debug::*;
//use std;

pub trait Movable {
    fn get_position(&self) -> Position;

    fn set_position(&mut self, Position);

    fn change_position(&mut self, tree: &Tree, change_vec: Vector2<f64>) {
        let current_position = self.get_position();

        let current_branch = tree.get_branches()
            .get(&current_position.get_branch_id())
            .expect("Movable Object Not on a Valid Branch!");

        let new_logical_pos = current_branch.get_new_logical_position(current_position.get_branch_position(), change_vec);

        if new_logical_pos.x < 0.0 {
            if let Some(connection) = current_branch.get_connection(new_logical_pos.y, super::tree::BranchSide::Left) {
                let matching_connection = tree.get_matching_connection(current_branch.get_id(), connection);
                let new_branch_id = connection.get_branch_id();
                let new_branch = tree.get_branch(new_branch_id);
                
                let new_branch_boundary_interval = new_branch.get_logical_boundary_interval(matching_connection.get_boundary());
                let old_branch_boundary_interval = current_branch.get_logical_boundary_interval(connection.get_boundary());
                let new_branch_boundary_pos = old_branch_boundary_interval.fit_point_to(new_logical_pos.y, new_branch_boundary_interval);
                let new_position = Position::new(new_branch_id, Vector2::new(new_branch.get_logical().length, new_branch_boundary_pos));

                self.set_position(new_position);
                // debug(&format!("Player Jumped to the left: Old Position: Branch:{:?} x:{:?} y:{:?}, \n New Position: Branch:{:?} x:{:?} y:{:?}, \n Jump: x:{:?} y:{:?}",
                //                current_position.get_branch_id(), current_position.get_branch_position().x, current_position.get_branch_position().y,
                //                new_position.get_branch_id(), new_position.get_branch_position().x, new_position.get_branch_position().y,
                //                change_vec.x, change_vec.y));
            }
            else {
                let new_position = Position::new(current_position.get_branch_id(), Vector2::new(0.0, new_logical_pos.y));
                self.set_position(new_position);
            }
        }
        else if new_logical_pos.x > current_branch.get_logical().length {
            if let Some(connection) = current_branch.get_connection(new_logical_pos.y, super::tree::BranchSide::Right) {
                let matching_connection = tree.get_matching_connection(current_branch.get_id(), connection);
                let new_branch_id = connection.get_branch_id();
                let new_branch = tree.get_branch(new_branch_id);
                
                let new_branch_boundary_interval = new_branch.get_logical_boundary_interval(matching_connection.get_boundary());
                let old_branch_boundary_interval = current_branch.get_logical_boundary_interval(connection.get_boundary());
                let new_branch_boundary_pos = old_branch_boundary_interval.fit_point_to(new_logical_pos.y, new_branch_boundary_interval);
                let new_position = Position::new(new_branch_id, Vector2::new(0.0, new_branch_boundary_pos));
                self.set_position(new_position);
                // debug(&format!("Player Jumped to the right: Old Position: Branch:{:?} x:{:?} y:{:?}, \n New Position: Branch:{:?} x:{:?} y:{:?}, \n Jump: x:{:?} y:{:?}",
                //                current_position.get_branch_id(), current_position.get_branch_position().x, current_position.get_branch_position().y,
                //                new_position.get_branch_id(), new_position.get_branch_position().x, new_position.get_branch_position().y,
                //                change_vec.x, change_vec.y));
            }
            else {
                let new_position = Position::new(current_position.get_branch_id(), Vector2::new(current_branch.get_logical().length, new_logical_pos.y));
                self.set_position(new_position);
                // debug(&format!("Player Stopped at the right: Old Position: Branch:{:?} x:{:?} y:{:?}, \n New Position: Branch:{:?} x:{:?} y:{:?}, \n Jump: x:{:?} y:{:?} \n connections: {:?}",
                //                current_position.get_branch_id(), current_position.get_branch_position().x, current_position.get_branch_position().y,
                //                new_position.get_branch_id(), new_position.get_branch_position().x, new_position.get_branch_position().y,
                //                change_vec.x, change_vec.y,
                //                current_branch.get_connections()));
            }
        }
        else {
            let new_position = Position::new(current_position.get_branch_id(), new_logical_pos);
            self.set_position(new_position);
        }
    }
}
