use tree_game::tree::Tree;
use gg::rendering::Renderable;

pub trait BranchObject {
    fn get_renderables(&self, tree: &Tree) -> Vec<Box<Renderable>>;
}
