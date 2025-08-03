
use bevy::prelude::*;
use bevy_egui::{egui::*, egui::{self, Button}};

use crate::{gameplay::GameState, ui::Theme};
#[derive(Clone, PartialEq)]
pub struct MenuWindow<F>
where
    F: FnMut(GameState), {
    title: String,
    title_bar: bool,
    closable: bool,
    collapsible: bool,
    resizable: bool,
    constrain: bool,
    scroll2: egui::Vec2b,

    anchored: bool,
    anchor: egui::Align2,
    anchor_offset: egui::Vec2,
    toggle_menu_state_fn: F
}


// impl Default for MenuWindow {
//     fn default() -> Self {
//         Self {
//             title: "Menu".to_owned(),
//             title_bar: false,
//             closable: true,
//             collapsible: true,
//             resizable: false,
//             constrain: true,
//             scroll2: Vec2b::TRUE,
//             anchored: true,
//             anchor: egui::Align2::CENTER_CENTER,
//             anchor_offset: egui::Vec2::ZERO,
//         }
//     }
// }

impl  <F>MenuWindow<F> where F: FnMut(GameState) {
    pub fn new( toggle_menu_state_fn: F) -> Self {
        Self {
            title: "Menu".to_owned(),
            title_bar: false,
            closable: true,
            collapsible: true,
            resizable: false,
            constrain: true,
            scroll2: Vec2b::TRUE,
            anchored: true,
            anchor: egui::Align2::CENTER_CENTER,
            anchor_offset: egui::Vec2::ZERO,
            toggle_menu_state_fn: toggle_menu_state_fn
        }
    }
    pub fn show(&mut self,ctx: &egui::Context, player_theme: &Theme, mut is_open: &mut bool) {
        let mut is_window_open = &mut is_open.clone();
       

       
       
        let mut window: egui::Window  = egui::Window::new(self.title.clone())
            .id(egui::Id::new("Menu")) // required since we change the title
            .resizable(self.resizable.clone())
            .constrain(self.constrain.clone())
            .collapsible(self.collapsible.clone())
            .title_bar(false)
            .scroll(Vec2b::FALSE)
            .max_width(300.0)
            .max_height(200.0);
        if self.anchored {
            window = window.anchor(self.anchor.clone(), self.anchor_offset.clone());
        }
         if self.closable {
            window = window.open(&mut is_window_open);
        }
        // window
        window.show(ctx, |ui| self.ui(ui, player_theme, &mut is_open));
    }   
    pub fn ui(&mut self, ui: &mut egui::Ui,player_theme: &Theme,  mut is_open: &mut bool) {
        let menu_text = RichText::new("Main menu").color(player_theme.text).size(24.0).strong();
        let resume_game = RichText::new("Resume game").color(player_theme.text).size(14.0);
        ui.vertical_centered(|ui| {
        
           ui.label(menu_text);
           
        });

       

       

        ui.separator();
        ui.horizontal(|ui| {
            
            if Button::new(resume_game).ui(ui).clicked()   {
            
                Self::close_menu(&mut is_open);
                (self.toggle_menu_state_fn)(GameState::Playing);
            };
        });
       
    }
    pub fn close_menu( is_open: &mut bool){
        *is_open = !*is_open;
    }
}

