use super::*;
use crate::game::{GameState, GameScene};

#[derive(Clone, Data, PartialEq, Eq, Lens)]
pub struct InitialState {
    pub next_scene: GameScene,
}

impl InitialState {
    pub fn login_button(&mut self) {
        self.next_scene = GameScene::Login;
    }
    pub fn sign_up_button(&mut self) {
        self.next_scene = GameScene::SignUp;
    }
}

impl Default for InitialState {
    fn default() -> Self {
        InitialState {
            next_scene: GameScene::Initial
        }
    }
}

pub fn initial_window() -> impl Widget<InitialState> {
    let login = Button::new("Login")
        .on_click(|_ctx, data: &mut InitialState, _env| data.login_button())
        .padding(5.0);
    let signup = Button::new("Sign Up")
        .on_click(|_ctx, data: &mut InitialState, _env| data.sign_up_button())
        .padding(5.0);
    Flex::column().with_child(login).with_child(signup)
}