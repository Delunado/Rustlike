use rltk::{RGB, FontCharType, BTerm};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Renderer {
    pub render: BTerm,
}

impl Renderer {
    pub fn render(&mut self, pos_x: i32, pos_y: i32, background: RGB, foreground: RGB, glyph: FontCharType) {
        self.render.set(pos_x, pos_y, background, foreground, glyph);
    }
}