use ckia::{paint::Paint, Color};
use helper::HelperContext;

mod helper;

fn main() {
    let ctx = HelperContext::new([800, 600]);
    ctx.enter_event_loop(|ctx| {
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        let mut paint = Paint::default();
        paint.set_antialias(true);
        paint.set_color(Color::BLACK);
        let rad = 200.0;
        let cx = 300.0;
        let cy = 200.0;
        canvas.draw_circle(cx, cy, rad, &paint);
        let rad = rad / 2.0;
        paint.set_color(Color::MAGENTA);
        canvas.draw_circle(cx, cy, rad, &paint);
        let rad = rad / 2.0;
        paint.set_color(Color::YELLOW);
        canvas.draw_circle(cx, cy, rad, &paint);
        let rad = rad / 2.0;
        paint.set_color(Color::CYAN);
        canvas.draw_circle(cx, cy, rad, &paint);
    });
}
