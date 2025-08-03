use bevy::{prelude::*};
use bevy_egui::{
    egui::{
        self, style::Selection, CornerRadius, Stroke
    }, EguiContexts, EguiPlugin, EguiPrimaryContextPass 
};
use crate::{
    gameplay::{
        GameState,
        PlayerRole
    }, 
    ui::{ControlerUi, ControlerUiDisplay, MainMenuDisplay, MenuWindow, StartupMenu, StarupMenuDisplay, UiDisplayExt, UiTheme, BLACK, BLUE_200, BLUE_700, GRAY_500, WHITE}
};
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiTheme>()
            .init_resource::<StarupMenuDisplay>()
            .init_resource::<MainMenuDisplay>()
            .init_resource::<ControlerUiDisplay>()
            .add_plugins(EguiPlugin::default()) 

            .add_systems(Startup, configure_visuals_system)
            .add_systems(EguiPrimaryContextPass,(add_startup_menu, add_control_ui, ui_menu_system))
            .add_systems(OnExit(GameState::StartupScreen),  enable_control_ui);
    }
}

fn add_startup_menu(
    mut contexts: EguiContexts,
    window: Single<&mut Window> ,
    mut startup_menu_state: ResMut<StarupMenuDisplay>,   
    mut player_role: ResMut<PlayerRole>,
    mut player_theme: ResMut<UiTheme>,
    mut game_state: ResMut<NextState<GameState>>
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut role = &mut player_role;
    let mut theme = &mut player_theme;
    let mut is_display= &mut startup_menu_state.is_display;
    let screen_res  = &window.resolution;
   
    let set_playing = |new_state: GameState| {
        let _ = game_state.set(new_state);
    };

    let _startup_meu = StartupMenu::new(ctx, screen_res, set_playing)
        .show(&mut is_display, &mut role, &mut theme );
        
    Ok(())
}

fn add_control_ui (
    window: Single<&mut Window> ,
    mut contexts: EguiContexts,
    player_theme: Res<UiTheme>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

    mut controler_ui_state: ResMut<ControlerUiDisplay>,

) -> Result {
    let choosed_theme = player_theme.theme;

    let ctx = contexts.ctx_mut()?;
    let screen_res  = &window.resolution;
    if keyboard_input.just_pressed(KeyCode::KeyU)  {
        controler_ui_state.is_display = !controler_ui_state.is_display;
       
    }
    let _control_ui = ControlerUi::new(ctx, &choosed_theme,screen_res).show(&controler_ui_state.is_display);

    Ok(())
}
fn enable_control_ui (
    mut controler_ui_state: ResMut<ControlerUiDisplay>,

) -> Result {
  
    let _ = controler_ui_state.enable();
    Ok(())
}
fn ui_menu_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuDisplay>,   
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_theme: Res<UiTheme>,
    mut game_state: ResMut<NextState<GameState>>

) -> Result {
    let ctx = contexts.ctx_mut()?;
    let choosed_theme = player_theme.theme;

    if keyboard_input.just_pressed(KeyCode::Slash)  {
        menu_state.is_display = !menu_state.is_display;
        game_state.set(GameState::Menu);
    }
     let set_playing = |new_state: GameState| {
        let _ = game_state.set(new_state);
    };
    let _menu = MenuWindow::new(set_playing).show(ctx, &choosed_theme, &mut menu_state.is_display);

   
    Ok(())
}

fn configure_visuals_system( mut contexts: EguiContexts) -> Result {
    contexts.ctx_mut()?.set_visuals(egui::Visuals {
        
        window_corner_radius: CornerRadius::from(5.),
        override_text_color: Some(WHITE),
        selection :Selection {
            bg_fill: BLACK,
            stroke: Stroke::new(1.0, BLACK),
        },
        window_stroke: Stroke::NONE,
        window_fill: GRAY_500,
        ..Default::default()
    });
    Ok(())
}