use std;

#[derive(Clone, Copy)]
pub struct TurtleMovement {
    pub x_from: f64,
    pub y_from: f64,
    pub x_to:   f64,
    pub y_to:   f64,
    pub draw:   bool
}

impl TurtleMovement {

    pub fn none(turtle: &Turtle) -> TurtleMovement {
        TurtleMovement {
            x_from: turtle.state.x,
            y_from: turtle.state.y,
            x_to: turtle.state.x,
            y_to: turtle.state.y,
            draw: false
        }
    }

}

#[derive(Clone, Copy)]
struct TurtleState {
    x: f64,
    y: f64,
    rot: f64,
    step: f64,
    spin: f64,
    draw: bool
}

#[derive(Clone)]
pub struct Turtle {
    stack: Vec<TurtleState>,
    state: TurtleState
}

impl Turtle {

    pub fn new(x: f64, y: f64) -> Turtle {
        Turtle {
            stack: Vec::new(),
            state: TurtleState {
                x: x,
                y: y,
                rot: (std::f64::consts::PI / 2.0),
                step: 5.0,
                spin: (std::f64::consts::PI / 2.0),
                draw: true
            }
        }
    }

    pub fn push(&mut self) -> TurtleMovement {
        // Push the current state onto the stack.
        self.stack.push(self.state.clone());
        // Return.
        return TurtleMovement::none(self);
    }

    pub fn pop(&mut self) -> TurtleMovement {
        // Save old location.
        let ox = self.state.x;
        let oy = self.state.y;
        // Attempt to pop state.
        self.state = self.stack.pop().unwrap();
        // Return.
        return TurtleMovement {
            x_from: ox,
            y_from: oy,
            x_to: self.state.x,
            y_to: self.state.y,
            draw: false
        }
    }

    pub fn turn_left(&mut self) -> TurtleMovement {
        // Add to current angle.
        self.state.rot += self.state.spin;
        // Return.
        return TurtleMovement::none(self);
    }

    pub fn turn_right(&mut self) -> TurtleMovement {
        // Subtract from current angle.
        self.state.rot -= self.state.spin;
        // Return.
        return TurtleMovement::none(self);
    }

    pub fn forward(&mut self) -> TurtleMovement {
        // Old location.
        let ox = self.state.x;
        let oy = self.state.y;
        // Add to current position.
        self.state.x += self.state.rot.cos() * self.state.step;
        self.state.y += self.state.rot.sin() * self.state.step;
        // Send line to consumer.
        return TurtleMovement {
            x_from: ox,
            y_from: oy,
            x_to: self.state.x,
            y_to: self.state.y,
            draw: self.state.draw
        }
    }

    pub fn up(&mut self) -> TurtleMovement {
        // Set pen to up.
        self.state.draw = false;
        // Return.
        return TurtleMovement::none(self);
    }

    pub fn down(&mut self) -> TurtleMovement {
        // Set pen to down.
        self.state.draw = true;
        // Return.
        return TurtleMovement::none(self);
    }

    pub fn eval(&mut self, command: char) -> TurtleMovement {
        match command {
            'f' => {
                return self.forward();
            }
            '+' => {
                return self.turn_left();
            }
            '-' => {
                return self.turn_right();
            }
            '[' => {
                return self.push();
            }
            ']' => {
                return self.pop();
            }
            '?' => {
                return self.up();
            }
            '!' => {
                return self.down();
            }
            _ => {
                // Print error message.
                print!("Turtle recieved invalid command: '{0}'!\n", command);
                return TurtleMovement::none(self);
            }
        }
    }

}
