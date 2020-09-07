use super::*;
use crate::{GameState, GameScene};
use std::fmt::{self, Display, Formatter};
use druid::widget::{TextBox, Checkbox};

#[derive(Clone, Data, Lens, PartialEq, Eq)]
pub struct SignUpState {
    username: String,
    // TODO Use Password
    password: String,
    local_only: bool,
    pub next_scene: GameScene,
}

impl SignUpState {
    pub fn sign_up(&mut self) {
        self.next_scene = GameScene::Login;
    }
}

impl Default for SignUpState {
    fn default() -> Self {
        SignUpState {
            next_scene: GameScene::SignUp,
            ..Default::default()
        }
    }
}

pub fn signup_window() -> impl Widget<SignUpState> {
    let username = TextBox::new().lens(SignUpState::username);
    // TODO Replace with password box
    let password = TextBox::new().lens(SignUpState::password);
    let local_only = Checkbox::new("Local Account Only").lens(SignUpState::local_only);

    let login = Button::new("Sign Up")
        .on_click(|_ctx, data: &mut SignUpState, _env| data.sign_up());
    Flex::column().with_child(username).with_child(password).with_child(local_only).with_child(login)
}