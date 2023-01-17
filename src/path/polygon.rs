
use std::task::Context;
use valora::canvas::Canvas;
use valora::forms::Ngon;
use valora::P2;
use valora::paint::Stroked;
use valora::prelude::LinSrgb;

pub fn draw_polygon(v: Vec<P2>, canvas:&mut Canvas, ctx: Context, color: LinSrgb, lineWidth: float)
{
    if v.is_empty() {
        return;
    }
    canvas.set_color(color);
    canvas.move_to(v[0]);
    for val in v {
        canvas.line_to(val);
        canvas.close_path();
        canvas.set_stroke_width(lineWidth);
        canvas.stroke();
    }
    canvas.line_to(v[0]);
    canvas.close_path();
    canvas.set_stroke_width(lineWidth);
    canvas.stroke();
}

pub fn draw_triangle_stroke(p:P2, r:float, canvas:&mut Canvas, lineWidth: float)
{
    let triangle = Ngon::triangle(p,r);
    canvas.paint(Stroked {
        width: lineWidth,
        element: triangle,
    });
}

pub fn draw_triangle_fill(p:P2, r:float, canvas:&mut Canvas, lineWidth: float)
{
    let triangle = Ngon::triangle(p,r);
    canvas.paint(Filled {
        width: lineWidth,
        element: triangle,
    });
}

