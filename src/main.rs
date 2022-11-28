mod map;

pub use map::*;

mod rect;

pub use rect::Rect;

mod player;

pub use player::*;

mod components;

mod visibility_system;

pub use visibility_system::VisibilitySystem;

mod renderer;

pub use renderer::*;

use components::{Position, Renderable, Player, Viewshed, Particle};

use rltk::{GameState, RandomNumberGenerator, Rltk, RGB};
use bevy::prelude::*;

//Map
fn render_map(query: Query<(With<Player>, With<Viewshed>)>, map: Res<Map>, mut renderer: ResMut<Renderer>) {
    for _ in query.iter() {
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

                renderer.render(x, y, foreground, RGB::from_f32(0., 0., 0.), glyph);
            }

            x += 1;
            if x > map.width - 1 {
                x = 0;
                y += 1;
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

        //Convert into a system using an Input resource
        //player_input(self, ctx);

        self.ecs.update();
    }
}

// Rendering
fn render_characters(query_characters: Query<(&Position, &Renderable)>, map: Res<Map>, mut renderer: ResMut<Renderer>) {
    for (pos, render) in query_characters.iter() {
        if map.position_is_inside_map(pos.x, pos.y) {
            let index = map.get_map_position_index(pos.x, pos.y);

            if map.visible_tiles[index] {
                renderer.render(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn render_particles(query_particles: Query<(&Position, &Renderable, With<Particle>)>, map: Res<Map>, mut renderer: ResMut<Renderer>) {
    for (pos, render, _) in query_particles.iter() {
        if map.position_is_inside_map(pos.x, pos.y) {
            renderer.render(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// Movement
fn update_particles(mut query: Query<(&mut Position, With<Particle>)>) {
    for (mut pos, _) in query.iter_mut() {
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

// Start up
fn add_player(mut commands: Commands, map: Res<Map>) {
    let (player_x, player_y) = map.rooms[0].center();

    commands.spawn((
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

fn add_monsters(mut commands: Commands, map: Res<Map>) {
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        commands.spawn((
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

fn add_particles(mut commands: Commands) {
    for i in 0..15 {
        commands.spawn((
            Position { x: i * 5, y: 70 },
            Renderable {
                glyph: rltk::to_cp437('/'),
                fg: RGB::named(rltk::BLUE),
                bg: RGB::named(rltk::BLACK),
            },
            Particle {}
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
    
    let map = Map::create_map();
    game_state.ecs.insert_resource(map);

    let renderer = Renderer::new(&context);
    game_state.ecs.insert_resource(renderer);

    game_state.ecs.add_startup_system(add_player);
    game_state.ecs.add_startup_system(add_monsters);
    game_state.ecs.add_startup_system(add_particles);

    game_state.ecs.add_system(update_particles);

    game_state.ecs.add_system(render_characters);
    game_state.ecs.add_system(render_map);
    game_state.ecs.add_system(render_particles);

    rltk::main_loop(context, game_state)
}