extern crate piston_window;
extern crate rand;


mod draw;
mod snake;
mod game;

use crate::draw::to_coord_u32;
use crate::game::Game;

use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5,0.5,0.5,0.8];


fn main() {
    let (w, h) = (30,30);

    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(w), to_coord_u32(h)])
        .exit_on_esc(true)
        .build()
        .unwrap();
    
        let mut game: Game = Game::new(w,h);
        
        while let Some(event) = window.next() {

            if let Some(Button::Keyboard(key)) = event.press_args() {
                game.key_pressed(key);
            }

            window.draw_2d(&event, |c, g, _| {
                clear(BACK_COLOR, g);
                game.draw(&c, g);
            });

            event.update(|arg| {
                game.update(arg.dt);
            });
        }
}
