
use bevy::prelude::*;

pub mod main_ingame_menu;
pub use main_ingame_menu::*;

pub mod control_ui;
pub use control_ui::*;

pub mod theme;
pub use theme::*;

pub mod start_menu_screen;
pub use start_menu_screen::*;

use crate::gameplay::PlayerRoleHierachy;

pub trait UiDisplayExt {
    fn enable(&mut self)  -> Self;
    fn disable(&mut self)  -> Self;
}

#[derive(Resource, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]

pub struct StarupMenuDisplay {
    pub is_display : bool
}

impl Default for StarupMenuDisplay {
    fn default() -> Self {
        Self {
            is_display: true
        }
    }
    
}

impl UiDisplayExt for StarupMenuDisplay {
    fn enable(&mut self) -> Self{
        self.is_display = true;
        *self
    }
    fn disable(&mut self) -> Self{
        self.is_display = false;
        *self
    }
}


#[derive(Resource, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]

pub struct MainMenuDisplay {
    pub is_display: bool,
}
impl Default for MainMenuDisplay {
    fn default() -> Self {
        Self {
            is_display: false
        }
    }
    
}

impl UiDisplayExt for MainMenuDisplay {
    fn enable(&mut self) -> Self {
        self.is_display = true;
        *self
    }
    fn disable(&mut self)  -> Self {
        self.is_display = false;
        *self

    }
}


#[derive(Resource, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]

pub struct ControlerUiDisplay {
    pub is_display: bool
}

impl Default for ControlerUiDisplay {
    fn default() -> Self {
        Self {
            is_display: false
        }
    }
}

impl UiDisplayExt for ControlerUiDisplay {
    fn enable(&mut self) -> Self{
        self.is_display = true;
        *self
    }
    fn disable(&mut self) -> Self{
        self.is_display = false;
        *self

    }
}



#[derive(Resource, Clone, Copy)]
pub struct UiTheme {
    pub theme: Theme
}
impl Default for UiTheme {
    fn default() -> Self {
        Self {
            theme: Theme::create_theme(PlayerRoleHierachy::Commander)
        }
    }
}
impl UiTheme {
    fn set_theme(&mut self, new_theme: Theme){
        self.theme = new_theme;
    }
}