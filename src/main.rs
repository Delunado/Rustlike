mod map;

pub use map::*;

mod rect;

pub use rect::Rect;

mod player;

pub use player::*;

mod components;

mod visibility_system;

pub use visibility_system::VisibilitySystem;

use components::{LeftMover, Position, Renderable, Player, Viewshed};

use rltk::{GameState, RandomNumberGenerator, Rltk, RGB};
use bevy::prelude::*;

//Map
fn draw_map(ecs: &App, ctx: &mut Rltk) {
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, _viewshed) in (&mut players, &mut viewsheds).join() {
        let mut y = 0;
        let mut x = 0;

        for (index, tile) in map.tiles.iter().enumerate() {
            if map.revealed_tiles[index] {
                let glyph;
                let mut foreground;

                match tile {
                    TileType::Floor => {
                        glyph = rltk::to_cp437(' ');
                        foreground = RGB::from_f32(0.5, 0.5, 0.5);
                    }

                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        foreground = RGB::from_f32(0.35, 0.35, 0.15);
                    }
                }

                if !map.visible_tiles[index] { foreground = foreground.to_greyscale() }

                ctx.set(x, y, foreground, RGB::from_f32(0., 0., 0.), glyph);
            }

            x += 1;
            if x > map.width - 1 {
                x = 0;
                y += 1;
            }
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
    ecs: App,
}

impl State {
    fn new() -> Self {
        Self { ecs: App::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        
        draw_map(&self.ecs, ctx);

        self.ecs.update();
    }
}

// Render world
fn render_characters(query_characters: Query<(&Position, &Renderable)>, map: ResMut<Map>) {
    for (pos, render) in query_characters.iter() {
        if map.position_is_inside_map(pos.x, pos.y) {
            let index = map.get_map_position_index(pos.x, pos.y);

            if map.visible_tiles[index] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

// Startup
fn init_world(&world: World) {
    let &map = Map::create_map();
    let (player_x, player_y) = map.rooms[0].center();

    add_player(player_x, player_y, world);
    add_monsters(map, world);
    add_particles(world);

    world.spawn(map);
}

fn add_player(player_x: i32, player_y: i32, &world: World) {
    world.spawn((
        Player {},
        Position { x: player_x, y: player_y },
        Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        }, Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        }));
}

fn add_monsters(&map: Map, &world: World) {
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        world.spawn((
            Position { x, y },
            Renderable {
                glyph: rltk::to_cp437('M'),
                fg: RGB::named(rltk::INDIANRED2),
                bg: RGB::named(rltk::BLACK),
            },
            Viewshed {
                visible_tiles: Vec::new(),
                range: 6,
                dirty: true,
            }));
    }
}

fn add_particles(&world: World) {
    for i in 0..15 {
        world.spawn((
            Position { x: i * 5, y: 70 },
            Renderable {
                glyph: rltk::to_cp437('/'),
                fg: RGB::named(rltk::BLUE),
                bg: RGB::named(rltk::BLACK),
            },
            LeftMover {}
        ));
    }
}

// Main
fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Tutorial")
        .build()?;

    // Configuring game state
    let mut game_state = State::new();

    game_state.ecs.add_system(render_characters);
    
    let &world = game_state.ecs.world;
    init_world(world);

    rltk::main_loop(context, game_state)
}