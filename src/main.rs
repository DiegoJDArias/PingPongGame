/* ////////////////////////////////////////////////
  Author Diego J D Arias - diegojdarias@gmail.com.
*////////////////////////////////////////////////

mod game;

use macroquad::prelude::*;
use crate::game::Game;


#[macroquad::main("Ping Pong by Diego J D Arias")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update();
        game.draw();

        next_frame().await
    }
}
