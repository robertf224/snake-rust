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
        WindowSettings::new("Snake", [520, 580])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // We need to provide and load a font -- Piston can't render text otherwise
    let texture_settings = TextureSettings::new();
    let raw_font = include_bytes!("../assets/FiraSans-Regular.ttf");
    let font = Glyphs::from_bytes(raw_font, window.factory.clone(), texture_settings);
    let font = match font {
        Ok(font) => font,
        Err(error) => panic!("Failed to load font: {:?}", error)
    };

    let mut app = App::new(font);

    while let Some(event) = window.next() {
        event.update(|args| app.update(args.dt));
        event.press(|button| app.press(button));
        window.draw_2d(&event, |context, graphics| app.render(context, graphics));
    }
}