use ckia::{paint::Paint, path::SkiaPath, path_effect::PathEffect, Color, PaintStyle};
use helper::HelperContext;
mod helper;

// this example is ported from <https://fiddle.skia.org/c/@sum_path_effect>
fn main() {
    // create window and other stuff.
    let ctx = HelperContext::new([800, 600]);

    // setup the paint
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
    paint.set_stroke_width(3.0);
    paint.set_antialias(true);
    paint.set_color(Color::GREEN);
    // create the path effect and set it in paint
    {
        // discrete just means "wobbly" or shanky. imagine if you were to draw a line really really slowly.
        let first = PathEffect::discrete(10.0, 4.0, 0);
        let second = PathEffect::discrete(10.0, 4.0, 1245);
        // sum of path effects just means drawing a path with *both* of those effects (essentially two paths).
        let mut sum = first.create_sum(&second);
        paint.set_path_effect(&mut sum);
    }
    // lets create the star path once and reuse it every frame.
    let star_path = create_star_path();
    ctx.enter_event_loop(|ctx| {
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        canvas.clear(Color::WHITE);
        // draw the paint with our pre-configured paint.
        canvas.draw_path(&star_path, &paint);
    });
}

fn create_star_path() -> SkiaPath {
    // we increase the sizes a little to make it bigger and cover a larger portion of the window
    let r = 115.2 * 2.0;
    let c = 128.0 * 2.0;
    let mut path = SkiaPath::default();
    path.move_to(c + r, c);
    for i in 1..8 {
        let a = 2.6927937 * i as f32;
        path.line_to(c + r * a.cos(), c + r * a.sin());
    }
    path
}
