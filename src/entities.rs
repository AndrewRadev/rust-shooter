use ggez::graphics::{Vector2, Point2};
use ggez::nalgebra as na;

#[derive(Debug)]
pub enum PlayerState {
    Normal,
    Shooting,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub pos: Point2,
    pub time_until_next_shot: f32,
    velocity: Vector2,
    bbox_size: f32,
}

impl Player {
    pub const SHOT_TIMEOUT: f32 = 1.0;
    pub const SPEED: f32 = 500.0;

    pub fn new(pos: Point2) -> Self {
        Player {
            state: PlayerState::Normal,
            pos,
            velocity: Vector2::new(0.0, 0.0),
            bbox_size: 10.0,
            time_until_next_shot: Self::SHOT_TIMEOUT,
        }
    }

    pub fn movement(&mut self, amount: f32, seconds: f32, max_right: f32) {
        let new_pos = self.pos.x + Self::SPEED * seconds * amount;
        self.pos.x = na::clamp(new_pos, 0.0, max_right);
    }
}

#[derive(Debug)]
pub struct Shot {
    pos: Point2,
    velocity: Vector2,
    bbox_size: f32,
}

impl Shot {
    pub fn new(pos: Point2) -> Self {
        Shot { pos, velocity: Vector2::new(0.0, 1.0), bbox_size: 10.0 }
    }

    pub fn movement(&mut self, seconds: f32) {
        self.pos += self.velocity * seconds;
    }
}
