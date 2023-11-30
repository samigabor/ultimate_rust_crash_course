use rusty_time::timer::Timer;
use crate::frame::Drawable;
use crate::frame::Frame;

pub struct Shot {
    x: usize,
    y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: std::time::Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            } else {
                self.timer.reset();
            }
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(750);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}