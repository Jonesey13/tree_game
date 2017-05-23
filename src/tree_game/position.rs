use na::Vector2;

pub struct Position {
    branch_index: u64,
    position: Vector2<f64>
}

impl Position {
    pub fn get_branch_index(&self) -> u64 {
        self.branch_index
    }

    pub fn set_branch_index(&mut self, new_index: u64) {
        self.branch_index = new_index;
    }

    pub fn get_position(&self) -> Vector2<f64> {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vector2<f64>) {
        self.position = new_position;
    }
}
