#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BranchId {
    pub id: usize,
    pub layer: usize
}

impl BranchId {
    pub fn new(id: usize, layer: usize) -> BranchId {
        BranchId {
            id,
            layer
        }
    }
}
