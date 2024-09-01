//! Данный модуль вычисляет позицию курсора в мире и сохраняет как ресурс
pub mod plugin;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests {
    use bevy::{
        app::AppExit,
        prelude::{App, Camera2d, EventWriter, Res, Startup},
    };

    use crate::{plugin::CursorToWorldPlugin, resources::CursorWorldPos};

    fn exit_system(mut app_exit_events: EventWriter<AppExit>, cursor: Res<CursorWorldPos>) {
        println!("cursor world position: {}", cursor.to_string());
        app_exit_events.send(AppExit::Success);
    }

    #[test]
    fn it_works() {
        App::new()
            .add_plugins(CursorToWorldPlugin::<Camera2d>::default())
            .add_systems(Startup, exit_system)
            .run();
    }
}
