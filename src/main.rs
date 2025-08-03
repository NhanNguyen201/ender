use bevy::prelude::{App, AppExit};
use ender::*;
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}