mod menu;
mod game;

use game::*;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Env, PlatformError, Widget, WidgetExt, WindowDesc};

#[tokio::main]
async fn main() -> Result<(), PlatformError>{
    println!("Starting...");
    let main_window = WindowDesc::new(game_window);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(GameState::default())
}