# bevy_world_cursor_2d

Existing bevy plugins can get cursor position in the window, but usually we
need cursor world position, so i created new plugin.

A very simple plugin for [bevy](https://bevyengine.org/)
All it does is provide a cursor position resource in the 2D game world,
which is automatically updated.
This resource eliminates the need to calculate the cursor position in the world for each individual system.

Usage:
```toml
[dependencies]
bevy_world_cursor_2d = {git = "https://github.com/CyberTea0X/bevy_world_cursor_2d.git"}
```

Example code:
```rust 
// In this case this won't even spawn a window, so cursor position will be [0, 0]
use bevy::{
    app::AppExit,
    prelude::{App, Camera2d, EventWriter, Res, Startup},
};

use crate::{plugin::CursorToWorldPlugin, resources::CursorWorldPos};

#[test]
fn it_works() {
    App::new()
        .add_plugins(CursorToWorldPlugin::<Camera2d>::default())
        .add_systems(Startup, exit_system)
        .run();
}

fn exit_system(mut app_exit_events: EventWriter<AppExit>, cursor: Res<CursorWorldPos>) {
    println!("cursor world position: {}", cursor.to_string());
    app_exit_events.send(AppExit::Success);
}
```
