use bevy::{prelude::*, window::WindowMode};


#[derive(Debug, Resource)]
pub struct ScreenModeSettings {
    is_fullscreen: bool
}
impl Default for ScreenModeSettings {
    fn default() -> Self {
        Self {
            is_fullscreen: true
        }
    }
}
pub struct ScreenModePlugin;
impl Plugin for ScreenModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenModeSettings>();
        // app.add_systems(Startup, starup_screen);
        app.add_systems(Update, toggle_screen);
    }   
}

// fn starup_screen(mut window: Single<&mut Window>, screen_settings: Res<ScreenModeSettings>){
//     if screen_settings.is_fullscreen  {
//         window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);

//     } else {
//         window.mode = WindowMode::Windowed;

//     }
// }
fn toggle_screen(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>, mut screen_settings: ResMut<ScreenModeSettings>){
    if input.just_pressed(KeyCode::KeyT) {
        screen_settings.is_fullscreen = !screen_settings.is_fullscreen;
        if screen_settings.is_fullscreen  {
            // window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
            window.mode = WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
    
        } else {
            window.mode = WindowMode::Windowed;
    
        }
    }
}