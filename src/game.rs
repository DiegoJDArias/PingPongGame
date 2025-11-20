/* ////////////////////////////////////////////////
  Author Diego J D Arias - diegojdarias@gmail.com.
*////////////////////////////////////////////////

use macroquad::audio::{load_sound, play_sound_once, Sound};
use macroquad::color::{Color, BLACK, GRAY, WHITE, YELLOW};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::prelude::{clear_background, draw_circle, draw_line, draw_rectangle, draw_text, rand, screen_height, screen_width};
use macroquad::shapes::{draw_circle_lines, draw_rectangle_lines};

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const BALL_SIZE: f32 = 15.0;
pub const PADDLE_SPEED: f32 = 7.0;
pub const BALL_SPEED: f32 = 5.0;

pub struct Game {
    paddle_left: Rect,
    paddle_right: Rect,
    ball: Rect,
    ball_velocity: Vec2,
    score_left: i32,
    score_right: i32,
    sound_hit: Sound,
    sound_score: Sound,
    game_started: bool,
}

impl Game {
    pub async fn new() -> Self {
        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;
        let sound_hit = load_sound("Sound/pong.ogg").await.unwrap();
        let sound_score = load_sound("Sound/score.ogg").await.unwrap();

        Self {
            paddle_left: Rect::new(
                20.0,
                screen_center_y - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT
            ),
            paddle_right: Rect::new(
                screen_width() - 40.0,
                screen_center_y - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT
            ),
            ball: Rect::new(
                screen_center_x - BALL_SIZE / 2.0,
                screen_center_y - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE
            ),
            ball_velocity: vec2(0.0, 0.0),
            score_left: 0,
            score_right: 0,
            sound_hit,
            sound_score,
            game_started: false,
        }
    }

    pub fn update(&mut self) {
        if !self.game_started && is_key_pressed(KeyCode::Enter) {
            self.game_started = true;
            self.ball_velocity = vec2(
                if rand::gen_range(0, 2) == 0 { BALL_SPEED } else { -BALL_SPEED },
                if rand::gen_range(0, 2) == 0 { BALL_SPEED } else { -BALL_SPEED }
            );
        }

        if is_key_down(KeyCode::W) && self.paddle_left.y > 0.0 {
            self.paddle_left.y -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::S) && self.paddle_left.y < screen_height() - PADDLE_HEIGHT {
            self.paddle_left.y += PADDLE_SPEED;
        }

        if is_key_down(KeyCode::Up) && self.paddle_right.y > 0.0 {
            self.paddle_right.y -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Down) && self.paddle_right.y < screen_height() - PADDLE_HEIGHT {
            self.paddle_right.y += PADDLE_SPEED;
        }

        if self.game_started {
            self.ball.x += self.ball_velocity.x;
            self.ball.y += self.ball_velocity.y;

            if self.ball.y <= 0.0 || self.ball.y >= screen_height() - BALL_SIZE {
                self.ball_velocity.y = -self.ball_velocity.y;
            }

            if self.ball.overlaps(&self.paddle_left) && self.ball_velocity.x < 0.0 {
                play_sound_once(&self.sound_hit);
                self.ball_velocity.x = -self.ball_velocity.x;
                let hit_position = (self.ball.center().y - self.paddle_left.center().y) / (PADDLE_HEIGHT / 2.0);
                self.ball_velocity.y = hit_position * BALL_SPEED;
            }

            if self.ball.overlaps(&self.paddle_right) && self.ball_velocity.x > 0.0 {
                play_sound_once(&self.sound_hit);
                self.ball_velocity.x = -self.ball_velocity.x;
                let hit_position = (self.ball.center().y - self.paddle_right.center().y) / (PADDLE_HEIGHT / 2.0);
                self.ball_velocity.y = hit_position * BALL_SPEED;
            }

            if self.ball.x < 0.0 {
                play_sound_once(&self.sound_score);
                self.score_right += 1;
                self.reset_ball();
            }
            if self.ball.x > screen_width() {
                play_sound_once(&self.sound_score);
                self.score_left += 1;
                self.reset_ball();
            }
        }
    }

    pub fn reset_ball(&mut self) {
        self.ball.x = screen_width() / 2.0 - BALL_SIZE / 2.0;
        self.ball.y = screen_height() / 2.0 - BALL_SIZE / 2.0;
        self.ball_velocity = vec2(0.0, 0.0);
        self.game_started = false;
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        draw_line(
            screen_width() / 2.0,
            0.0,
            screen_width() / 2.0,
            screen_height(),
            2.0,
            GRAY
        );

        draw_rectangle(
            self.paddle_left.x,
            self.paddle_left.y,
            self.paddle_left.w,
            self.paddle_left.h,
            WHITE
        );

        draw_rectangle(
            self.paddle_right.x,
            self.paddle_right.y,
            self.paddle_right.w,
            self.paddle_right.h,
            WHITE
        );

        draw_circle_lines(
            screen_width() / 2.0,
            screen_height() / 2.0,
            50.0,
            2.0,
            Color::new(1.0, 1.0, 1.0, 0.5)
        );

        draw_rectangle_lines(
            1.0,
            1.0,
            screen_width() - 1.0,
            screen_height() - 1.0,2.0,
            Color::new(1.0, 1.0, 1.0, 0.5)
        );

        draw_circle(
            self.ball.x + self.ball.w / 2.0,
            self.ball.y + self.ball.h / 2.0,
            self.ball.w / 2.0,
            WHITE
        );

        draw_text(
            &format!("{}", self.score_left),
            screen_width() / 4.0,
            50.0,
            40.0,
            WHITE
        );

        draw_text(
            &format!("{}", self.score_right),
            3.0 * screen_width() / 4.0,
            50.0,
            40.0,
            WHITE
        );

        if !self.game_started {
            draw_text(
                "PRESIONA ENTER PARA COMENZAR",
                screen_width() / 2.0 - 160.0,
                35.0,
                25.0,
                YELLOW
            );
        }
    }
}