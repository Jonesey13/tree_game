use super::tree_branch::{BranchType, TreeBranch};
use super::BranchData;
use na::Vector2;

pub struct TreeBuilder {
    pub num_layers: u64,
    current_index: u64,
    vertical_fill: f64,
    horizontal_fill: f64,
    branch_list: Vec<TreeBranch>
}

impl TreeBuilder {
    pub fn new(num_layers: u64) -> TreeBuilder {
        TreeBuilder {
            num_layers: num_layers,
            current_index: 0,
            vertical_fill: 0.5,
            horizontal_fill: 0.3,
            branch_list: Vec::new()            
        }
    }
    
    pub fn build_tree(mut self) -> BranchData {
        self.build_single_branch_recursive(0, Vector2::new(-1.0, 0.0));
        BranchData {
            branches: self.branch_list,
            current_id: self.current_index
        }
    }

    pub fn build_single_branch_recursive(&mut self, depth: u64, left_center_pos: Vector2<f64>) {
        let trunk_branch = TreeBranch::new(
            self.generate_new_index(),
            depth,
            left_center_pos,
            BranchType::Trunk,
            self.vertical_fill,
            self.horizontal_fill
        );
        self.branch_list.push(trunk_branch);


        let trunk_width =  (1.0 - (1.0 - 2.0 * self.vertical_fill).powi((depth as i32) + 1)) / 2f64.powi((depth as i32) + 1);
        let trunk_length = self.horizontal_fill * (1.0 - self.horizontal_fill).powi(depth as i32) / 2.0;
        
        let left_top_pos = left_center_pos + Vector2::new(trunk_length * 2.0, trunk_width / 2.0);
        let branch_top = TreeBranch::new(
            self.generate_new_index(),
            depth,
            left_top_pos,
            BranchType::BranchTop,
            self.vertical_fill,
            self.horizontal_fill
        );
        let left_top_end_pos = left_top_pos
            + branch_top.get_visual().patch.control.eval(1.0)
            - branch_top.get_visual().patch.control.eval(0.0);
        self.branch_list.push(branch_top);
        if depth != self.num_layers {
            self.build_single_branch_recursive(depth + 1, left_top_end_pos);
        }

        let left_bottom_pos = left_center_pos + Vector2::new(trunk_length * 2.0, -trunk_width / 2.0);
        let branch_bottom = TreeBranch::new(
            self.generate_new_index(),
            depth,
            left_bottom_pos,
            BranchType::BranchBottom,
            self.vertical_fill,
            self.horizontal_fill
        );
        let left_bottom_end_pos = left_bottom_pos
            + branch_bottom.get_visual().patch.control.eval(1.0)
            - branch_bottom.get_visual().patch.control.eval(0.0);
        self.branch_list.push(branch_bottom);
        if depth != self.num_layers {
            self.build_single_branch_recursive(depth + 1, left_bottom_end_pos);
        }
    }

    pub fn generate_new_index(&mut self) -> u64 {
        let old_index = self.current_index;
        self.current_index += 1;
        old_index
    }
}
