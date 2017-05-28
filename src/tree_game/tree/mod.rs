pub mod tree_branch;
pub mod tree_builder;
pub mod branch_id;
use std::collections::HashMap;

pub use self::tree_branch::TreeBranch;
pub use self::branch_id::BranchId;
pub use self::tree_builder::TreeBuilder;

pub trait Tree {
    fn get_root(&self) -> &TreeBranch;

    fn get_branches(&self) -> &HashMap<BranchId, TreeBranch>;
}


pub struct TreeData {
    branches: HashMap<BranchId, TreeBranch>,
    max_depth: usize
}

impl Tree for TreeData {
    fn get_root(&self) -> &TreeBranch {
        self.branches.get(&BranchId::new(0, 0)).expect("No Root Branch!")
    }

    fn get_branches(&self) -> &HashMap<BranchId, TreeBranch> {
        &self.branches
    }
}
