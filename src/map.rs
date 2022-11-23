use rltk::{Rltk, RandomNumberGenerator, RGB};
use std::cmp::{max, min};
use super::{Rect};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Flower,
}

pub fn get_map_position_index(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Creates a basic random map with 400 walls and some flowers.
pub fn create_basic_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    //Boundaries walls
    for x in 0..80 {
        map[get_map_position_index(x, 0)] = TileType::Wall;
        map[get_map_position_index(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[get_map_position_index(0, y)] = TileType::Wall;
        map[get_map_position_index(79, y)] = TileType::Wall;
    }

    //Set random walls
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let index = get_map_position_index(x, y);

        if index != get_map_position_index(40, 25) {
            map[index] = TileType::Wall;
        }
    }

    for _i in 0..100 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let index = get_map_position_index(x, y);

        if index != get_map_position_index(40, 25) && map[index] != TileType::Wall {
            map[index] = TileType::Flower;
        }
    }

    map
}

pub fn create_map() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - w - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let mut validRoom = true;

        for other_room in rooms.iter() {
            if new_room.intersect(&other_room) {
                validRoom = false;
                break;
            }
        }

        if validRoom {
            adjust_room_to_map(&new_room, &mut map);
            
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                
                if rng.range(0,2) == 1 {
                    adjust_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    adjust_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    adjust_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    adjust_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }
            
            rooms.push(new_room);
        }
    }

    (rooms, map)
}

fn adjust_room_to_map(room: &Rect, map: &mut Vec<TileType>) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[get_map_position_index(x, y)] = TileType::Floor;
        }
    }
}

fn adjust_horizontal_tunnel(map: &mut Vec<TileType>, xFrom: i32, xTo: i32, y: i32) {
    for x in min(xFrom, xTo)..=max(xFrom, xTo) {
        let index = get_map_position_index(x, y);
        if index > 0 && index < 80 * 50 {
            map[index as usize] = TileType::Floor;
        }
    }
}

fn adjust_vertical_tunnel(map: &mut Vec<TileType>, yFrom: i32, yTo: i32, x: i32) {
    for y in min(yFrom, yTo)..=max(yFrom, yTo) {
        let index = get_map_position_index(x, y);
        if index > 0 && index < 80 * 50 {
            map[index as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437(' '))
            }

            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.45, 0.45, 0.35), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }

            TileType::Flower => {
                ctx.set(x, y, RGB::from_f32(0.8, 0.2, 0.3),
                        RGB::from_f32(0., 0., 0.), rltk::to_cp437(','));
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}