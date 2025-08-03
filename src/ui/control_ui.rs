use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::egui::{self, emath, epaint::{PathShape, PathStroke}, Color32, Id, LayerId, Order, Painter, Pos2, Rect, Shape};

use crate::ui::{Theme};
#[derive(Clone)]

pub struct ControlerUi {
    is_visible: bool,
    painter: Painter,
    background_color: Color32,
    stroke_color: Color32,
    paint_rect: Rect
}

impl ControlerUi {
    pub fn new(ctx: &egui::Context, player_theme: &Theme,window_res:  &WindowResolution) -> Self {
        let screen_rect = emath::Rect {min: Pos2::new(0.0,0.0), max: Pos2::new(window_res.width(), window_res.height())};
        let layer_id = LayerId::new(Order::Background, Id::new("control_ui"));
        Self { 
            painter: Painter::new(ctx.clone(), layer_id.clone(), screen_rect), 
            background_color: player_theme.background, 
            stroke_color: player_theme.stroke, 
            paint_rect: screen_rect,
            is_visible: true
        }
    }
    pub fn show(&mut self, is_display: &bool) -> Result{

        
        let mut painter = self.painter.clone();
        self.is_visible = *is_display;
        if !self.is_visible {
            painter.set_invisible();
        }

      
        painter.add(
            Shape::Path(
                PathShape { 
                    points: vec![
                        Pos2::new(self.paint_rect.width() / 2. , 0.0),
                        Pos2::new(self.paint_rect.width()  , 0.0),
                        Pos2::new(self.paint_rect.width()  , 25.0),

                        Pos2::new(self.paint_rect.width() / 2. + 25.0, 25.0),
                        Pos2::new(self.paint_rect.width() / 2. , 50.0),
                        Pos2::new(self.paint_rect.width() / 2. - 25., 25.0),
                        Pos2::new(0.0 , 25.0),
                        Pos2::new(0.0 , 0.0),

                    ], 
                    closed: true, 
                    fill: self.background_color, 
                    stroke: PathStroke {
                        width: 2.,
                        color: egui::epaint::ColorMode::Solid(self.stroke_color),
                        kind: egui::StrokeKind::Outside
                    }
                }
            )
        );
        painter.add(
            Shape::Path(
                PathShape { 
                    points: vec![
                        Pos2::new(self.paint_rect.width() / 2. , self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() / 2. + 300., self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() / 2. + 350., self.paint_rect.height() - 75.),
                        Pos2::new(self.paint_rect.width() / 2. + 350., self.paint_rect.height() ),
                        Pos2::new(self.paint_rect.width() / 2. - 350., self.paint_rect.height() ),
                        Pos2::new(self.paint_rect.width() / 2. - 350., self.paint_rect.height() - 75.),
                        Pos2::new(self.paint_rect.width() / 2. - 300., self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() / 2. , self.paint_rect.height() - 125.),
                        
                        

                    ], 
                    closed: true, 
                    fill: self.background_color, 
                    stroke: PathStroke::default()
                }
            )
        );

        painter.add(
            Shape::Path(
                PathShape { 
                    points: vec![
                        Pos2::new(self.paint_rect.width() / 2. + 500., self.paint_rect.height() - 75.),
                        Pos2::new(self.paint_rect.width() / 2. + 550., self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() , self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() , self.paint_rect.height() ),
                        Pos2::new(self.paint_rect.width() / 2. + 500., self.paint_rect.height() ),
                        Pos2::new(self.paint_rect.width() / 2. + 500., self.paint_rect.height() - 75.),
                        
                        
                        

                    ], 
                    closed: true, 
                    fill: self.background_color, 
                    stroke: PathStroke::default()
                }
            )
        );
        painter.add(
            Shape::Path(
                PathShape { 
                    points: vec![
                        Pos2::new(0. , self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() / 2. - 550., self.paint_rect.height() - 125.),
                        Pos2::new(self.paint_rect.width() / 2. - 500., self.paint_rect.height() - 75.),
                        Pos2::new(self.paint_rect.width() / 2. - 500., self.paint_rect.height() ),
                        Pos2::new(0. , self.paint_rect.height() ),
                        
                        
                        

                    ], 
                    closed: true, 
                    fill: self.background_color, 
                    stroke: PathStroke::default()
                }
            )
        );
        Ok(())
    }
}