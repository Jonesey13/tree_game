use gg::geometry::Interval;
use super::BranchId;

#[derive(Debug, Clone)]
pub struct Connection {
    id: BranchId,
    boundary: Boundary
}

impl Connection {
    pub fn new(id: BranchId, boundary: Boundary) -> Connection {
        Connection {
            id,
            boundary
        }
    }
    
    pub fn get_interval(&self) -> Interval {
        match self.boundary {
            Boundary::Left(start, end) => Interval::new(start, end),
            Boundary::Right(start, end) => Interval::new(start, end),
        }
    }

    pub fn get_branch_id(&self) -> BranchId {
        self.id
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Boundary {
    Left(f64, f64),
    Right(f64, f64)
}

impl Boundary {
    pub fn get_interval(&self) -> Interval {
        match self {
            &Boundary::Left(start, end) => Interval::new(start, end),
            &Boundary::Right(start, end) => Interval::new(start, end),
        }
    }
}
