mod game;
use game::constants::*;
use game::objects::*;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

fn main() {
    let fps: f64 = 60.0;
    let mut play_against_bot = false;
    let mut player1 = build_new_player(1, (WINDOW_HEIGHT / 2) as u32);
    let mut player2 = build_new_player((WINDOW_WIDTH - 11) as u32, (WINDOW_HEIGHT / 2) as u32);
    let mut ball = build_new_ball(
        WINDOW_WIDTH as u32 / 2,
        WINDOW_HEIGHT as u32 / 2,
        Box::new([
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0),
        ]),
        fps as f32,
    );

    let mut game_over =
        crate::game::physics_visuals::initialise_game(&mut player1, &mut player2, &mut ball);

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut window = Window::new(
        "P0NG - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(
        (1.0 / fps * 1_000_000.0) as u64,
    )));

    let mut loop_count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !game_over {
            game::physics_visuals::clear_buffer(&mut buffer);
            game::physics_visuals::handle_input(
                &mut window,
                &mut player1,
                &mut player2,
                &mut ball,
                play_against_bot,
                loop_count,
            );
            game_over = game::physics_visuals::ball_physics(&mut ball, &mut player1, &mut player2);

            game::physics_visuals::calc_image(&mut buffer, &player1, &player2, &ball, loop_count);
        } else {
            //println!("Game Over!");
            game::physics_visuals::draw_game_over_screen(&mut buffer, &ball, loop_count);
        }

        if window.is_key_down(Key::R) {
            game_over =
                game::physics_visuals::initialise_game(&mut player1, &mut player2, &mut ball);
        }
        if window.is_key_down(Key::B) {
            play_against_bot = !play_against_bot;
        }

        ball.set_speed(ball.get_speed() + (fps / (1 + loop_count * 1_000_000) as f64) as f32);
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        loop_count += 1;
    }
}
