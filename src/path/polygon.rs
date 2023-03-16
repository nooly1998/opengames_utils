
use std::task::Context;
use valora::canvas::Canvas;
use valora::forms::Ngon;
use valora::P2;
use valora::paint::Stroked;
use valora::prelude::LinSrgb;

pub fn draw_polygon(v: Vec<P2>, canvas:&mut Canvas, ctx: Context, color: LinSrgb, lineWidth: f32)
{
    if v.is_empty() {
        return;
    }
    let start_p = v[0].clone();
    canvas.set_color(color);
    canvas.move_to(start_p);
    for val in v {
        canvas.line_to(val);
        canvas.close_path();
        canvas.set_stroke_width(lineWidth);
        canvas.stroke();
    }
    canvas.line_to(start_p);
    canvas.close_path();
    canvas.set_stroke_width(lineWidth);
    canvas.stroke();
}

pub fn draw_triangle_stroke(p:P2, r:f32, canvas:&mut Canvas, lineWidth: f32)
{
    let triangle = Ngon::triangle(p,r);
    canvas.paint(Stroked {
        width: lineWidth,
        element: triangle,
    });
}

