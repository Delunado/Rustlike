﻿use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
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
}