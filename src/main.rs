//! Basic hello world example.

extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use ggez::audio;
use ggez::graphics::{Vector2, Point2};
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

#[derive(Debug)]
struct Player {
    pos: Point2,
    velocity: Vector2,
    bbox_size: f32,
}

impl Player {
    fn new(pos: Point2) -> Self {
        Player {
            pos,
            velocity: Vector2::new(0.0, 0.0),
            bbox_size: 10.0,
        }
    }
}

struct MainState {
    assets: Assets,
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
            (screen_width as f32) / 2.0 + player_bbox_size,
            (screen_height as f32) - player_bbox_size
        );

        let s = MainState {
            assets: assets,
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


        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        //let pos = world_to_screen_coords(self.screen_width, self.screen_height, ferris.pos);
        //let image = assets.actor_image(actor);
        //let drawparams = graphics::DrawParam {
        //    dest: pos,
        //    rotation: actor.facing as f32,
        //    offset: graphics::Point2::new(0.5, 0.5),
        //    .. Default::default()
        //};
        //graphics::draw_ex(ctx, image, drawparams);

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
