mod system;
use bevy::{prelude::*, app::App};

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

pub fn gui(){
    App::new()
       .add_plugins(DefaultPlugins)
       .insert_resource(ClearColor(BACKGROUND_COLOR))
       .add_startup_system(system::setup)
       .add_system(system::button_system)
       .run();
}