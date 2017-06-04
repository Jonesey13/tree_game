use na::Vector2;
use super::tree::BranchId;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    branch_index: BranchId,
    position: Vector2<f64>
}

impl Position {
    pub fn new(branch_index: BranchId, position: Vector2<f64>) -> Position {
        Position {
            branch_index,
            position
        }
    }
    
    pub fn get_branch_index(&self) -> BranchId {
        self.branch_index
    }

    pub fn set_branch_index(&mut self, new_index: BranchId) {
        self.branch_index = new_index;
    }

    pub fn get_branch_position(&self) -> Vector2<f64> {
        self.position
    }

    pub fn set_branch_position(&mut self, new_position: Vector2<f64>) {
        self.position = new_position;
    }
}
