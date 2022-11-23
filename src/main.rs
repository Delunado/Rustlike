mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

mod player;
pub use player::*;

use rltk::{FontCharType, GameState, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

// Movement
#[derive(Component)]
struct LeftMover {}

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

        let map = self.ecs.fetch::<Vec<TileType>>();
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

    let (rooms, map) = create_map();
    game_state.ecs.insert(map);

    let (player_x, player_y) = rooms[0].center();

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