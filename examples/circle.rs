use ckia::{paint::Paint, shader::Shader, Color, Point, ShaderTileMode};
use helper::HelperContext;

mod helper;

fn main() {
    let ctx = HelperContext::new([800, 600]);
    let mut gradient = Shader::new_linear_gradient(
        &[Point::new(100.0, 100.0), Point::new(300.0, 300.0)],
        &[Color::RED, Color::YELLOW],
        None,
        ShaderTileMode::REPEAT_SK_SHADER_TILEMODE,
        None,
    );
    ctx.enter_event_loop(|ctx| {
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        let mut paint = Paint::default();
        paint.set_antialias(true);
        paint.set_color(Color::BLACK);
        paint.set_shader(Some(&mut gradient));
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
