pub mod tree;
pub mod player;
pub mod position;
pub mod movable;
use self::tree::{Tree, TreeBranch, TreeData, TreeBuilder, BranchId};
use self::player::Player;
use self::position::Position;
use self::movable::Movable;
use gg::debug::*;
use gg::games::view_details::{ViewDetails, ViewDetails2D};
use gg::games::GameInput;
use gg::games::Game;
use gg::rendering::{Renderable, BezierRect};
use gg::input::{JoystickInput, KeyboardInput};
use na::Vector2;
use num::Zero;

pub struct TreeGame {
    pub input_keys: InputKeys,
    pub setup: GameSetup,
    pub state: GameState,
    external_input: ExternalInput,
    pub tree: TreeData,
    view_details: ViewDetails,
    pub player: player::Player
}

impl TreeGame {
    pub fn new(setup: GameSetup) -> TreeGame {
        TreeGame {
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
            tree: TreeBuilder::new(8).build_tree(),
            player: Player::new(Position::new(BranchId::new(0, 0), Vector2::zero()), 0.1, 0.1)
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
        self.input_keys.player_mov.x = (self.external_input.kbd.right as isize - (self.external_input.kbd.left as isize)) as f64;
        self.input_keys.player_mov.y = (self.external_input.kbd.up as isize - (self.external_input.kbd.down as isize)) as f64;
    }

    fn update_logic(&mut self, t_step: f64){
        debug_clock_start("Logic::update_logic");
        self.player.change_position(&self.tree, self.input_keys.player_mov * t_step);
        debug_clock_stop("Logic::update_logic");
    }

    fn get_view(&self) -> ViewDetails {
        self.view_details
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        debug_clock_start("Render::get_renderables");
        let mut output: Vec<Box<Renderable>> =
            self.tree.get_branches()
            .values()
            .map(|br| -> Box<Renderable> { Box::new(BezierRect::from(br.get_visual())) })
            .collect();

        let mut player_parts: Vec<Box<Renderable>> = self.player.get_render_parts(&self.tree)
            .into_iter()
            .map( |p| -> Box<Renderable> {Box::new(p)} )
            .collect();
        output.append(&mut player_parts);

        debug_clock_stop("Render::get_renderables");
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
         Some(&mut self.external_input)
    }
}

#[derive(Copy, Clone)]
pub struct InputKeys{
    player_mov: Vector2<f64>
}

impl Default for InputKeys {
    fn default() -> Self {
        InputKeys {
            player_mov: Vector2::zero()
        }
    }
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
