extern crate piston_window;
use piston_window::*;
use crate::app::App;

mod cell;
mod direction;
mod dimensions;
mod snake;
mod game;
mod app;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [510, 510])
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut app = App::new();

    while let Some(event) = window.next() {
        event.update(|args| app.update(args.dt));
        event.press(|button| app.press(button));
        window.draw_2d(&event, |context, graphics| app.render(context, graphics));
    }

    println!("Record: {}", app.record);
}