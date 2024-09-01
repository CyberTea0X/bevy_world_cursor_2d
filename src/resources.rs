use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

/// Позиция курсора в игровом мире. И да, это не тоже самое, что и обычная
/// позиция курсора
#[derive(Resource, Debug, Default)]
pub struct CursorWorldPos {
    pub pos: Vec2,
}

impl Deref for CursorWorldPos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl DerefMut for CursorWorldPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
}
