use na::Vector2;
use super::tree::BranchId;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    branch_id: BranchId,
    branch_position: Vector2<f64>
}

impl Position {
    pub fn new(branch_id: BranchId, branch_position: Vector2<f64>) -> Position {
        Position {
            branch_id,
            branch_position
        }
    }
    
    pub fn get_branch_id(&self) -> BranchId {
        self.branch_id
    }

    pub fn set_branch_id(&mut self, new_id: BranchId) {
        self.branch_id = new_id;
    }

    pub fn get_branch_position(&self) -> Vector2<f64> {
        self.branch_position
    }

    pub fn set_branch_position(&mut self, new_position: Vector2<f64>) {
        self.branch_position = new_position;
    }
}
