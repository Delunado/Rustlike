mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

mod player;
pub use player::*;

mod components;
use components::{LeftMover, Position, Renderable, Player};

use rltk::{GameState, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;

//Map
fn draw_map(map: &Map, ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.tiles.iter() {
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
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            let mut random = RandomNumberGenerator::new();

            let y_movement: i32 = random.range(0, 3);

            pos.y += y_movement;
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
            if pos.y > 50 {
                pos.y = 0;
            }
        }
    }
}

// Game State
pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();

        player_input(self, ctx);

        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// Main
fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Tutorial")
        .build()?;

    // Configuring game state
    let mut game_state = State { ecs: World::new() };

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<LeftMover>();
    game_state.ecs.register::<Player>();

    let map = Map::create_map();
    let (player_x, player_y) = map.rooms[0].center();
    
    game_state.ecs.insert(map);

    game_state
        .ecs
        .create_entity()
        .with(Player {})
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 0..15 {
        game_state
            .ecs
            .create_entity()
            .with(Position { x: i * 5, y: 70 })
            .with(Renderable {
                glyph: rltk::to_cp437('/'),
                fg: RGB::named(rltk::BLUE),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, game_state)
}