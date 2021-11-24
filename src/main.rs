extern crate graphics;
extern crate piston;
extern crate piston_window;

use piston::window::WindowSettings;
use piston_window::*;

const WINDOW_X: u32 = 1024;
const WINDOW_Y: u32 = 1024;
const MAX_ITER: u32 = 255;

fn sqr(x: f32) -> f32 {
    x*x
}

fn iterate(c: (f32, f32)) -> u32 {
    let mut z = (0.0, 0.0);
    let mut iter = 0;

    while z.0*z.0 + z.1*z.1 < 4.0 && iter < MAX_ITER {
        let temp = sqr(z.0) - sqr(z.1) + c.0;
        z.1 = (2.0*z.0*z.1).abs() + c.1;
        z.0 = temp;
        iter += 1;
    }

    iter
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Mandelbrot",
        [WINDOW_X, WINDOW_Y])
        .exit_on_esc(true)
        .build()
        .unwrap();


    while let Some(e) = window.next() {
        window.draw_2d(&e, |b, g, _| {
            clear([0.1, 0.1, 0.1, 1.0], g);

            for i in 0..WINDOW_X {
                for j in 0..WINDOW_Y {
                    let coords = (
                        (i as f32 - WINDOW_X as f32 / 2.0) / (WINDOW_Y as f32 / 4.0),
                        (j as f32 - WINDOW_Y as f32 / 2.0) / (WINDOW_Y as f32 / 4.0),
                    );
                    let iters = iterate(coords);

                    rectangle(
                        [0.3, 0.3, 0.7, iters as f32 / MAX_ITER as f32],
                        rectangle::square(i.into(), j.into(), 1.0),
                        b.transform,
                        g
                    );
                }
            }
        });
    }
}
