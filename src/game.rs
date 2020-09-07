use crate::menu::*;
use druid::{Widget, WidgetExt, Data, Lens};
use druid::widget::{ViewSwitcher, Flex};

#[derive(Clone, Data, Lens, Default)]
pub struct GameState {
    current_scene: GameScene,
    initial_state: InitialState,
    login_state: LoginState,
    signup_state: SignUpState,
}

#[derive(Clone, Debug, Data, PartialEq, Eq, Copy)]
pub enum GameScene {
    Login,
    SignUp,
    Initial,
}

impl GameState {
    pub fn get_next_scene(&self) -> GameScene {
        use GameScene::*;
        match self.current_scene {
            Initial => self.initial_state.next_scene,
            SignUp => self.signup_state.next_scene,
            Login => self.login_state.next_scene,
        }
    }
}

impl Default for GameScene {
    fn default() -> Self {
        GameScene::Initial
    }
}

pub fn game_window() -> impl Widget<GameState> {
    let view_switcher = ViewSwitcher::new(
        |data: &GameState, _env| data.get_next_scene(),
        |selector, _data, _env| match selector {
        GameScene::Initial => initial_window().lens(GameState::initial_state).boxed(),
        GameScene::SignUp => signup_window().lens(GameState::signup_state).boxed(),
        GameScene::Login => login_window().lens(GameState::login_state).boxed(),
        _ => unimplemented!(),
    });
    Flex::row().with_flex_child(view_switcher, 1.0)
}