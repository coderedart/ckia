mod helper;
use ckia::{
    filter::MaskFilter,
    paint::Paint,
    paragraph::{FontCollection, ParagraphBuider, ParagraphStyle},
    path_effect::PathEffect,
    *,
};
use helper::HelperContext;
fn main() {
    let helper = helper::HelperContext::new([800, 600]);
    // for special effects
    let mut blur_mask = MaskFilter::new_blur(BlurStyle::NORMAL_SK_BLUR_STYLE, 1.8);
    let mut dash_effect = PathEffect::create_dash(&[32.0, 4.0], 0.0);

    let mut collection = FontCollection::default();
    collection.enable_font_fallback();
    collection.set_default_font_manager(&helper.fontmgr);

    let mut font_size = 18.0;
    let mut para = None;
    helper.enter_event_loop(|ctx| {
        let HelperContext {
            fira_font,
            fira_font_huge,
            surface,
            events,
            fontmgr,
            fira_typface,
            fira_font_scaled,
            render_target,
            gl_direct_context,
            gl_interface,
            glow_context,
            events_receiver,
            window,
            glfw_context,
            scale,
        } = ctx;
        for ev in events.iter() {
            match ev {
                glfw::WindowEvent::Scroll(x, y) => {
                    font_size += *y as f32;
                    dbg!(font_size);
                    let mut style = ParagraphStyle::default();
                    let mut ts = style.get_text_style();
                    ts.set_font_size(font_size);
                    ts.set_color(Color::BLACK);
                    style.set_text_style(&ts);
                    let mut builder = ParagraphBuider::new(&style, &collection);
                    builder.add_text(TEXT);
                    para = {
                        let mut p = builder.build();
                        p.layout(window.get_size().1 as f32);
                        p.into()
                    };
                }
                _ => {}
            }
        }
        {
            let mut canvas = surface.get_canvas();
            let canvas = canvas.as_mut();
            canvas.draw_color(Color::WHITE, BlendMode::SRC_SK_BLENDMODE);
            // let scale = window.get_content_scale();
            // canvas.scale(scale.0, scale.1);
            // let mut paint = Paint::default();
            // paint.set_antialias(true);
            // paint.set_color(Color::GREEN);
            // paint.set_stroke_width(32.0);
            // // don't fill the shapes. just the outlines.
            // paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
            // canvas.draw_circle(ckia::Vector::new(100.0, 100.0), 100.0, &paint);
            // let rect = Rect::new(300.0, 200.0, 700.0, 600.0);
            // canvas.draw_round_rect(&rect, 40.0, 20.0, &paint);
            // paint.set_color(Color::BLACK);
            // paint.set_stroke_width(2.0);
            // canvas.draw_simple_text("Hello everyone", 50.0, 200.0, fira_font, &paint);
            // paint.set_style(PaintStyle::STROKE_AND_FILL_SK_PAINT_STYLE);
            // canvas.draw_simple_text("This is Fira Code font", 50.0, 300.0, fira_font, &paint);
            // // setup some special effects for text and another rect
            // paint.set_color(Color::new(128, 0, 0, 0));
            // paint.set_path_effect(Some(&mut dash_effect));
            // paint.set_stroke_width(10.0);
            // paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
            // paint.set_stroke_cap(StrokeCap::BUTT_SK_STROKE_CAP);
            // paint.set_stroke_join(StrokeJoin::ROUND_SK_STROKE_JOIN);

            // canvas.draw_simple_text("Skia", 0.0, 500.0, fira_font_huge, &paint);
            // paint.set_maskfilter(Some(&mut blur_mask));
            // paint.set_stroke_width(20.0);
            // canvas.draw_round_rect(&Rect::new(200.0, 100.0, 400.0, 400.0), 64.0, 64.0, &paint);
            if let Some(para) = para.as_mut() {
                para.paint(canvas, 0.0, 0.0);
            }
        }
    });
}

const TEXT: &str = r#"
Traditionally, text is composed to create a readable, coherent, and visually satisfying typeface
that works invisibly, without the awareness of the reader. Even distribution of typeset material,
with a minimum of distractions and anomalies, is aimed at producing clarity and transparency.
Choice of typeface(s) is the primary aspect of text typography—prose fiction, non-fiction,
editorial, educational, religious, scientific, spiritual, and commercial writing all have differing
characteristics and requirements of appropriate typefaces and their fonts or styles.
"#;
/*
مرئية وسهلة قراءة وجذابة. ترتيب الحروف يشمل كل من اختيار عائلة الخط وحجم وطول الخط والمسافة بين السطور

مرئية وسهلة قراءة وجذابة. ترتيب الحروف يشمل كل من اختيار (asdasdasdasdasdasd) عائلة الخط وحجم وطول الخط والمسافة بين السطور

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur in nisi at ligula lobortis pretium. Sed vel eros tincidunt, fermentum metus sit amet, accumsan massa. Vestibulum sed elit et purus suscipit
Sed at gravida lectus. Duis eu nisl non sem lobortis rutrum. Sed non mauris urna. Pellentesque suscipit nec odio eu varius. Quisque lobortis elit in finibus vulputate. Mauris quis gravida libero.
Etiam non malesuada felis, nec fringilla quam.

"#;
*/
