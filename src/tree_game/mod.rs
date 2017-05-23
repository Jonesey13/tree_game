pub mod tree_branch;
pub mod tree_builder;
pub mod player;
pub mod position;
use self::tree_branch::TreeBranch;
use gg::debug::*;
use gg::games::view_details::{ViewDetails, ViewDetails2D};
use gg::games::GameInput;
use gg::games::Game;
use gg::rendering::{Renderable, BezierRect};
use gg::input::{JoystickInput, KeyboardInput};

pub struct TreeGame {
    pub input_keys: InputKeys,
    pub setup: GameSetup,
    pub state: GameState,
    external_input: ExternalInput,
    pub branches: BranchData,
    view_details: ViewDetails,
    pub player: player::Player
}

impl TreeGame {
    pub fn new(setup: GameSetup) -> TreeGame {
        let game_tree = tree_builder::TreeBuilder::new(10).build_tree();
            
        TreeGame{
            input_keys: InputKeys::default(),
            setup: setup,
            state: Default::default(),
            external_input: Default::default(),
            view_details: ViewDetails::TwoDim(
                ViewDetails2D {
                    use_aspect_ratio: false,
                    ..Default::default()
                }
            ),
            branches: tree_builder::TreeBuilder::new(10).build_tree()
        }
    }

    pub fn reset(&mut self) {
    }

    fn update_view_details(&mut self) {
    }
}

impl Game for TreeGame {
    fn init(&mut self) {
        
    }

    fn update_input(&mut self) {
    }

    fn update_logic(&mut self, t_step: f64){
        debug_clock_start("Logic::update_logic");
        debug_clock_stop("Logic::update_logic");
    }

    fn get_view(&self) -> ViewDetails {
        self.view_details
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        debug_clock_start("Render::get_renderables");
        let mut output: Vec<Box<Renderable>> =
            self.branches.branches
            .iter()
            .map(|br| -> Box<Renderable> { Box::new(BezierRect::from(br.get_visual())) }).collect();
        debug_clock_stop("Render::get_renderables");
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
         Some(&mut self.external_input)
    }
}

#[derive(Default, Copy, Clone)]
pub struct InputKeys{
    pub jump_angle: f64,
    pub jump_radial: f64,
    pub reset: bool,
    pub pause: bool,
    pub pause_lock: bool
}

#[derive(Copy, Clone)]
pub struct GameSetup{
}

impl Default for GameSetup {
    fn default() -> Self {
        GameSetup {
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct GameState{
    pub player_death: bool,
    pub survival_time: f64,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{ player_death: false,
                   survival_time: 0.0,
        }
    }
}

#[derive(Clone, Default)]
struct ExternalInput {
    kbd: KeyboardInput,
    gamepad: JoystickInput
}

impl GameInput for ExternalInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
    fn get_joystick_inp<'a>(&'a mut self) -> Option<&'a mut JoystickInput> { Some(&mut self.gamepad) }
}

pub struct BranchData {
    branches: Vec<TreeBranch>,
    current_id: u64,
}
