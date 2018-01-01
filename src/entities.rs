use ggez::{Context, GameResult};
use ggez::graphics::{self, Vector2, Point2};
use ggez::nalgebra as na;

use assets::Assets;

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

    pub fn update(&mut self, amount: f32, seconds: f32, max_right: f32) {
        let new_pos = self.pos.x + Self::SPEED * seconds * amount;
        self.pos.x = na::clamp(new_pos, 0.0, max_right);
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        match self.state {
            PlayerState::Normal => {
                graphics::draw_ex(ctx, &assets.ferris_normal_image, graphics::DrawParam {
                    dest: self.pos,
                    scale: Point2::new(0.95, 0.95),
                    offset: Point2::new(0.5, 1.0),
                    .. Default::default()
                })?;
            },

            PlayerState::Shooting => {
                graphics::draw_ex(ctx, &assets.ferris_shooting_image, graphics::DrawParam {
                    dest: self.pos,
                    offset: Point2::new(0.545, 0.96),
                    .. Default::default()
                })?;
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Shot {
    pub pos: Point2,
    velocity: Vector2,
    bbox_size: f32,
}

impl Shot {
    pub fn new(pos: Point2) -> Self {
        Shot { pos, velocity: Vector2::new(0.0, -500.0), bbox_size: 10.0 }
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos += self.velocity * seconds;
    }
}
