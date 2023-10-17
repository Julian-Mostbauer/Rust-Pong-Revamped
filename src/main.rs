use minifb::{Key, Window, WindowOptions};
use rand::Rng;

/* GLOBAL CONSTANTS---------------------------------------------------------------------------------------------------- */
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 360;
const _RED: u32 = 0xff0000;
const _GREEN: u32 = 0x00ff00;
const _BLUE: u32 = 0x0000ff;
const _BLACK: u32 = 0x000000;
const _WHITE: u32 = 0xffffff;

/* CODE FOR PLAYERS---------------------------------------------------------------------------------------------------*/
struct Player {
    position_x: u32,
    position_y: u32,

    speed: u32,

    width: u32,
    height: u32,

    score: u32,
}

impl Player {
    fn _set_y(&mut self, new_y: u32) {
        self.position_y = new_y;
    }

    fn init(&mut self, y: u32) {
        self._set_y(y);
        //self._set_score(0);
    }

    fn _set_score(&mut self, score: u32) {
        self.score = score;
    }
}

fn build_new_player(x: u32, y: u32) -> Player {
    let player = Player {
        position_x: x,
        position_y: y,

        speed: 10,

        height: 100,
        width: 10,
        score: 0,
    };
    return player;
}
/* CODE FOR BALL---------------------------------------------------------------------------------------------------*/
struct Ball {
    position_x: u32,
    position_y: u32,

    velocity_vec_norm: Box<[f64]>,
    speed: u32,
    width: u32,
    height: u32,
}

impl Ball {
    fn _set_x(&mut self, new_x: u32) {
        self.position_x = new_x;
    }
    fn _set_y(&mut self, new_y: u32) {
        self.position_y = new_y;
    }

    fn _get_velocity_x(&mut self) -> f64 {
        return self.velocity_vec_norm[0];
    }
    fn _get_velocity_y(&mut self) -> f64 {
        return self.velocity_vec_norm[1];
    }

    fn _set_velocity_x(&mut self, new_x: f64) {
        self.velocity_vec_norm[0] = new_x;
    }
    fn _set_velocity_y(&mut self, new_y: f64) {
        self.velocity_vec_norm[1] = new_y;
    }

    fn _normalize(&mut self) {
        let x_0 = self._get_velocity_x();
        let y_0 = self._get_velocity_y();
        self._set_velocity_x(x_0 / f64::max(x_0.abs(), y_0.abs()));
        self._set_velocity_y(y_0 / f64::max(x_0.abs(), y_0.abs()));
    }

    fn init(&mut self) {
        self._set_x(WINDOW_WIDTH as u32 / 2);
        self._set_x(WINDOW_HEIGHT as u32 / 2);
        self._set_velocity_x(rand::thread_rng().gen_range(-1.0..1.0));
        self._set_velocity_y(rand::thread_rng().gen_range(-1.0..1.0));
    }
}

fn build_new_ball(pos_x: u32, pos_y: u32, init_vel: Box<[f64]>) -> Ball {
    let ball = Ball {
        position_x: pos_x,
        position_y: pos_y,

        velocity_vec_norm: init_vel,
        speed: 5,
        width: 10,
        height: 10,
    };
    return ball;
}
/* MAIN FUNCTION-------------------------------------------------------------------------------------------------*/
fn main() {
    let mut player1 = build_new_player(1, (WINDOW_HEIGHT / 2) as u32);
    let mut player2 = build_new_player((WINDOW_WIDTH - 11) as u32, (WINDOW_HEIGHT / 2) as u32);
    let mut ball = build_new_ball(
        WINDOW_WIDTH as u32 / 2,
        WINDOW_HEIGHT as u32 / 2,
        Box::new([
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
        ]),
    );

    let mut game_over = initialise_game(&mut player1, &mut player2, &mut ball);

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
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut loop_count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !game_over {
            clear_buffer(&mut buffer);
            handle_input(&mut window, &mut player1, &mut player2);
            game_over = ball_physics(&mut ball, &mut player1, &mut player2);

            calc_image(&mut buffer, &player1, &player2, &ball);
        } else {
            println!("Game Over!");
            draw_game_over_screen(&mut buffer, &ball, loop_count);
        }
        if window.is_key_down(Key::R) {
            game_over = initialise_game(&mut player1, &mut player2, &mut ball);
        }
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        loop_count += 1;
    }
}

/* INPUT HANDLEING AND PHYSICS -----------------------------------------------------------------------------------------------*/
fn handle_input(window: &mut Window, player1: &mut Player, player2: &mut Player) {
    if window.is_key_down(Key::W) {
        move_if_valid(player1, "up");
    }
    if window.is_key_down(Key::S) {
        move_if_valid(player1, "down");
    }

    if window.is_key_down(Key::Up) {
        move_if_valid(player2, "up");
    }
    if window.is_key_down(Key::Down) {
        move_if_valid(player2, "down");
    }
}

fn move_if_valid(player: &mut Player, direction: &str) {
    let distance: u32;
    distance = player.speed;

    match direction {
        "up" => {
            if (player.position_y as i32) < (distance as i32) + 1 {
                player._set_y(1);
            } else {
                player._set_y(player.position_y - distance);
            }
        }
        "down" => {
            if player.position_y + distance + player.height > WINDOW_HEIGHT as u32 {
                player._set_y((WINDOW_HEIGHT as u32) - player.height);
            } else {
                player._set_y(player.position_y + distance);
            }
        }
        &_ => todo!(),
    }
}

fn ball_physics(ball: &mut Ball, player1: &mut Player, player2: &mut Player) -> bool {
    let x_0 = ball._get_velocity_x();
    let y_0 = ball._get_velocity_y();

    let x_dif: i32 = (x_0 * (ball.speed as f64)).ceil() as i32;
    let y_dif: i32 = (y_0 * (ball.speed as f64)).ceil() as i32;

    let x_new: u32 = (ball.position_x as i32 + x_dif) as u32;
    let y_new: u32 = (ball.position_y as i32 + y_dif) as u32;

    /* BOUNCE ON WALL */
    if (ball.position_x as i32) + x_dif < 0 {
        ball._set_velocity_x(-x_0);
        player1._set_score(player1.score + 1);
        return true;
    }

    if (ball.position_x as i32) + x_dif + (ball.width as i32) > WINDOW_WIDTH as i32 {
        ball._set_velocity_x(-x_0);
        player2._set_score(player2.score + 1);
        return true;
    }

    if ((ball.position_y as i32) + y_dif < 0)
        || ((ball.position_y as i32) + y_dif + (ball.height as i32) > WINDOW_HEIGHT as i32)
    {
        ball._set_velocity_y(-y_0);
        
        let current_x = ball._get_velocity_x();
        let current_y = ball._get_velocity_y();
        
        match rand::thread_rng().gen_range(1..=10) {
            1 => ball._set_velocity_x(current_x * 1.5),
            2 => ball._set_velocity_y(current_y * 1.5),
            _ => (),
        }
        ball._normalize();
        return false;
    }
    /* BOUNCE ON PLAYER */
    let inside_player_1_left = (ball.position_x as i32) + x_dif > player1.position_x as i32;
    let inside_player_1_right =
        (ball.position_x as i32) + x_dif < (player1.position_x + player1.width) as i32;

    let inside_player_1_top = (ball.position_y as i32) + y_dif > player1.position_y as i32;
    let inside_player_1_bottom =
        (ball.position_y as i32) + y_dif < (player1.position_y + player1.height) as i32;

    let inside_player_1 = (inside_player_1_left && inside_player_1_right)
        && (inside_player_1_bottom && inside_player_1_top);

    let inside_player_2_left =
        (ball.position_x as i32) + x_dif + (ball.width as i32) > player2.position_x as i32;
    let inside_player_2_right = (ball.position_x as i32) + x_dif + (ball.width as i32)
        < (player2.position_x + player2.width) as i32;

    let inside_player_2_top = (ball.position_y as i32) + y_dif > player2.position_y as i32;
    let inside_player_2_bottom =
        (ball.position_y as i32) + y_dif < (player2.position_y + player2.height) as i32;

    let inside_player_2 = (inside_player_2_left && inside_player_2_right)
        && (inside_player_2_bottom && inside_player_2_top);

    if inside_player_1 || inside_player_2 {
        println!("Collision!");
        ball._set_velocity_x(-x_0);
        return false;
    }

    ball._set_x(x_new);
    ball._set_y(y_new);
    return false;
}

/* DRAWING TO THE BUFFER---------------------------------------------------------------------------------------------------*/
fn calc_image(buffer: &mut Vec<u32>, player1: &Player, player2: &Player, ball: &Ball) {
    draw_player(buffer, &player1);
    draw_player(buffer, &player2);
    draw_ball(buffer, &ball);
    draw_score(buffer, &player1, &player2);
}

fn draw_player(buffer: &mut Vec<u32>, player: &Player) {
    for x in 0..player.width {
        for y in 0..player.height {
            draw_pixel(buffer, player.position_x + x, player.position_y + y, _WHITE);
        }
    }
}

fn draw_ball(buffer: &mut Vec<u32>, ball: &Ball) {
    for x in 0..ball.width {
        for y in 0..ball.height {
            draw_pixel(buffer, ball.position_x + x, ball.position_y + y, _WHITE);
        }
    }
}

fn draw_game_over_screen(buffer: &mut Vec<u32>, ball: &Ball, loop_counter: u32) {
    clear_buffer(buffer);
    draw_ball(buffer, ball);

    if loop_counter % 24 == 0
        || (loop_counter + 1) % 24 == 0
        || (loop_counter + 2) % 24 == 0
        || (loop_counter + 3) % 24 == 0
        || (loop_counter + 4) % 24 == 0
    {
        for i in buffer.iter_mut() {
            *i = _RED;
        }
    }
}

fn draw_score(buffer: &mut Vec<u32>, player1: &Player, player2: &Player) {
    for y in 0..=player2.score * 10 {
        for x in 0..10 {
            draw_pixel(buffer, WINDOW_WIDTH as u32 / 2 - 1 - x, y, _WHITE);
        }
    }
    for y in 0..=player1.score * 10 {
        for x in 0..10 {
            draw_pixel(buffer, WINDOW_WIDTH as u32 / 2 + 1 + x, y, _WHITE);
        }
    }
}

fn draw_pixel(buffer: &mut Vec<u32>, x: u32, y: u32, color: u32) {
    let pixel: usize = (x + ((WINDOW_WIDTH as u32) * y)) as usize;
    let _ = buffer[pixel] = color;
}

fn clear_buffer(buffer: &mut Vec<u32>) {
    for i in buffer.iter_mut() {
        *i = _BLACK;
    }
}

/*GAMESTATE HANDLEING */

fn initialise_game(player1: &mut Player, player2: &mut Player, ball: &mut Ball) -> bool {
    player1.init(WINDOW_HEIGHT as u32 / 2);
    player2.init(WINDOW_HEIGHT as u32 / 2);
    ball.init();
    ball._normalize();

    return false;
}
