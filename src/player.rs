use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{components::{Position, Player}, TileType, State};
use std::cmp::{min, max};
use crate::{Map, Viewshed};

pub fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_index = map.get_map_position_index(pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination_index] != TileType::Wall {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => move_player(-1, 0, &mut game_state.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => move_player(1, 0, &mut game_state.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => move_player(0, -1, &mut game_state.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => move_player(0, 1, &mut game_state.ecs),
            _ => {}
        },
    }
}