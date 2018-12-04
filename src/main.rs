//! Basic hello world example.

extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use ggez::graphics::{Vector2, Point2};
use std::env;
use std::path;

struct Assets {
    ferris_normal_image: graphics::Image,
    ferris_shooting_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ferris_normal_image = graphics::Image::new(ctx, "/ferris-normal.png")?;
        let ferris_shooting_image = graphics::Image::new(ctx, "/ferris-shooting.png")?;

        Ok(Assets {
            ferris_normal_image, ferris_shooting_image,
        })
    }
}

#[derive(Debug)]
struct Player {
    pos: Point2,
    velocity: Vector2,
}

impl Player {
    fn new(pos: Point2) -> Self {
        Player {
            pos,
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Debug, Default)]
struct InputState {
    fire: bool,
}

struct MainState {
    assets: Assets,
    input: InputState,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        // Player starts in bottom-middle of the screen
        let player_pos = Point2::new(
            (screen_width as f32) / 2.0,
            screen_height as f32,
        );

        let s = MainState {
            assets: assets,
            input: InputState::default(),
            player: Player::new(player_pos),
        };

        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);


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
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if self.input.fire {
            graphics::draw_ex(ctx, &self.assets.ferris_shooting_image, graphics::DrawParam {
                dest: self.player.pos,
                offset: Point2::new(0.545, 0.96),
                .. Default::default()
            })?;
        } else {
            graphics::draw_ex(ctx, &self.assets.ferris_normal_image, graphics::DrawParam {
                dest: self.player.pos,
                scale: Point2::new(0.95, 0.95),
                offset: Point2::new(0.5, 1.0),
                .. Default::default()
            })?;
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
