use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::egui::{self, emath, epaint::{PathShape, PathStroke, RectShape}, Button, Color32, Id, LayerId, Order, Painter, Pos2, Rect, RichText, Shape, Vec2b, Widget};
use crate::{gameplay::{GameState, PlayerRole, PlayerRoleHierachy}, ui::{Theme, UiTheme, BLACK, BLUE_700, GRAY_700}};


pub struct StartupMenu <F>
where
    F: FnMut(GameState),
{
    is_visible: bool,
    painter: Painter,
    paint_rect: Rect,
    set_game_state: F
}


impl <F>StartupMenu<F> where F: FnMut(GameState) {
    pub fn new(ctx: &egui::Context,window_res:  &WindowResolution, set_state_fn: F) -> Self {
        let screen_rect = emath::Rect {min: Pos2::new(0.0,0.0), max: Pos2::new(window_res.width(), window_res.height())};
        let layer_id = LayerId::new(Order::Background, Id::new("start_up_menu_screen"));
        Self { 
            painter: Painter::new(ctx.clone(), layer_id.clone(), screen_rect), 
            paint_rect: screen_rect,
            is_visible: true,
            set_game_state: set_state_fn
        }
    }
   
    pub fn show(mut self, is_display: &mut bool, player_role: &mut PlayerRole, player_theme: &mut UiTheme ) -> Self { 
        let mut painter = self.painter.clone();
        let ctx = self.painter.ctx();
        // let menu_rect = Rect{ 
        //     min: Pos2 { x: self.paint_rect.width() / 2. - 200., y:  self.paint_rect.height() / 2. - 200. }, 
        //     max: Pos2 { x: self.paint_rect.width() / 2. + 200., y:  self.paint_rect.height() / 2. + 200. }
        // };
        self.is_visible = *is_display;
        if !self.is_visible {
            painter.set_invisible();
        }
        painter.rect_filled(Rect{min: Pos2 { x: 0.0, y: 0.0 }, max: self.paint_rect.max }, 0., BLUE_700);
       
        let mut starup_menu: egui::Window<'_> = egui::Window::new("start up menu")
            .id(egui::Id::new("start_up_menu")) // required since we change the title

            .resizable(false)
            .constrain(true)
            .collapsible(false)
            .title_bar(false)
            .scroll(Vec2b::FALSE)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .default_width(400.)
            .default_height(500.)
            .max_width(400.0)
            .max_height(500.0);
            
        let mut bor_is_display = is_display.clone();
        starup_menu = starup_menu.open(&mut bor_is_display);
        starup_menu.show(ctx, |ui| {
            let start_game_text = egui::RichText::new("Start the game").size(32.0).strong();
            let role_pick_text = egui::RichText::new("You will play as: ").size(18.);
            let mut role_selected = player_role.role.clone();
            let mut before = role_selected;
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    let start_button = Button::new(start_game_text).ui(ui);
                    ui.horizontal(|ui| {
                        ui.label(role_pick_text);
                        egui::ComboBox::from_label("Chosed")
    
                            .selected_text(RichText::new(format!("{:?}", role_selected)).size(18.))
    
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut role_selected, PlayerRoleHierachy::Commander, RichText::new("-Commander- ").size(14.));
                                ui.selectable_value(&mut role_selected, PlayerRoleHierachy::Captain, RichText::new("-Captain-").size(14.));
                                ui.selectable_value(&mut role_selected, PlayerRoleHierachy::Pilot, RichText::new("-Pilot-").size(14.));
                            })
                    });
                    if start_button.clicked() {
                        *is_display = !*is_display;

                        (self.set_game_state)(GameState::Playing);

                    };
                });
                if role_selected != before {
                    player_role.set_role(role_selected);
                  
                    player_theme.set_theme(Theme::create_theme(role_selected));
                    before = role_selected;
                } 
                
            })
        });
      
        self
    }

    
}
