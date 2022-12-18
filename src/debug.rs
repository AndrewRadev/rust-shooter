use ggez::{Context, GameResult};
use ggez::graphics;

pub fn is_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

pub fn draw_outline(bounding_box: graphics::Rect, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult<()>  {
    let draw_mode = graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(1.0));
    let red = graphics::Color::from_rgb(255, 0, 0);

    let mut mesh_builder = graphics::MeshBuilder::new();
    let outline_data = mesh_builder.rectangle(draw_mode, bounding_box, red)?.build();
    let outline = graphics::Mesh::from_data(ctx, outline_data);

    canvas.draw(&outline, graphics::DrawParam::default());
    Ok(())
}
