use ggez::audio::{self, SoundSource};
use ggez::graphics::{self, Drawable};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use std::fmt::Debug;

pub struct Assets {
    pub ferris_normal_image:   graphics::Image,
    pub ferris_shooting_image: graphics::Image,
    pub shot_image:            graphics::Image,

    pub shot_sound: audio::Source,
    pub boom_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ferris_normal_image   = graphics::Image::from_path(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::from_path(ctx, "/ferris-shooting.png")?;
        let shot_image            = graphics::Image::from_path(ctx, "/shot.png")?;

        let mut shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        shot_sound.set_volume(0.5);

        let mut boom_sound = audio::Source::new(ctx, "/boom.ogg")?;
        boom_sound.set_volume(0.3);

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image, shot_image,
            shot_sound, boom_sound,
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, center: Point2<f32>, canvas: &mut graphics::Canvas);
    fn width(&self, ctx: &mut Context) -> f32;
    fn height(&self, ctx: &mut Context) -> f32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str) -> GameResult<TextSprite> {
        let mut text = graphics::Text::new(label);

        text.set_font("MainFont");
        text.set_scale(graphics::PxScale::from(32.0));

        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, top_left: Point2<f32>, canvas: &mut graphics::Canvas) {
        canvas.draw(&self.text, graphics::DrawParam::default().dest(top_left))
    }

    fn width(&self, ctx: &mut Context) -> f32 { self.text.dimensions(ctx).unwrap().w }
    fn height(&self, ctx: &mut Context) -> f32 { self.text.dimensions(ctx).unwrap().h }
}
