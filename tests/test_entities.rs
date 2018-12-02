extern crate ggez;
extern crate shooter;

use ggez::nalgebra as na;
use shooter::entities::*;

#[test]
fn test_shots_fly_upwards() {
    let mut shot = Shot::new(na::Point2::new(0.0, 0.0));

    let old_pos = shot.pos.clone();
    shot.update(10.0);

    assert_eq!(shot.pos.x, old_pos.x);
    assert!(shot.pos.y < old_pos.y);
}
