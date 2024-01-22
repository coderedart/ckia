use ckia::{
    paint::Paint, skottie::SkottieAnimation, sksg::SkSgInvalidationController, Color, Rect,
};
use helper::HelperContext;

mod helper;
const BLUR_SKOTTIE: &str = include_str!("blur_skottie.json");
fn main() {
    // create window, opengl context, skia surface etc..
    let ctx = HelperContext::new([800, 600]);
    // create the data we want to use in *this* example
    let mut animation = SkottieAnimation::new_from_string(BLUR_SKOTTIE).unwrap();
    let mut invalidator = SkSgInvalidationController::default();
    // enter event loop and start rendering frames
    ctx.enter_event_loop(|ctx| {
        // get canvas to draw
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        // clear canvas
        canvas.clear(Color::TRANSPARENT);

        // where to draw animation and how big should it be.
        let mut rect = Rect::new(0.0, 0.0, 400.0, 500.0);
        animation.render(canvas, &mut rect);

        let time = ctx.glfw_context.get_time() as f32;
        animation.seek_frame_time(time, &mut invalidator);
    });
}
