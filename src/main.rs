extern crate pancurses;
extern crate rand;

extern crate text_io;

mod entities;
mod state;
mod tiling;
mod world;

use entities::{Entity, Player};
use pancurses::{endwin, initscr, noecho, Input};
use state::State;
use std::env;
use world::{Dungeon, DOWN, LEFT, RIGHT, UP};

fn get_player_name() -> String {
    match env::var_os("USER") {
        Some(val) => val.into_string().unwrap(),
        None => String::from("Kshar"),
    }
}

fn main() {
    let window = initscr();

    let mut state = State::new(
        Player::new(get_player_name(), String::from("Warrior"), 30, 10, 10, 20),
        Dungeon::new(
            window.get_max_x() as usize,
            (window.get_max_y() - 2) as usize,
            5,
        ),
    );

    state.init();

    window.keypad(true);
    noecho();

    state.render_level(&window);

    loop {
        // update
        state.render_entities(&window);

        state.render_player(&window);

        // get input and execute it
        match window.getch() {
            Some(Input::Character('?')) => {
                window.addstr("q: quit\n");
            }
            Some(Input::Character('j')) => {
                state.player.move_by(DOWN).unwrap();
                // state.get_player_mut().move_by(DOWN).unwrap();
            }
            Some(Input::Character('k')) => {
                state.get_player_mut().move_by(UP).unwrap();
            }
            Some(Input::Character('h')) => {
                state.get_player_mut().move_by(LEFT).unwrap();
            }
            Some(Input::Character('l')) => {
                state.get_player_mut().move_by(RIGHT).unwrap();
            }
            Some(Input::Character('q')) => break,
            Some(_) => (),
            None => (),
        }
        // actors actions (normally attack / interact if on same location as the character)
    }
    endwin();
}
