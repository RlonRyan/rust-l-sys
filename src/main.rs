extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::HashMap;

mod turtle;

use turtle::*;

pub const STEP_DELAY: f64 = 0.02;
pub const MAX_DEPTH: i32 = 3;

pub struct App {
    gl: GlGraphics,         // OpenGL drawing backend.
    commands: Vec<char>,    // Vector containing commands.
    timer: f64,             // Timer used to animate.
    turtle: Turtle,         // The turtle.
    todo: Vec<TurtleMovement>,
    grammar: HashMap<char, String>,
    depth: i32
}

fn basic_grammar() -> HashMap<char, String> {
    let mut grammar = HashMap::new();
    grammar.insert('f', "f+f-f-f+f".to_string());
    return grammar;
}

impl App {

    // A nice render function.
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Define the line to use as the step.
        let test = Line::new(color::WHITE, 0.5);

        // Fetch line.
        let todo = self.todo.last().cloned();

        // Get center.
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        // Do gl draw.
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //clear(color::BLACK, gl);

            // Translate to center.
            let transform = c.transform.trans(x, y);

            // Draw a box.
            if todo.is_some() {
                // Unwrap.
                let mov = todo.unwrap();
                // Only draw if movement is seen.
                if mov.draw {
                    // Draw line.
                    test.draw([mov.x_from, mov.y_from, mov.x_to, mov.y_to], &c.draw_state, transform, gl);
                }
            }

            // Draw a rectangle.
            //rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], c.transform, gl);

        });
    }

    fn expand(&mut self) -> Option<char> {
        // Loop
        loop {
            // Fetch next
            let next = self.commands.pop();
            // If no next abort.
            if next.is_none() {
                // Abort!
                return next;
            }
            // Unwrap next.
            let c = next.unwrap();
            // If next is pop character, pop.
            if c == '\0' {
                // Pop
                self.depth -= 1;
                continue;
            }
            // If at max depth, abort.
            if self.depth == MAX_DEPTH {
                // Abort
                return next;
            }
            // Attempt to find expansion.
            let expansion = self.grammar.get(&c);
            // Expand if possible.
            if expansion.is_some() {
                // Increase depth.
                self.depth += 1;
                // Push pop token.
                self.commands.push('\0');
                // Push expansion.
                for e in expansion.unwrap().chars() {
                    self.commands.push(e);
                }
            } else {
                // Abort!
                return next;
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Update the timer.
        self.timer = self.timer - args.dt;
        // If timer out...
        if self.timer <= 0.0 {
            // Reset timer.
            self.timer = STEP_DELAY;
            // Expand
            let command = self.expand();
            // Run Command if needed.
            if command.is_some() {
                // Unwrap
                let c = command.unwrap();
                // Add to turtle queue.
                self.todo.pop();
                let mov = self.turtle.eval(c);
                self.todo.push(mov);
            }
        }
    }

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
        commands: vec!['f'],
        turtle: Turtle::new(0.0, 0.0),
        timer: STEP_DELAY,
        grammar: basic_grammar(),
        depth: 0,
        todo: Vec::new()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
