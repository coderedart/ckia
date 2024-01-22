mod helper;
use ckia::{filter::MaskFilter, paint::Paint, path_effect::PathEffect, *};
use helper::HelperContext;
fn main() {
    let helper = helper::HelperContext::new([800, 600]);
    // for special effects
    let blur_mask = MaskFilter::new_blur(BlurStyle::NORMAL_SK_BLUR_STYLE, 1.8);
    let mut dash_effect = PathEffect::create_dash(&[32.0, 4.0], 0.0);
    helper.enter_event_loop(move |ctx| {
        let HelperContext {
            fira_font,
            fira_font_huge,
            surface,
            ..
        } = ctx;
        {
            let mut canvas = surface.get_canvas();
            let canvas = canvas.as_mut();
            canvas.draw_color(Color::LTGRAY, BlendMode::SRC_SK_BLENDMODE);
            // let scale = window.get_content_scale();
            // canvas.scale(scale.0, scale.1);
            let mut paint = Paint::default();
            paint.set_antialias(true);
            paint.set_color(Color::GREEN);
            paint.set_stroke_width(32.0);
            // don't fill the shapes. just the outlines.
            paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
            canvas.draw_circle(100.0, 100.0, 100.0, &paint);
            let rect = Rect::new(300.0, 200.0, 700.0, 600.0);
            canvas.draw_round_rect(&rect, 40.0, 20.0, &paint);
            paint.set_color(Color::BLACK);
            paint.set_stroke_width(2.0);
            canvas.draw_simple_text("Hello everyone", 50.0, 200.0, &fira_font, &paint);
            paint.set_style(PaintStyle::STROKE_AND_FILL_SK_PAINT_STYLE);
            canvas.draw_simple_text("This is Fira Code font", 50.0, 300.0, &fira_font, &paint);
            // setup some special effects for text and another rect
            paint.set_color(Color::new(128, 0, 0, 0));
            paint.set_path_effect(&mut dash_effect);
            paint.set_stroke_width(10.0);
            paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
            paint.set_stroke_cap(StrokeCap::BUTT_SK_STROKE_CAP);
            paint.set_stroke_join(StrokeJoin::ROUND_SK_STROKE_JOIN);

            canvas.draw_simple_text("Skia", 0.0, 500.0, &fira_font_huge, &paint);
            paint.set_maskfilter(&blur_mask);
            paint.set_stroke_width(20.0);
            canvas.draw_round_rect(&Rect::new(200.0, 100.0, 400.0, 400.0), 64.0, 64.0, &paint);
        }
    });
}
