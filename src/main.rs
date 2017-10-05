extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod turtle;
mod lsys;

use turtle::*;
use lsys::*;

//
// Since the buffers don't appear to transfer.
//
pub const NUM_BUFFERS: u8 = 2;

pub struct App {
    gl: GlGraphics,                 // OpenGL drawing backend.
    lsys: Lsys,                     // The magical lsys
    counter: u8,                    // Counter used to prevent flash.
    turtle: Turtle,                 // The turtle.
    todo: Option<TurtleMovement>    // The next thing to draw.
}

impl App {

    // A nice render function.
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Define the line to use as the step.
        let test = Line::new(color::WHITE, 0.5);

        // Increment frame counter.
        self.counter += 1;

        // If counter done, move to next command.
        if self.counter >= NUM_BUFFERS {
            // Reset counter.
            self.counter = 0;
            // Change command.
            self.todo = self.lsys.next().map(|c| self.turtle.eval(c));
        }

        // Fetch line.
        let todo = self.todo;

        // Get center.
        //let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        // Do gl draw.
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //clear(color::BLACK, gl);

            // Translate to center.
            //let transform = c.transform.trans(x, y);

            // Draw a box.
            if todo.is_some() {
                // Unwrap.
                let mov = todo.unwrap();
                // Only draw if movement is seen.
                if mov.draw {
                    // Draw line.
                    test.draw([mov.x_from, mov.y_from, mov.x_to, mov.y_to], &c.draw_state, c.transform, gl);
                }
            }

            // Draw a rectangle.
            //rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], c.transform, gl);

        });
    }

    /*
    fn update(&mut self, args: &UpdateArgs) {
        // Update the timer.
        self.timer = self.timer - args.dt;
    }
    */

}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Rust L-Systems",
            [640, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        lsys: Lsys::new(str_char_vec("f"), basic_grammar(), 10),
        turtle: Turtle::new(10.0, 10.0, EAST),
        counter: 0,
        todo: None
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        /*
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        */
    }
}
