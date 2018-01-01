//! Basic hello world example.

extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::timer;
use ggez::graphics::{self, Point2};

extern crate shooter;
use shooter::entities::{Player, PlayerState, Shot};
use shooter::assets::Assets;

use std::env;
use std::path;

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}

struct MainState {
    assets: Assets,
    input: InputState,
    player: Player,
    shots: Vec<Shot>,
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
            shots: Vec::new(),
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

            // Update player state
            self.player.update(self.input.movement, seconds, self.screen_width as f32);
            self.player.time_until_next_shot -= seconds;
            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                let shot_pos = Point2::new(self.player.pos.x - 75.0, self.player.pos.y - 80.0);
                let shot = Shot::new(shot_pos);
                self.shots.push(shot);

                let _ = self.assets.shot_sound.play();

                self.player.time_until_next_shot = Player::SHOT_TIMEOUT;
                self.player.state = PlayerState::Shooting;
            } else if !self.input.fire {
                self.player.state = PlayerState::Normal;
            }

            // Update shots state
            for shot in self.shots.iter_mut() {
                shot.update(seconds);
            }

            // Remove dead shots
            self.shots.retain(|shot| shot.pos.y >= 0.0);
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

        self.player.draw(ctx, &self.assets)?;

        // Draw all the shots
        for shot in self.shots.iter() {
            graphics::draw_ex(ctx, &self.assets.shot_image, graphics::DrawParam {
                dest: shot.pos,
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
