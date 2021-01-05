use ggez::audio;
use ggez::graphics;
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
        let ferris_normal_image   = graphics::Image::new(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::new(ctx, "/ferris-shooting.png")?;
        let shot_image            = graphics::Image::new(ctx, "/shot.png")?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let boom_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image, shot_image,
            shot_sound, boom_sound,
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, center: Point2<f32>, ctx: &mut Context) -> GameResult<()>;
    fn width(&self, ctx: &mut Context) -> f32;
    fn height(&self, ctx: &mut Context) -> f32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str, ctx: &mut Context) -> GameResult<TextSprite> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let mut text = graphics::Text::new(label);
        text.set_font(font, graphics::PxScale::from(26.0));
        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, top_left: Point2<f32>, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.text, graphics::DrawParam {
            dest: top_left,
            .. Default::default()
        })
    }

    fn width(&self, ctx: &mut Context) -> f32 { self.text.width(ctx) }
    fn height(&self, ctx: &mut Context) -> f32 { self.text.height(ctx) }
}
