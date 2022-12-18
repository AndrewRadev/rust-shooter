use ggez::mint::Point2;
use ggez::{graphics, Context};
use quickcheck::quickcheck;

use shooter::entities::*;
use shooter::assets::Sprite;

#[derive(Debug)]
struct MockSprite {
    width: f32,
    height: f32,
}

impl Sprite for MockSprite {
    fn draw(&mut self, _center: Point2<f32>, _canvas: &mut graphics::Canvas) {}

    fn width(&self, _ctx: &mut Context) -> f32 { self.width }
    fn height(&self, _ctx: &mut Context) -> f32 { self.height }
}

quickcheck! {
    fn prop_enemies_fall_downwards(x: f32, y: f32) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0 });
        let mut enemy = Enemy::new("test", Point2 { x, y }, 10.0, mock_sprite).unwrap();

        let old_pos = enemy.pos.clone();
        enemy.update(10.0);

        enemy.pos.x == old_pos.x && enemy.pos.y > old_pos.y
    }

    fn prop_shots_fly_upwards(x: f32, y: f32) -> bool {
        let mut shot = Shot::new(Point2 { x, y });

        let old_pos = shot.pos.clone();
        shot.update(10.0);

        shot.pos.x == old_pos.x && shot.pos.y < old_pos.y
    }
}
