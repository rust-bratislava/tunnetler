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
const PI: f64 = 3.14159265358979323;
const MOVE_STEP: f64 = 2.0;

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
        graphics::clear([0.0, 0.0, 0.0, 1.0], gl);

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
    use piston::input::{Button, Key};

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

    let mut players = [Player {
        position: Position {
            x: 100.0,
            y: 100.0,
        },
        direction: 0.0,
    }];

    let mut events = Events::new(EventSettings::new());

    let mut left_down = false;
    let mut right_down = false;
    let mut up_down = false;
    let mut down_down = false;
    let mut move_it = false;

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => render(r.viewport(), &mut gl, &players),
            Input::Press(Button::Keyboard(Key::Right)) => right_down = true,
            Input::Release(Button::Keyboard(Key::Right)) => right_down = false,
            Input::Press(Button::Keyboard(Key::Left)) => left_down = true,
            Input::Release(Button::Keyboard(Key::Left)) => left_down = false,
            Input::Press(Button::Keyboard(Key::Down)) => down_down = true,
            Input::Release(Button::Keyboard(Key::Down)) => down_down = false,
            Input::Press(Button::Keyboard(Key::Up)) => up_down = true,
            Input::Release(Button::Keyboard(Key::Up)) => up_down = false,

            Input::Update(_) if move_it => {
                players[0].position.x -= players[0].direction.sin() * MOVE_STEP;
                players[0].position.y += players[0].direction.cos() * MOVE_STEP;
            },
            _ => (),
        }

        move_it = true;
        match (left_down, right_down, up_down, down_down) {
            (false, false, false,  true) => players[0].direction = 0.0 / 180.0 * PI,
            (false, true,  false,  true) => players[0].direction = -45.0 / 180.0 * PI,
            (true,  false, false,  true) => players[0].direction = 45.0 / 180.0 * PI,
            (false, false, true,  false) => players[0].direction = 180.0 / 180.0 * PI,
            (false, true,  true,  false) => players[0].direction = -135.0 / 180.0 * PI,
            (true,  false, true,  false) => players[0].direction = 135.0 / 180.0 * PI,
            (false, true,  false, false) => players[0].direction = -90.0 / 180.0 * PI,
            (true,  false, false, false) => players[0].direction = 90.0 / 180.0 * PI,
            (_, _, _, _) => move_it = false, 
        }

    }
}
