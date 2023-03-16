
use std::task::Context;
use valora::canvas::Canvas;
use valora::P2;
use valora::prelude::LinSrgb;

pub fn draw_line(p1:P2, p2:P2, canvas:&mut Canvas, ctx: Context, color: LinSrgb, lineWidth: f32)
{
    canvas.set_color(color);
    canvas.move_to(p1);
    canvas.line_to(p2);
    canvas.close_path();
    canvas.set_stroke_width(lineWidth);
    canvas.stroke();
}


