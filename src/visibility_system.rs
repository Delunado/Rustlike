use bevy::prelude::Query;
use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

pub fn visibility_system(query: Query<(&mut Map, &mut Viewshed, &Position, &Player)>) {
    let (mut map, entities, mut viewshed, pos, player) = query;
    
    for entity in query.iter() {
        if !viewshed.dirty {
            return;
        }

        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y),
                                               viewshed.range, &*map);
        viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        let p: Option<&Player> = player.get(entity);
        if let Some(_p) = p {
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