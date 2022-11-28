use bevy::prelude::{Query, ResMut};
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

pub fn visibility_system(mut query: Query<(&mut Viewshed, &Position, Option<&Player>)>, mut map: ResMut<Map>) {
    for (mut viewshed, position, player) in query.iter_mut() {
        if !viewshed.dirty {
            return;
        }

        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(position.x, position.y),
                                               viewshed.range, &*map);
        viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        if let Some(_player) = player {
            for tile in map.visible_tiles.iter_mut() { *tile = false }

            for vis in viewshed.visible_tiles.iter() {
                let index = map.get_map_position_index(vis.x, vis.y);
                map.revealed_tiles[index] = true;
                map.visible_tiles[index] = true;
            }
        }

        viewshed.dirty = false;
    }
}