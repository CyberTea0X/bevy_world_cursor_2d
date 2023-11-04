use bevy::prelude::*;

use super::{resources::CursorWorldPos, systems::cursor_to_world_system};
/// Данный плагин отвечает за вычисление позиции курсора в мире
/// Возможно, имеет смысл выложить на гитхаб
#[derive(Default)]
pub struct CursorToWorldPlugin<T>
where
    T: Component,
{
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Component> Plugin for CursorToWorldPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPos>()
            .add_systems(Update, cursor_to_world_system::<T>);
    }
}
