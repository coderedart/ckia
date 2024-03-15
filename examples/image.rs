use ckia::{data::SkiaData, image::Image, paint::Paint, Color, Rect, SamplingOptions};
use helper::HelperContext;

mod helper;

fn main() {
    let mut ctx = HelperContext::new([800, 600]);
    // Photo by Alin Andersen on Unsplash
    let data = include_bytes!("alin-andersen-8Mj83xR2S1I-unsplash.jpg");
    let data = SkiaData::new_with_copy(data);
    let image = Image::new_from_encoded(&data).unwrap().make_texture_image(
        &mut ctx.gl_direct_context,
        true,
        true,
    );
    let image_rect = Rect {
        right: image.get_width() as _,
        bottom: image.get_height() as _,
        ..Default::default()
    };
    assert!(image.is_texture_backed());
    let sampling = SamplingOptions::LINEAR;
    ctx.enter_event_loop(|ctx| {
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        let mut paint = Paint::default();
        paint.set_antialias(true);
        paint.set_color(Color::BLACK);
        let scale = 3.0;
        canvas.draw_image_rect(
            &image,
            &image_rect,
            // we make three times smaller rect than src. src is 1920x1280 which is too big
            // we don't want to hardcode constants, as we need to preserve aspect ratio
            &Rect {
                left: 100.0,
                top: 100.0,
                right: image_rect.right / scale,
                bottom: image_rect.bottom / scale,
            },
            &sampling,
            &paint,
        );
    });
}
