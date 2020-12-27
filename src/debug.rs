use ggez::{Context, GameResult};
use ggez::graphics;

pub fn is_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

pub fn draw_outline(bounding_box: graphics::Rect, ctx: &mut Context) -> GameResult<()>  {
    let draw_mode = graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(1.0));
    let red = graphics::Color::from_rgb(255, 0, 0);
    let outline = graphics::MeshBuilder::new().
        rectangle(draw_mode, bounding_box, red).
        build(ctx).
        unwrap();

    graphics::draw(ctx, &outline, graphics::DrawParam::default())?;
    Ok(())
}
