use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::input;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;

use shooter::entities::{Player, PlayerState, Shot, Enemy};
use shooter::assets::{Assets, TextSprite};
use shooter::debug;

use std::env;
use std::path;

#[derive(Debug, Default)]
struct InputState {
    movement: f32,
    fire: bool,
}

struct MainState {
    conf: Conf,
    rng: ThreadRng,
    game_over: bool,
    killed_by: String,
    score: u32,
    assets: Assets,
    input: InputState,
    player: Player,
    shots: Vec<Shot>,
    enemies: Vec<Enemy>,
    time_until_next_enemy: f32,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    const ENEMIES: [&'static str; 7] = [
        "Segfaults", "Undefined Behaviour",
        "NULLs", "Inefficiencies", "Bloat",
        "Unnecessary Heap Allocations", "Data Races"
    ];

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        // Player starts in bottom-middle of the screen
        let player_pos = Point2 {
            x: (screen_width as f32) / 2.0,
            y: screen_height as f32,
        };

        let s = MainState {
            conf: conf.clone(),
            rng: rand::thread_rng(),
            game_over: false,
            score: 0,
            killed_by: String::new(),
            assets: assets,
            input: InputState::default(),
            player: Player::new(player_pos),
            shots: Vec::new(),
            enemies: Vec::new(),
            time_until_next_enemy: 1.0,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
        };

        Ok(s)
    }

    fn handle_collisions(&mut self, ctx: &mut Context) {
        for enemy in &mut self.enemies {
            for shot in &mut self.shots {
                if enemy.bounding_rect(ctx).contains(shot.pos) {
                    shot.is_alive = false;
                    enemy.is_alive = false;
                    self.score += 1;
                    let _ = self.assets.boom_sound.play(ctx);
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
                let random_point = Point2 {
                    x: self.rng.gen_range(40.0 .. self.conf.window_mode.width - 40.0),
                    y: 0.0,
                };
                let random_text = Self::ENEMIES[self.rng.gen_range(0 .. Self::ENEMIES.len())];
                let random_speed = self.rng.gen_range(50.0 .. 200.0);

                let enemy_sprite = Box::new(TextSprite::new(random_text, ctx)?);
                let enemy = Enemy::new(random_text, random_point, random_speed, enemy_sprite)?;

                self.enemies.push(enemy);
                self.time_until_next_enemy = self.rng.gen_range(0.5 .. 1.8);
            }

            // Update player state
            self.player.update(self.input.movement, seconds, self.screen_width as f32);
            self.player.time_until_next_shot -= seconds;
            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                let shot_pos = Point2 {
                    x: self.player.pos.x - 75.0,
                    y: self.player.pos.y - 80.0,
                };
                let shot = Shot::new(shot_pos);
                self.shots.push(shot);

                let _ = self.assets.shot_sound.play(ctx);

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
                    if debug::is_active() {
                        // We don't end the game in debug mode, but we do make sure the enemy is
                        // cleaned up from the enemy array
                        enemy.is_alive = false;
                    } else {
                        self.game_over = true;
                        self.killed_by = String::from(enemy.label());
                        let _ = self.assets.boom_sound.play(ctx);
                    }
                }
            }

            self.handle_collisions(ctx);

            self.shots.retain(|shot| shot.is_alive && shot.pos.y >= 0.0);
            self.enemies.retain(|enemy| enemy.is_alive);
        }

        Ok(())
    }

    fn key_down_event(&mut self,
                      ctx: &mut Context,
                      keycode: event::KeyCode,
                      _keymod: input::keyboard::KeyMods,
                      _repeat: bool) {
        match keycode {
            event::KeyCode::Space => self.input.fire = true,
            event::KeyCode::Left => self.input.movement = -1.0,
            event::KeyCode::Right => self.input.movement = 1.0,
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self,
                    _ctx: &mut Context,
                    keycode: event::KeyCode,
                    _keymod: input::keyboard::KeyMods) {
        match keycode {
            event::KeyCode::Space => self.input.fire = false,
            event::KeyCode::Left | event::KeyCode::Right => self.input.movement = 0.0,
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        graphics::clear(ctx, dark_blue);

        if self.game_over {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
            let mut text = graphics::Text::new(format!("Killed by {}. Score: {}", self.killed_by, self.score));
            text.set_font(font, graphics::PxScale::from(40.0));

            let center = Point2 {
                x: self.screen_width as f32 / 2.0,
                y: self.screen_height as f32 / 2.0,
            };
            graphics::draw(ctx, &text, graphics::DrawParam {
                dest: center,
                offset: Point2 { x: 0.5, y: 0.5 },
                .. Default::default()
            })?;
            graphics::present(ctx)?;
            return Ok(())
        }

        self.player.draw(ctx, &self.assets)?;

        for shot in self.shots.iter_mut() {
            shot.draw(ctx, &self.assets)?;
        }

        for enemy in self.enemies.iter_mut() {
            enemy.draw(ctx)?;
        }

        if debug::is_active() {
            for enemy in &mut self.enemies {
                debug::draw_outline(enemy.bounding_rect(ctx), ctx).unwrap();
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            min_width: 1024.0,
            min_height: 768.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("shooter", "Andrew").
        conf(conf.clone()).
        build().
        unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, &conf).unwrap();

    event::run(ctx, event_loop, state);
}
