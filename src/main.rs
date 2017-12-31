//! Basic hello world example.

extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use ggez::audio;
use ggez::graphics::{Vector2, Point2};

extern crate shooter;
use shooter::entities::{Player, PlayerState, Shot};

use std::env;
use std::path;

struct Assets {
    ferris_normal_image: graphics::Image,
    ferris_shooting_image: graphics::Image,
    shot_image: graphics::Image,
    font: graphics::Font,
    shot_sound: audio::Source,
    hit_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ferris_normal_image = graphics::Image::new(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::new(ctx, "/ferris-shooting.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;

        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image,
            shot_image, font,
            shot_sound, hit_sound,
        })
    }
}

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}

struct MainState {
    assets: Assets,
    input: InputState,
    player: Player,
    screen_width: u32,
    screen_height: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        // Player starts in bottom-middle of the screen
        let player_bbox_size = 10.0;
        let player_pos = Point2::new(
            (screen_width as f32) / 2.0,
            (screen_height as f32),
        );

        let s = MainState {
            assets: assets,
            input: InputState::default(),
            player: Player::new(player_pos),
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };

        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.player.movement(self.input.movement, seconds, self.screen_width as f32);
            self.player.time_until_next_shot -= seconds;
            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                // fire shot
                let _ = self.assets.shot_sound.play();
                self.player.time_until_next_shot = Player::SHOT_TIMEOUT;
                self.player.state = PlayerState::Shooting;
            } else if !self.input.fire {
                self.player.state = PlayerState::Normal;
            }
        }

        Ok(())
    }

    fn key_down_event(&mut self,
                      ctx: &mut Context,
                      keycode: event::Keycode,
                      _keymod: event::Mod,
                      _repeat: bool) {
        match keycode {
            event::Keycode::Space => {
                self.input.fire = true;
            }
            event::Keycode::Left => {
                self.input.movement = -1.0;
            }
            event::Keycode::Right => {
                self.input.movement = 1.0;
            }
            event::Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self,
                    _ctx: &mut Context,
                    keycode: event::Keycode,
                    _keymod: event::Mod,
                    _repeat: bool) {
        match keycode {
            event::Keycode::Space => {
                self.input.fire = false;
            }
            event::Keycode::Left | event::Keycode::Right => {
                self.input.movement = 0.0;
            }
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        match self.player.state {
            PlayerState::Normal => {
                graphics::draw_ex(ctx, &self.assets.ferris_normal_image, graphics::DrawParam {
                    dest: self.player.pos,
                    scale: Point2::new(0.95, 0.95),
                    offset: Point2::new(0.5, 1.0),
                    .. Default::default()
                })?;
            },

            PlayerState::Shooting => {
                graphics::draw_ex(ctx, &self.assets.ferris_shooting_image, graphics::DrawParam {
                    dest: self.player.pos,
                    offset: Point2::new(0.545, 0.96),
                    .. Default::default()
                })?;
            },
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("shooter", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
