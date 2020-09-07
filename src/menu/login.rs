use super::*;
use crate::{GameState, GameScene};
use druid::widget::{TextBox, Button};

#[derive(Clone, Data)]
pub struct UserCreationState;

#[derive(Clone, Data, Lens, PartialEq, Eq)]
pub struct LoginState {
    username: String,
    // TODO Use Password
    password: String,
    local_only: bool,
    pub next_scene: GameScene,
}

impl LoginState {
    pub fn login_button(&mut self) {
    }
}

impl Default for LoginState {
    fn default() -> Self {
        LoginState {
            next_scene: GameScene::Login,
            .. Default::default()
        }
    }
}

pub fn login_window() -> impl Widget<LoginState> {
    let username = TextBox::new().lens(LoginState::username);
    // TODO Replace with password box
    let password = TextBox::new().lens(LoginState::password);
    // TODO fix clicks
    let local_login = Button::new("Local Login")
        .on_click(|_ctx, data: &mut LoginState, _env| data.login_button());
    let login = Button::new("Sign Up")
        .on_click(|_ctx, data: &mut LoginState, _env| data.login_button());
    Flex::column().with_child(local_login).with_child(login).with_child(username).with_child(password)
}