extern crate piston;         
extern crate graphics;       
extern crate glutin_window;  
extern crate opengl_graphics;
extern crate rand;           
extern crate num_traits;
extern crate viewport;

use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::context::Context as GraphicsContext;
use num_traits::Num;
use viewport::Viewport;

const POINT_SIZE: f64 = 5.0;

struct Position<T: Num + Into<f64>> {
    x: T,
    y: T
}

struct Player<T: Num + Into<f64>> {
    position: Position<T>,
    direction: f64,
}

impl<T: Num + Into<f64> + Copy> Player<T> {
    pub fn draw(&self, ctx: GraphicsContext, gl: &mut GlGraphics) {
        use graphics::rectangle;
        use graphics::Transformed;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0/256.0*(0xB4 as f32), 1.0];
        const LIGHT_BLUE: [f32; 4] = [1.0/256.0*(0x2C as f32), 1.0/256.0*(0x2C as f32), 1.0/256.0*(0xFC as f32), 1.0];
        const YELLOW: [f32; 4] = [1.0/256.0*(0xF4 as f32), 1.0/256.0*(0xEA as f32), 1.0/256.0*(0x1B as f32), 1.0];

        let (x, y) = (self.position.x.into(), self.position.y.into());
        let transform = ctx.transform.trans(x, y)
            .rot_rad(self.direction)
            .trans(-2.5 * POINT_SIZE, -3.5 * POINT_SIZE);

        let left_side = [0.0, 0.0, POINT_SIZE, POINT_SIZE * 6.0];
        let right_side = [POINT_SIZE * 4.0, 0.0, POINT_SIZE, POINT_SIZE * 6.0];
        let center = [POINT_SIZE, POINT_SIZE, POINT_SIZE * 3.0, POINT_SIZE * 4.0];
        let canon = [POINT_SIZE * 2.0, POINT_SIZE * 2.5, POINT_SIZE, POINT_SIZE * 4.0];

        rectangle(BLUE, left_side, transform, gl);
        rectangle(BLUE, right_side, transform, gl);
        rectangle(LIGHT_BLUE, center, transform, gl);
        rectangle(YELLOW, canon, transform, gl);
    }
}

fn render<T: Num + Into<f64> + Copy>(viewport: Viewport, gl: &mut GlGraphics, players: &[Player<T>]) {
    gl.draw(viewport, |c, gl| {
        for player in players {
            player.draw(c, gl);
        }
    });
}

fn main() {
    use piston::window::WindowSettings;
    use piston::input::Input;
    use piston::event_loop::{Events, EventSettings};
    use glutin_window::GlutinWindow as Window;

    let opengl = OpenGL::V3_2;

    let width = 800;
    let height = 600;

    let mut window: Window = WindowSettings::new(
            "Tunnetler",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let players = [Player {
        position: Position {
            x: 100.0,
            y: 100.0,
        },
        direction: 0.0,
    }];

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => render(r.viewport(), &mut gl, &players),
            _ => (),
        }
    }
}
