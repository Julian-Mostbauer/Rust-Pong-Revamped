pub mod objects {
    use rand::Rng;

    use crate::game::constants::*;

    pub struct Player {
        pub position_x: u32,
        pub position_y: u32,

        pub speed: u32,

        pub width: u32,
        pub height: u32,

        pub score: u32,
    }

    impl Player {
        pub fn set_y(&mut self, new_y: u32) {
            self.position_y = new_y;
        }
        pub fn get_y(&self) -> u32 {
            self.position_y
        }
        pub fn set_x(&mut self, new_x: u32) {
            self.position_x = new_x;
        }

        pub fn get_x(&self) -> u32 {
            self.position_x
        }

        pub fn get_speed(&self) -> u32 {
            self.speed
        }

        pub fn init(&mut self, y: u32) {
            self.set_y(y);
        }

        pub fn set_score(&mut self, score: u32) {
            self.score = score;
        }
        pub fn get_score(&self) -> u32 {
            self.score
        }

        pub fn set_height(&mut self, new_height: u32) {
            self.height = new_height;
        }
        pub fn get_height(&self) -> u32 {
            self.height
        }
        pub fn set_width(&mut self, new_width: u32) {
            self.width = new_width;
        }
        pub fn get_width(&self) -> u32 {
            self.width
        }
    }

    pub fn build_new_player(x: u32, y: u32) -> Player {
        Player {
            position_x: x,
            position_y: y,

            speed: 10,

            height: 100,
            width: 10,
            score: 0,
        }
    }

    /* CODE FOR BALL---------------------------------------------------------------------------------------------------*/
    pub struct Ball {
        pub position_x: u32,
        pub position_y: u32,

        pub velocity_vec_norm: Box<[f64]>,
        pub speed: f32,
        pub width: u32,
        pub height: u32,

        pub history_pos_x: Vec<u32>,
        pub history_pos_y: Vec<u32>,
    }

    impl Ball {
        pub fn set_x(&mut self, new_x: u32) {
            self.position_x = new_x;
        }
        pub fn set_y(&mut self, new_y: u32) {
            self.position_y = new_y;
        }
        pub fn get_x(&self) -> u32 {
            self.position_x
        }
        pub fn get_y(&self) -> u32 {
            self.position_y
        }

        pub fn get_width(&self) -> u32 {
            self.width
        }
        pub fn get_height(&self) -> u32 {
            self.height
        }

        pub fn get_velocity_x(&mut self) -> f64 {
            self.velocity_vec_norm[0]
        }
        pub fn get_velocity_y(&mut self) -> f64 {
            self.velocity_vec_norm[1]
        }

        pub fn set_velocity_x(&mut self, new_x: f64) {
            self.velocity_vec_norm[0] = new_x;
        }
        pub fn set_velocity_y(&mut self, new_y: f64) {
            self.velocity_vec_norm[1] = new_y;
        }

        pub fn normalize(&mut self) {
            let x_0 = self.get_velocity_x();
            let y_0 = self.get_velocity_y();

            // self.set_velocity_x(x_0 / f64::max(x_0.abs(), y_0.abs()));
            // self.set_velocity_y(y_0 / f64::max(x_0.abs(), y_0.abs()));

            self.set_velocity_x(x_0 % 1.0);
            self.set_velocity_y(y_0 % 1.0);
        }

        pub fn init(&mut self) {
            self.set_x(WINDOW_WIDTH as u32 / 2);
            self.set_y(WINDOW_HEIGHT as u32 / 2);
            self.set_velocity_x(rand::thread_rng().gen_range(-1.0..1.0));
            self.set_velocity_y(rand::thread_rng().gen_range(-1.0..1.0));
        }

        pub fn clear_history(&mut self) {
            self.history_pos_x = Vec::new();
            self.history_pos_y = Vec::new();
        }

        pub fn add_to_history_x(&mut self, new_x: u32) {
            self.history_pos_x.push(new_x)
        }
        pub fn add_to_history_y(&mut self, new_y: u32) {
            self.history_pos_y.push(new_y)
        }

        pub fn get_speed(&self) -> f32 {
            self.speed
        }
        pub fn set_speed(&mut self, newspeed: f32) {
            self.speed = newspeed;
        }

        pub fn get_history_x(&self) -> Vec<u32> {
            self.history_pos_x.clone()
        }
        pub fn get_history_y(&self) -> Vec<u32> {
            self.history_pos_y.clone()
        }
    }

    pub fn build_new_ball(pos_x: u32, pos_y: u32, init_vel: Box<[f64]>, fps: f32) -> Ball {
        Ball {
            position_x: pos_x,
            position_y: pos_y,

            velocity_vec_norm: init_vel,
            speed: fps / 30.0,
            width: 10,
            height: 10,
            history_pos_x: Vec::new(),
            history_pos_y: Vec::new(),
        }
    }
}

pub mod physics_visuals {
    use crate::game::constants::*;
    use crate::game::objects::*;
    use minifb::{Key, Window, WindowOptions};
    use std::cmp::Ordering;

    pub fn handle_input(
        window: &mut Window,
        player1: &mut Player,
        player2: &mut Player,
        ball: &mut Ball,
        play_against_bot: bool,
        loop_counter: u32,
    ) {
        if window.is_key_down(Key::W) {
            move_if_valid(player1, "up");
        }
        if window.is_key_down(Key::S) {
            move_if_valid(player1, "down");
        }
        /*Player 2 Movement */
        if !play_against_bot {
            if window.is_key_down(Key::Up) {
                move_if_valid(player2, "up");
            }
            if window.is_key_down(Key::Down) {
                move_if_valid(player2, "down");
            }
        } else {
            let player_y = player2.get_y() + player2.get_height() / 2;
            match ball.get_y().cmp(&player_y) {
                Ordering::Less => move_if_valid(player2, "up"),
                Ordering::Greater => move_if_valid(player2, "down"),
                Ordering::Equal => (),
            }
        }

        if window.is_key_down(Key::I) {
            println!(
                "--------------------FRAME {}--------------------------",
                loop_counter
            );
            println!("Player 1: ");
            println!("Position = ({}, {})", player1.get_x(), player1.get_y());

            println!("Player 2: ");
            println!("Position = ({}, {})", player2.get_x(), player2.get_y());

            println!("Ball: ");
            println!("Position = ({},{})", ball.get_x(), ball.get_y());
            println!(
                "Speed Vector = ({},{})",
                ball.get_velocity_x() * ball.get_speed() as f64,
                ball.get_velocity_y() * ball.get_speed() as f64
            );
            println!(
                "Normalized Speed Vector = ({},{})",
                ball.get_velocity_x(),
                ball.get_velocity_y()
            );
            println!("Trace lenght = {}", ball.get_history_x().len());
        }
    }

    pub fn move_if_valid(player: &mut Player, direction: &str) {
        let distance = player.get_speed();

        match direction {
            "up" => {
                if (player.get_y() as i32) < (distance as i32) + 1 {
                    player.set_y(1);
                } else {
                    player.set_y(player.get_y() - distance);
                }
            }
            "down" => {
                if player.get_y() + distance + player.get_height() > WINDOW_HEIGHT as u32 {
                    player.set_y((WINDOW_HEIGHT as u32) - player.get_height());
                } else {
                    player.set_y(player.get_y() + distance);
                }
            }
            &_ => todo!(),
        }
    }

    pub fn ball_physics(ball: &mut Ball, player1: &mut Player, player2: &mut Player) -> bool {
        let x_0 = ball.get_velocity_x();
        let y_0 = ball.get_velocity_y();

        let x_dif: i32 = (x_0 * (ball.get_speed() as f64)).ceil() as i32;
        let y_dif: i32 = (y_0 * (ball.get_speed() as f64)).ceil() as i32;

        let x_new: u32 = (ball.get_x() as i32 + x_dif) as u32;
        let y_new: u32 = (ball.get_y() as i32 + y_dif) as u32;

        /* BOUNCE ON WALL */
        if (ball.get_x() as i32) + x_dif < 0 {
            ball.set_velocity_x(-x_0);
            player1.set_score(player1.get_score() + 1);
            return true;
        }

        if (ball.get_x() as i32) + x_dif + (ball.get_width() as i32) > WINDOW_WIDTH as i32 {
            ball.set_velocity_x(-x_0);
            player2.set_score(player2.get_score() + 1);
            return true;
        }

        if ((ball.get_y() as i32) + y_dif < 0)
            || ((ball.get_y() as i32) + y_dif + (ball.get_height() as i32) > WINDOW_HEIGHT as i32)
        {
            ball.set_velocity_y(-y_0);

            let current_x = ball.get_velocity_x();
            let current_y = ball.get_velocity_y();

            match rand::Rng::gen_range(&mut rand::thread_rng(), 1..=10) {
                1 => ball.set_velocity_x(current_x * 1.5),
                2 => ball.set_velocity_y(current_y * 1.5),
                _ => (),
            }
            ball.normalize();
            return false;
        }
        /* BOUNCE ON PLAYER */
        let inside_player_1_left = (ball.get_x() as i32) + x_dif > player1.get_x() as i32;
        let inside_player_1_right =
            (ball.get_x() as i32) + x_dif < (player1.get_x() + player1.get_width()) as i32;

        let inside_player_1_top = (ball.get_y() as i32) + y_dif > player1.get_y() as i32;
        let inside_player_1_bottom =
            (ball.get_y() as i32) + y_dif < (player1.get_y() + player1.get_height()) as i32;

        let inside_player_1 = (inside_player_1_left && inside_player_1_right)
            && (inside_player_1_bottom && inside_player_1_top);

        let inside_player_2_left =
            (ball.get_x() as i32) + x_dif + (ball.get_width() as i32) > player2.get_x() as i32;
        let inside_player_2_right = (ball.get_x() as i32) + x_dif + (ball.get_width() as i32)
            < (player2.get_x() + player2.get_width()) as i32;

        let inside_player_2_top = (ball.get_y() as i32) + y_dif > player2.get_y() as i32;
        let inside_player_2_bottom =
            (ball.get_y() as i32) + y_dif < (player2.get_y() + player2.get_height()) as i32;

        let inside_player_2 = (inside_player_2_left && inside_player_2_right)
            && (inside_player_2_bottom && inside_player_2_top);

        if inside_player_1 || inside_player_2 {
            println!("Collision!");
            ball.set_velocity_x(-x_0);
            return false;
        }

        /*BOUNCE ON SCORE */

        /* */

        ball.set_x(x_new);
        ball.set_y(y_new);
        ball.add_to_history_x(x_new);
        ball.add_to_history_y(y_new);
        false
    }

    /* DRAWING TO THE BUFFER---------------------------------------------------------------------------------------------------*/
    pub fn calc_image(
        buffer: &mut [u32],
        player1: &Player,
        player2: &Player,
        ball: &Ball,
        loop_counter: u32,
    ) {
        draw_player(buffer, player1, loop_counter);
        draw_player(buffer, player2, loop_counter);
        draw_ball(buffer, ball, loop_counter);
        draw_score(buffer, player1, player2);
        draw_trace(buffer, ball);
    }

    pub fn draw_player(buffer: &mut [u32], player: &Player, loop_counter: u32) {
        let player_x = player.get_x();
        let player_y = player.get_y();
        for x in 0..player.get_width() {
            for y in 0..player.get_height() {
                draw_pixel(buffer, player_x + x, player_y + y, WHITE);
            }
        }
    }

    pub fn draw_ball(buffer: &mut [u32], ball: &Ball, loop_counter: u32) {
        for x in 0..ball.get_width() {
            for y in 0..ball.get_height() {
                draw_pixel(
                    buffer,
                    ball.get_x() + x,
                    ball.get_y() + y,
                    loop_counter + RED,
                );
            }
        }
    }

    pub fn draw_game_over_screen(buffer: &mut [u32], ball: &Ball, loop_counter: u32) {
        clear_buffer(buffer);
        draw_ball(buffer, ball, loop_counter);

        if loop_counter % 24 == 0
            || (loop_counter + 1) % 24 == 0
            || (loop_counter + 2) % 24 == 0
            || (loop_counter + 3) % 24 == 0
            || (loop_counter + 4) % 24 == 0
            || (loop_counter + 5) % 24 == 0
        {
            for i in buffer.iter_mut() {
                *i = RED;
            }
        }
    }

    pub fn draw_trace(buffer: &mut [u32], ball: &Ball) {
        let history_x = ball.get_history_x();
        let history_y = ball.get_history_y();
        for i in 0..history_x.len() {
            draw_pixel(buffer, history_x[i], history_y[i], RED + (i as u32 * 0x100));
        }
    }

    pub fn draw_score(buffer: &mut [u32], player1: &Player, player2: &Player) {
        for y in 0..=player2.get_score() * 10 {
            for x in 0..10 {
                draw_pixel(buffer, WINDOW_WIDTH as u32 / 2 - 1 - x, y, WHITE);
            }
        }
        for y in 0..=player1.get_score() * 10 {
            for x in 0..10 {
                draw_pixel(buffer, WINDOW_WIDTH as u32 / 2 + 1 + x, y, WHITE);
            }
        }
    }

    pub fn draw_pixel(buffer: &mut [u32], x: u32, y: u32, color: u32) {
        let pixel: usize = (x + ((WINDOW_WIDTH as u32) * y)) as usize;
        buffer[pixel] = color;
    }

    pub fn clear_buffer(buffer: &mut [u32]) {
        for i in buffer.iter_mut() {
            *i = BLACK;
        }
    }

    /*GAMESTATE HANDLEING */

    pub fn initialise_game(player1: &mut Player, player2: &mut Player, ball: &mut Ball) -> bool {
        player1.init(WINDOW_HEIGHT as u32 / 2);
        player2.init(WINDOW_HEIGHT as u32 / 2);
        ball.init();
        ball.normalize();

        false
    }
}

pub mod constants {
    pub const WINDOW_WIDTH: usize = 640;
    pub const WINDOW_HEIGHT: usize = 360;
    pub const RED: u32 = 0xff0000;
    pub const GREEN: u32 = 0x00ff00;
    pub const BLUE: u32 = 0x0000ff;
    pub const BLACK: u32 = 0x000000;
    pub const WHITE: u32 = 0xffffff;
}
