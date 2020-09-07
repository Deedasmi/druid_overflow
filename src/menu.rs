pub mod initial;
pub mod login;
pub mod signup;

pub use initial::*;
pub use login::*;
pub use signup::*;

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, LocalizedString, Lens, Env, PlatformError, Widget, WidgetExt};