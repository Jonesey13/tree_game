pub mod tree_branch;
pub mod tree_builder;
pub mod branch_id;
pub mod connection;
use std::collections::HashMap;

pub use self::tree_branch::TreeBranch;
pub use self::branch_id::BranchId;
pub use self::tree_builder::TreeBuilder;
pub use self::tree_branch::BranchSide;
pub use self::connection::{Connection, Boundary};

pub trait Tree {
    fn get_root(&self) -> &TreeBranch;

    fn get_branches(&self) -> &HashMap<BranchId, TreeBranch>;

    fn get_branch(&self, id: BranchId) -> &TreeBranch {
        self.get_branches().get(&id).expect(&format!("Could not find branch with id: {:?}", id))
    }

    fn get_matching_connection(&self, branch: BranchId, connection: &Connection) -> &Connection {
        let matching_branch = self.get_branch(connection.get_id());

        matching_branch
            .get_connections()
            .iter()
            .find(|c| { c.get_id() == branch})
            .expect(&format!("Could Not Find Matching Connection for Branch {:?} and Connection {:?}", branch, connection))
    }
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
