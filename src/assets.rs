use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio;

pub struct Assets {
    pub ferris_normal_image: graphics::Image,
    pub ferris_shooting_image: graphics::Image,
    pub shot_image: graphics::Image,
    pub font: graphics::Font,
    pub shot_sound: audio::Source,
    pub boom_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ferris_normal_image = graphics::Image::new(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::new(ctx, "/ferris-shooting.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let boom_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image,
            shot_image, font,
            shot_sound, boom_sound,
        })
    }
}

