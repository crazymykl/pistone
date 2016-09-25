extern crate piston_window;

use piston_window::*;

fn main() {

    let mut window: PistonWindow = WindowSettings::new("Pistone", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut pos = (50.0, 50.0);
    let mut delta = (0.0, 0.0);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                rectangle([1.0, 0.0, 1.0, 0.7], [pos.0, pos.1, 100.0, 100.0], c.transform, g);
            });
        }

        if let Some(_) = e.update_args() {
            pos.0 += delta.0;
            pos.1 += delta.1;
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up)    => delta.1 = -1.0,
                Button::Keyboard(Key::Down)  => delta.1 =  1.0,
                Button::Keyboard(Key::Left)  => delta.0 = -1.0,
                Button::Keyboard(Key::Right) => delta.0 =  1.0,
                Button::Keyboard(Key::Space) => delta   = (0.0, 0.0),
                _ => ()
            }
        }
    };
}
