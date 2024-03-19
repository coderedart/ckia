use ckia::{
    data::SkiaData, filter::ImageFilter, image::Image, paint::Paint, path::SkiaPath, ClipOp, Color,
    PaintStyle, PathDirection, Rect, SamplingOptions, ShaderTileMode,
};
use helper::HelperContext;

mod helper;

fn main() {
    let mut ctx = HelperContext::new([800, 600]);
    let data = include_bytes!("alin-andersen-8Mj83xR2S1I-unsplash.jpg");
    let data = SkiaData::new_with_copy(data);
    let image = Image::new_from_encoded(&data).unwrap().make_texture_image(
        &mut ctx.gl_direct_context,
        true,
        true,
    );

    assert!(image.is_texture_backed());
    let blur_filter = ImageFilter::new_blur(
        10.0,
        10.0,
        ShaderTileMode::DECAL_SK_SHADER_TILEMODE,
        None,
        None,
    );
    let mut shadow = ImageFilter::new_drop_shadow_only(
        2.0,
        2.0,
        5.0,
        5.0,
        Color::BLACK.with_alpha(128),
        None,
        None,
    );
    let clip_rect = Rect::new(100.0, 100.0, 400.0, 300.0);
    let mut clip_path = SkiaPath::default();
    clip_path.add_rounded_rect(&clip_rect, 10.0, 10.0, PathDirection::CW_SK_PATH_DIRECTION);
    clip_path.close();
    ctx.enter_event_loop(|ctx| {
        let mut surface_canvas = ctx.surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        canvas.clear(Color::TRANSPARENT);
        let mut paint = Paint::default();
        paint.set_antialias(true);
        {
            canvas.save();
            canvas.scale(0.4, 0.4);
            canvas.draw_image(&image, 0.0, 0.0, &SamplingOptions::LINEAR, &paint);
            canvas.restore();
        }

        // draw shadow
        paint.set_image_filter(Some(&mut shadow));
        paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
        paint.set_stroke_width(8.0);
        canvas.draw_round_rect(&clip_rect, 2.0, 2.0, &paint);
        canvas.save();
        {
            // set clip for the area you want to blur
            // save and restore with blur filter. done
            canvas.clip_path_with_operation(&clip_path, ClipOp::INTERSECT_SK_CLIPOP, true);
            canvas.save_layer(Some(&clip_rect), None, Some(&blur_filter), false);
            canvas.restore();
        }
        canvas.restore();
    });
}
