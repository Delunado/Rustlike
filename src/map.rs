use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator};
use std::cmp::{max, min};
use super::{Rect};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Flower,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles : Vec<bool>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn get_map_position_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn adjust_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.get_map_position_index(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn adjust_horizontal_tunnel(&mut self, x_from: i32, x_to: i32, y: i32) {
        for x in min(x_from, x_to)..=max(x_from, x_to) {
            let index = self.get_map_position_index(x, y);

            if index > 0 && index < self.width as usize * self.height as usize {
                self.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn adjust_vertical_tunnel(&mut self, y_from: i32, y_to: i32, x: i32) {
        for y in min(y_from, y_to)..=max(y_from, y_to) {
            let idx = self.get_map_position_index(x, y);

            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn create_map() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            revealed_tiles: vec![false; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);

            let mut valid_room = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(&other_room) {
                    valid_room = false;
                    break;
                }
            }

            if valid_room {
                map.adjust_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.adjust_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.adjust_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.adjust_vertical_tunnel(prev_y, new_y, prev_x);
                        map.adjust_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index as usize] == TileType::Wall
    }
}