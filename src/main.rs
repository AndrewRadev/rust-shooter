use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::graphics::{self, Drawable};
use ggez::input::keyboard;
use ggez::mint::Point2;
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
        "Unnecessary Heap\nAllocations", "Data Races"
    ];

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        // Player starts in bottom-middle of the screen
        let player_pos = Point2 {
            x: screen_width / 2.0,
            y: screen_height,
        };

        let s = MainState {
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

        const DESIRED_TICKS_PER_SEC: u32 = 60;

        while ctx.time.check_update_time(DESIRED_TICKS_PER_SEC) {
            let seconds = 1.0 / DESIRED_TICKS_PER_SEC as f32;

            // Spawn enemies
            self.time_until_next_enemy -= seconds;
            if self.time_until_next_enemy <= 0.0 {
                let random_point = Point2 {
                    x: self.rng.gen_range(0.0 .. self.screen_width - 100.0),
                    y: 0.0,
                };
                let random_text = Self::ENEMIES[self.rng.gen_range(0 .. Self::ENEMIES.len())];
                let random_speed = self.rng.gen_range(50.0 .. 200.0);

                let enemy_sprite = Box::new(TextSprite::new(random_text)?);
                let enemy = Enemy::new(random_text, random_point, random_speed, enemy_sprite)?;

                self.enemies.push(enemy);
                self.time_until_next_enemy = self.rng.gen_range(0.5 .. 1.8);
            }

            // Update player state
            self.player.update(self.input.movement, seconds, self.screen_width);
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

                if enemy.pos.y >= self.screen_height {
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

    fn key_down_event(&mut self, ctx: &mut Context, input: keyboard::KeyInput, repeat: bool) -> GameResult<()> {
        if repeat {
            return Ok(());
        }

        match input.keycode {
            Some(keyboard::KeyCode::Space) => self.input.fire = true,
            Some(keyboard::KeyCode::Left) => self.input.movement = -1.0,
            Some(keyboard::KeyCode::Right) => self.input.movement = 1.0,
            Some(keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> GameResult<()> {
        match input.keycode {
            Some(keyboard::KeyCode::Space) => self.input.fire = false,
            Some(keyboard::KeyCode::Left | keyboard::KeyCode::Right) => {
                self.input.movement = 0.0
            },
            _ => (), // Do nothing
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        let mut canvas = graphics::Canvas::from_frame(ctx, dark_blue);

        if self.game_over {
            let mut text = graphics::Text::new(format!("Killed by {}.\nScore: {}", self.killed_by, self.score));
            text.set_font("MainFont");
            text.set_scale(graphics::PxScale::from(40.0));

            let top_left = Point2 {
                x: (self.screen_width - text.dimensions(ctx).unwrap().w) / 2.0,
                y: (self.screen_height - text.dimensions(ctx).unwrap().h) / 2.0,
            };
            canvas.draw(&text, graphics::DrawParam::default().dest(top_left));
            canvas.finish(ctx)?;
            return Ok(())
        }

        self.player.draw(&mut canvas, &self.assets);

        for shot in self.shots.iter_mut() {
            shot.draw(&mut canvas, &self.assets);
        }

        for enemy in self.enemies.iter_mut() {
            enemy.draw(&mut canvas);
        }

        if debug::is_active() {
            for enemy in &mut self.enemies {
                debug::draw_outline(enemy.bounding_rect(ctx), &mut canvas, ctx).unwrap();
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1200.0,
            height: 1000.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("shooter", "Andrew").
        default_conf(conf.clone()).
        build().
        unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let font_data = graphics::FontData::from_path(&ctx, "/DejaVuSerif.ttf").unwrap();
    ctx.gfx.add_font("MainFont", font_data);

    let state = MainState::new(&mut ctx, &conf).unwrap();

    event::run(ctx, event_loop, state);
}
