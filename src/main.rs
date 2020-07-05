extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut x = 0.0;
    let mut y = 0.0;
    let size = 64.0;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [x - size / 2.0, y - size / 2.0, size, size],
                context.transform,
                graphics,
            );
        });

        if let Some(cursor) = event.mouse_cursor_args() {
            x = cursor[0];
            y = cursor[1];
        }
    }
}
