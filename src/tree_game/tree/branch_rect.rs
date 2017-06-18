use tree_game::tree::{Tree, BranchId};
use gg::rendering::{Renderable, BezierSubrect};
use na::{Vector2, Vector4};
use super::branch_object::BranchObject;

#[derive(Debug)]
pub struct BranchRect {
    pub length: f64,
    pub height: f64,
    pub pos: Vector2<f64>,
    pub color: Vector4<f64>,
    pub branch_id: BranchId
}

impl BranchRect {
    pub fn new(length: f64, height:f64, pos: Vector2<f64>, color: Vector4<f64>, branch: BranchId) -> BranchRect {
        BranchRect {
            length: length,
            height: height,
            pos: pos,
            color: color,
            branch_id: branch
        }
    }
}

impl BranchObject for BranchRect {
    fn get_renderables(&self, tree: &Tree) -> Vec<Box<Renderable>> {
        let tree_branch = tree.get_branch(self.branch_id);
        let bezier_rect = tree_branch.get_visual().into();
        let bezier_logic = tree_branch.get_logical().into();
        
        let subrect = BezierSubrect {
            bezier: bezier_rect,
            logic: bezier_logic,
            length: self.length,
            height: self.height,
            sub_pos: self.pos,
            color: self.color
        };
        
        vec!(Box::new(subrect))
    }
}

