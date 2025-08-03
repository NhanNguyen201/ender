use bevy_egui::egui::Color32;
use crate::gameplay::PlayerRoleHierachy;

pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);

pub const GRAY_100: Color32 = Color32::from_rgb(238, 238, 238);
pub const GRAY_200: Color32 = Color32::from_rgb(221, 221, 221);
pub const GRAY_300: Color32 = Color32::from_rgb(204, 204, 204);
pub const GRAY_400: Color32 = Color32::from_rgb(187, 187, 187);
pub const GRAY_500: Color32 = Color32::from_rgb(170, 170, 170);
pub const GRAY_700: Color32 = Color32::from_rgb(123, 123, 123);
   

pub const BLUE_200: Color32 = Color32::from_rgb(191, 219, 247);
pub const BLUE_300: Color32 = Color32::from_rgb(157, 203, 243);
pub const BLUE_400: Color32 = Color32::from_rgb(111, 174, 235);
pub const BLUE_700: Color32 = Color32::from_rgb(47, 95, 198);


pub const YELLOW_200: Color32 = Color32::from_rgb(240, 216, 151);
pub const YELLOW_300: Color32 = Color32::from_rgb(235, 198, 111);
pub const YELLOW_700: Color32 = Color32::from_rgb(235, 198, 111);



/// The colors for a theme variant.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Theme {
  
    pub stroke: Color32,
    pub text: Color32,
    pub background: Color32
}
impl Theme {
    pub fn create_theme(player_role: PlayerRoleHierachy) -> Theme {
        let stroke_color = match player_role {
            PlayerRoleHierachy::Commander => BLUE_200,
            PlayerRoleHierachy::Captain => YELLOW_200,
            PlayerRoleHierachy::Pilot => GRAY_200,
        };
        let bg_color = match player_role {
            PlayerRoleHierachy::Commander => GRAY_700,
            PlayerRoleHierachy::Captain => YELLOW_700,
            PlayerRoleHierachy::Pilot => BLUE_700,
        };
        Theme { 
            stroke: stroke_color, 
            text: WHITE, 
            background: bg_color
        }
    }
    
}