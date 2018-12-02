use std::fmt::Debug;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio;

pub struct Assets {
    pub ferris_normal_image: graphics::Image,
    pub ferris_shooting_image: graphics::Image,
    pub shot_image: graphics::Image,
    pub shot_sound: audio::Source,
    pub boom_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ferris_normal_image = graphics::Image::new(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::new(ctx, "/ferris-shooting.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let boom_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image, shot_image,
            shot_sound, boom_sound,
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, center: graphics::Point2, ctx: &mut Context) -> GameResult<()>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text,
}

impl TextSprite {
    pub fn new(label: &str, ctx: &mut Context) -> GameResult<TextSprite> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
        let text = graphics::Text::new(ctx, label, &font)?;
        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, center: graphics::Point2, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(ctx, &self.text, graphics::DrawParam {
            dest: center,
            offset: graphics::Point2::new(0.5, 0.5),
            .. Default::default()
        })
    }

    fn width(&self) -> u32 { self.text.width() }
    fn height(&self) -> u32 { self.text.height() }
}
