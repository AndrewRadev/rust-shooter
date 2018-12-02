extern crate ggez;
extern crate shooter;

use ggez::{Context, GameResult};
use ggez::graphics::Point2;
use ggez::nalgebra as na;
use shooter::entities::*;

#[derive(Debug)]
struct MockSprite {
    width: u32,
    height: u32,
}

impl Sprite for MockSprite {
    fn draw(&mut self, _center: Point2, _ctx: &mut Context) -> GameResult<()> { Ok(()) }

    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
}

#[test]
fn test_enemies_fall_downwards() {
    let mock_sprite = Box::new(MockSprite { width: 100, height: 100 });
    let mut enemy = Enemy::new("test", na::Point2::new(1000.0, 1000.0), 10.0, mock_sprite).unwrap();

    let old_pos = enemy.pos.clone();
    enemy.update(10.0);

    assert_eq!(enemy.pos.x, old_pos.x);
    assert!(enemy.pos.y > old_pos.y);
}

#[test]
fn test_shots_fly_upwards() {
    let mut shot = Shot::new(na::Point2::new(0.0, 0.0));

    let old_pos = shot.pos.clone();
    shot.update(10.0);

    assert_eq!(shot.pos.x, old_pos.x);
    assert!(shot.pos.y < old_pos.y);
}
