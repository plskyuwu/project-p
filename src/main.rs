mod component;
mod entity;
mod level;

use raylib::prelude::*;
use std::{process, rc::Rc};

use crate::{
    component::transform::Transform,
    entity::{Entity, drawable::Drawable, moveable::Moveable, player::Player, tile::Tile},
};

const PLAYER_SPEED: f32 = 100.0;

const ERROR_ASSET_NOT_FOUND: &str = "ERROR: Asset not found";

const TEST_LEVEL: &str = "
GGAAA
AAAAG
AAAGG
GGGGG";

const PLAYER_TEXTURE: usize = 0;
const GRASS_TEXTURE: usize = 1;

fn main() {
    println!("INFO: PID: {}", process::id());

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("project-p")
        .resizable()
        .build();

    let rl_audio = RaylibAudio::init_audio_device().unwrap();
    let music = rl_audio
        .new_music("assets/music.ogg")
        .expect("Couln't find asset");

    let textures = [
        Rc::new(
            rl.load_texture(&thread, "assets/player.png")
                .expect(ERROR_ASSET_NOT_FOUND),
        ),
        Rc::new(
            rl.load_texture(&thread, "assets/grass.png")
                .expect(ERROR_ASSET_NOT_FOUND),
        ),
    ];

    music.set_volume(0.1);
    music.play_stream();

    let player = Player::new(
        Transform::new(Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 100.0, y: 100.0 }),
        Rc::clone(&textures[PLAYER_TEXTURE]),
    );

    let grass_tile = Tile::new(
        Vector2 { x: 50.0, y: 50.0 },
        Rc::clone(&textures[GRASS_TEXTURE]),
    );

    let mut entities: Vec<Entity> = vec![Entity::Player(player), Entity::Tile(grass_tile)];

    rl.trace_log(
        TraceLogLevel::LOG_INFO,
        ("LEVEL: String representation:".to_owned() + TEST_LEVEL).as_str(),
    );

    while !rl.window_should_close() {
        music.update_stream();

        process_input(&mut rl, &mut entities);

        let dt = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);

        for entity in entities.iter_mut() {
            entity.apply_velocity(dt);
            entity.draw(&mut d);
        }

        d.clear_background(Color::RAYWHITE);
        d.draw_fps(0, 0);
    }
}

fn process_input(rl: &mut RaylibHandle, entities: &mut [Entity]) {
    let mut new_player_velocity = Vector2 { x: 0.0, y: 0.0 };

    if rl.is_key_down(KeyboardKey::KEY_W) {
        new_player_velocity.y -= 1.0;
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        new_player_velocity.y += 1.0;
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        new_player_velocity.x -= 1.0;
    }

    if rl.is_key_down(KeyboardKey::KEY_D) {
        new_player_velocity.x += 1.0;
    }

    entities[0].set_velocity(new_player_velocity.normalized() * PLAYER_SPEED);
}
