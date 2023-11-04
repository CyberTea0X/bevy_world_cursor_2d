use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use bevy::window::WindowRef;

use super::resources::CursorWorldPos;

/// Отвечает за вычисления позиции курсора в мире
pub fn cursor_to_world_system<T: Component>(
    // need to get window dimensions
    windows: Query<(&Window, Option<&PrimaryWindow>)>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<T>>,
    mut cursor_pos: ResMut<CursorWorldPos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = match camera_q.get_single() {
        Ok(camera) => camera,
        _ => return,
    };

    // get the window that the camera is displaying to or primary window
    let (window, _) = if let RenderTarget::Window(WindowRef::Entity(id)) = camera.target {
        windows.get(id).unwrap()
    } else {
        match windows.into_iter().find(|(_, p)| p.is_some()) {
            Some(window) => window,
            None => return,
        }
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_pos.x = world_position.x;
        cursor_pos.y = world_position.y;
    }
}
