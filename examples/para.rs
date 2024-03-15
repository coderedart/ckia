mod helper;
use ckia::{
    paint::Paint,
    paragraph::{FontCollection, ParagraphBuider, ParagraphStyle},
    *,
};
use helper::HelperContext;
const POEM: &'static str = r#"Looking up at the stars, I know quite well
That, for all they care, I can go to hell,
But on earth indifference is the least
We have to dread from man or beast."#;

fn main() {
    let helper = helper::HelperContext::new([800, 600]);

    let mut paint = Paint::default();
    paint.set_antialias(true);
    paint.set_color(Color::BLACK);
    paint.set_stroke_width(2.0);
    paint.set_style(PaintStyle::FILL_SK_PAINT_STYLE);
    // font collection for paragraph layout/fallbacks
    let mut fc = FontCollection::default();
    fc.set_default_font_manager(&helper.fontmgr);

    let mut para = None;
    helper.enter_event_loop(move |ctx| {
        let HelperContext {
            fira_font, surface, ..
        } = ctx;
        // build paragraph only once
        let poem = para.get_or_insert_with(|| {
            // paragraph style
            let mut style = ParagraphStyle::default();
            // text style for paragraph
            let mut ts = style.get_text_style();
            paint.set_color(Color::YELLOW);
            ts.set_background(&paint);
            ts.set_color(Color::BLACK);
            paint.set_color(Color::BLACK);
            ts.set_foreground(&paint);
            style.set_text_style(&ts);

            // builder. call reset to use the default style. or push style for your own custom thing.
            let mut builder = ParagraphBuider::new(&style, &fc);
            builder.add_text(POEM);
            let mut poem = builder.build();
            poem.update_font_size(0, POEM.len(), 32.0);
            poem.layout(1000.0);
            poem
        });

        {
            let mut canvas = surface.get_canvas();
            let canvas = canvas.as_mut();
            canvas.draw_color(Color::LTGRAY, BlendMode::SRC_SK_BLENDMODE);

            poem.paint(canvas, 0.0, 0.0);

            let mut y = 200.0;
            for line in POEM.lines() {
                canvas.draw_simple_text(line, 0.0, y, &fira_font, &paint);
                y += 40.0;
            }
        }
    });
}
