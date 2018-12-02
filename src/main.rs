extern crate ggez;
use ggez::conf::{WindowMode};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event;
use ggez::timer;
use ggez::graphics::{self, Point2};

extern crate rand;
use rand::Rng;

extern crate shooter;
use shooter::entities::{Player, PlayerState, Shot, Enemy, TextSprite};
use shooter::assets::Assets;

use std::env;
use std::path;

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}

struct MainState {
    game_over: bool,
    killed_by: String,
    score: u32,
    assets: Assets,
    input: InputState,
    player: Player,
    shots: Vec<Shot>,
    enemies: Vec<Enemy>,
    time_until_next_enemy: f32,
    screen_width: u32,
    screen_height: u32,
}

impl MainState {
    const ENEMIES: [&'static str; 7] = [
        "Segfaults", "Undefined Behaviour",
        "NULLs", "Inefficiencies", "Bloat",
        "Unnecessary Heap Allocations", "Data Races"
    ];

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
            game_over: false,
            score: 0,
            killed_by: String::new(),
            assets: assets,
            input: InputState::default(),
            player: Player::new(player_pos),
            shots: Vec::new(),
            enemies: Vec::new(),
            time_until_next_enemy: 1.0,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };

        Ok(s)
    }

    fn handle_collisions(&mut self) {
        for enemy in &mut self.enemies {
            for shot in &mut self.shots {
                if enemy.bounding_rect().contains(shot.pos) {
                    shot.is_alive = false;
                    enemy.is_alive = false;
                    self.score += 1;
                    let _ = self.assets.boom_sound.play();
                }
            }
        }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(());
        }

        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            // Spawn enemies
            self.time_until_next_enemy -= seconds;
            if self.time_until_next_enemy <= 0.0 {
                let mut rng = rand::thread_rng();
                let random_point = Point2::new(rng.gen_range(40.0, ctx.conf.window_mode.width as f32 - 40.0), 0.0);
                let random_text = Self::ENEMIES[rng.gen_range(0, Self::ENEMIES.len())];
                let random_speed = rng.gen_range(50.0, 200.0);

                let enemy_sprite = Box::new(TextSprite::new(random_text, ctx)?);
                let enemy = Enemy::new(random_text, random_point, random_speed, enemy_sprite)?;

                self.enemies.push(enemy);
                self.time_until_next_enemy = rng.gen_range(0.5, 1.8);
            }

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

            for shot in self.shots.iter_mut() {
                shot.update(seconds);
            }

            for enemy in self.enemies.iter_mut() {
                enemy.update(seconds);

                if enemy.pos.y >= self.screen_height as f32 {
                    self.game_over = true;
                    self.killed_by = String::from(enemy.label());
                    let _ = self.assets.boom_sound.play();
                }
            }

            self.handle_collisions();

            self.shots.retain(|shot| shot.is_alive && shot.pos.y >= 0.0);
            self.enemies.retain(|enemy| enemy.is_alive);
        }

        Ok(())
    }

    fn key_down_event(&mut self,
                      ctx: &mut Context,
                      keycode: event::Keycode,
                      _keymod: event::Mod,
                      _repeat: bool) {
        match keycode {
            event::Keycode::Space => self.input.fire = true,
            event::Keycode::Left => self.input.movement = -1.0,
            event::Keycode::Right => self.input.movement = 1.0,
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
            event::Keycode::Space => self.input.fire = false,
            event::Keycode::Left | event::Keycode::Right => self.input.movement = 0.0,
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if self.game_over {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 24)?;
            let text = graphics::Text::new(ctx, &format!("Killed by {}. Score: {}", self.killed_by, self.score), &font)?;

            let center = Point2::new(self.screen_width as f32 / 2.0, self.screen_height as f32 / 2.0);
            graphics::draw_ex(ctx, &text, graphics::DrawParam {
                dest: center,
                offset: Point2::new(0.5, 0.5),
                .. Default::default()
            })?;
            graphics::present(ctx);
            return Ok(())
        }

        self.player.draw(ctx, &self.assets)?;

        for shot in self.shots.iter_mut() {
            shot.draw(ctx, &self.assets)?;
        }

        for enemy in self.enemies.iter_mut() {
            enemy.draw(ctx)?;
        }

        if std::env::var("DEBUG").is_ok() {
            for enemy in &mut self.enemies {
                graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
                graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), enemy.bounding_rect())?;
                graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let ctx = &mut ContextBuilder::new("shooter", "Andrew").
        window_mode(WindowMode {
            min_width: 1024,
            min_height: 768,
            ..Default::default()
        }).
        build().unwrap();

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
