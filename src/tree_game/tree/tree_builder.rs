use super::tree_branch::{BranchType, TreeBranch};
use super::TreeData;
use super::{BranchId, Boundary};
use na::Vector2;
use std::collections::HashMap;

pub struct TreeBuilder {
    pub num_layers: usize,
    current_indices: Vec<usize>,
    vertical_fill: f64,
    horizontal_fill: f64,
    branches: HashMap<BranchId, TreeBranch>
}

impl TreeBuilder {
    pub fn new(num_layers: usize) -> TreeBuilder {
        TreeBuilder {
            num_layers: num_layers,
            current_indices: vec![0; num_layers],
            vertical_fill: 0.5,
            horizontal_fill: 0.3,
            branches: HashMap::new()            
        }
    }
    
    pub fn build_tree(mut self) -> TreeData {
        self.build_single_branch_recursive(0, Vector2::new(-1.0, 0.0), None);
        TreeData {
            branches: self.branches,
            max_depth: self.num_layers - 1
        }
    }

    ////////////////////////////////////////////////////
    //              //          Branch Top
    ////////////////     ///////////////////////////////
    // Trunk           /////////////////////////////////
    ////////////////     /////////////////////////////// 
    //              //          Branch Bottom
    ////////////////////////////////////////////////////
    pub fn build_single_branch_recursive(&mut self, depth: usize, left_center_pos: Vector2<f64>, parent_branch: Option<&mut TreeBranch>) {

        // Build the trunk
        let trunk_index = self.generate_new_index(depth);
        let mut trunk_branch = TreeBranch::new(
            trunk_index,
            left_center_pos,
            BranchType::Trunk,
            self.vertical_fill,
            self.horizontal_fill
        );
        if let Some(parent) = parent_branch {
            let trunk_boundary = Boundary::Left(-1.0, 1.0);
            let parent_boundary = Boundary::Right(-1.0, 1.0);
            trunk_branch.add_connection(trunk_boundary, parent, parent_boundary); 
        }

        let trunk_width =  (1.0 - (1.0 - 2.0 * self.vertical_fill).powi((depth as i32) + 1)) / 2f64.powi((depth as i32) + 1);
        let trunk_length = self.horizontal_fill * (1.0 - self.horizontal_fill).powi(depth as i32) / 2.0;

        // Build Branch Top
        let left_top_pos = left_center_pos + Vector2::new(trunk_length * 2.0, trunk_width / 2.0);
        let left_index = self.generate_new_index(depth);
        let mut branch_top = TreeBranch::new (
            left_index,
            left_top_pos,
            BranchType::BranchTop,
            self.vertical_fill,
            self.horizontal_fill
        );
        let left_top_end_pos = left_top_pos
            + branch_top.get_visual().patch.control.eval(1.0)
            - branch_top.get_visual().patch.control.eval(0.0);
        let branch_top_boundary = Boundary::Left(-1.0, 1.0);
        let trunk_top_boundary = Boundary::Right(0.0, 1.0);
        branch_top.add_connection(branch_top_boundary, &mut trunk_branch, trunk_top_boundary);        
        if depth != self.num_layers - 1 {
            self.build_single_branch_recursive(depth + 1, left_top_end_pos, Some(&mut branch_top));
        }

        // Build Branch Bottom 
        let left_bottom_pos = left_center_pos + Vector2::new(trunk_length * 2.0, -trunk_width / 2.0);
        let right_index = self.generate_new_index(depth);
        let mut branch_bottom = TreeBranch::new(
            right_index,
            left_bottom_pos,
            BranchType::BranchBottom,
            self.vertical_fill,
            self.horizontal_fill
        );
        let left_bottom_end_pos = left_bottom_pos
            + branch_bottom.get_visual().patch.control.eval(1.0)
            - branch_bottom.get_visual().patch.control.eval(0.0);
        let branch_bottom_boundary = Boundary::Left(-1.0, 1.0);
        let trunk_bottom_boundary = Boundary::Right(-1.0, 0.0);
        branch_bottom.add_connection(branch_bottom_boundary, &mut trunk_branch, trunk_bottom_boundary);
        if depth != self.num_layers - 1 {
            self.build_single_branch_recursive(depth + 1, left_bottom_end_pos, Some(&mut branch_bottom));
        }

        // Add Branches to Tree
        self.branches.insert(trunk_index, trunk_branch);
        self.branches.insert(left_index, branch_top);
        self.branches.insert(right_index, branch_bottom);
    }

    pub fn generate_new_index(&mut self, depth: usize) -> BranchId {
        let old_index = self.current_indices[depth];
        self.current_indices[depth] += 1;
        BranchId {
            id: old_index,
            layer: depth
        }
    }
}
