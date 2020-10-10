use macroquad::prelude::*;
use particles::{AtlasConfig, Emitter, EmitterConfig};

fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 0.2,
        explosiveness: 0.6,
        amount: 15,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        ..Default::default()
    }
}

fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 1.0,
        amount: 20,
        initial_direction_spread: 0.2,
        atlas: Some(AtlasConfig::new(4, 4, 0..8)),
        ..Default::default()
    }
}

#[macroquad::main("Fountain")]
async fn main() {
    let texture = load_texture("examples/smoke_fire.png").await;

    let mut one_shot_emitter = particles::Emitter::new(EmitterConfig {
        texture: Some(texture),
        ..explosion()
    });

    let mut flying_emitter_local = Emitter::new(EmitterConfig {
        local_coords: true,
        texture: Some(texture),
        ..smoke()
    });
    let mut flying_emitter_world = Emitter::new(EmitterConfig {
        local_coords: false,
        texture: Some(texture),
        ..smoke()
    });

    loop {
        clear_background(BLACK);

        draw_text("Local coord emitter", 20.0, 0.0, 30.0, RED);

        draw_text("World coord emitter", 20.0, 30.0, 30.0, GREEN);

        draw_text(
            "One shot emitter, press Space to emit",
            20.0,
            60.0,
            30.0,
            YELLOW,
        );
        draw_circle(650.0, 82.0, 15.0, YELLOW);

        one_shot_emitter.draw(vec2(650.0, 82.0));
        if is_key_pressed(KeyCode::Space) {
            one_shot_emitter.config.emitting = true;
        }

        let local_emitter_pos = vec2(
            (get_time() * 0.3).sin() as f32 * screen_width() / 2.5 + screen_width() / 2.0,
            (get_time() * 0.5).cos() as f32 * screen_height() / 2.5 + screen_height() / 2.0,
        );
        draw_circle(local_emitter_pos.x(), local_emitter_pos.y(), 15.0, RED);
        flying_emitter_local.draw(local_emitter_pos);

        let world_emitter_pos = vec2(
            (get_time() * 0.5 + 1.0).sin() as f32 * screen_width() / 2.5 + screen_width() / 2.0,
            (get_time() * 0.2 + 1.0).cos() as f32 * screen_height() / 2.5 + screen_height() / 2.0,
        );

        draw_circle(world_emitter_pos.x(), world_emitter_pos.y(), 15.0, GREEN);
        flying_emitter_world.draw(world_emitter_pos);

        next_frame().await
    }
}
