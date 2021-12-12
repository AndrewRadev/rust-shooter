use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::{Vector2, Point2};

use crate::assets::{Assets, Sprite};

#[derive(Debug)]
pub enum PlayerState {
    Normal,
    Shooting,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub pos: Point2<f32>,
    pub time_until_next_shot: f32,
}

impl Player {
    pub const SHOT_TIMEOUT: f32 = 1.0;
    pub const SPEED: f32 = 500.0;

    pub fn new(pos: Point2<f32>) -> Self {
        Player {
            state: PlayerState::Normal,
            pos,
            time_until_next_shot: Self::SHOT_TIMEOUT,
        }
    }

    pub fn update(&mut self, amount: f32, seconds: f32, max_right: f32) {
        let new_pos = self.pos.x + Self::SPEED * seconds * amount;
        self.pos.x = nalgebra::clamp(new_pos, 0.0, max_right);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        match self.state {
            PlayerState::Normal => {
                graphics::draw(ctx, &assets.ferris_normal_image, graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.95, y: 0.95 }).
                    offset(Point2 { x: 0.5, y: 1.0 })
                )?;
            },

            PlayerState::Shooting => {
                graphics::draw(ctx, &assets.ferris_shooting_image, graphics::DrawParam::default().
                    dest(self.pos).
                    offset(Point2 { x: 0.545, y: 0.96 })
                )?;
            },
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Shot {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    velocity: Vector2<f32>,
}

impl Shot {
    pub fn new(pos: Point2<f32>) -> Self {
        Shot {
            pos,
            is_alive: true,
            velocity: Vector2 { x: 0.0, y: -500.0 },
        }
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
    }

    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        graphics::draw(ctx, &assets.shot_image, graphics::DrawParam::default().dest(self.pos))
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    label: String,
    velocity: Vector2<f32>,
    sprite: Box<dyn Sprite>,
}

impl Enemy {
    pub fn new(label: &str, pos: Point2<f32>, speed: f32, sprite: Box<dyn Sprite>) -> GameResult<Self> {
        let label = String::from(label);

        Ok(Enemy {
            pos, label, sprite,
            is_alive: true,
            velocity: Vector2 { x: 0.0, y: speed },
        })
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.sprite.draw(self.pos, ctx)
    }

    pub fn bounding_rect(&self, ctx: &mut Context) -> graphics::Rect {
        let left   = self.pos.x;
        let right  = self.pos.x + self.sprite.width(ctx);
        let top    = self.pos.y;
        let bottom = self.pos.y + self.sprite.height(ctx);

        graphics::Rect::new(left, top, right - left, bottom - top)
    }
}
