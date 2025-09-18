#[allow(unused)]
use ckia::{
    bindings::gr_gl_textureinfo_t,
    color::ColorSpace,
    filter::ImageFilter,
    gr_context::BackendTexture,
    paragraph::{FontCollection, ParagraphBuider, ParagraphStyle},
    path::SkiaPath,
    picture::PictureRecorder,
    shader::Shader,
    surface::{Surface, SurfaceProps, TextureSurface},
    typeface::FontStyle,
    BlendMode, ClipOp, Color, ColorType, FontStyleSlant, Matrix, PaintStyle, ParagraphTextAlign,
    PathDirection, PixelGeometry, Point, Rect, ShaderTileMode, SurfaceOrigin,
};
use helper::HelperContext;

mod helper;

/// This example is primarily to test the performance difference between native and mlua.
/// mlua will obviously be slower as all ckia calls need to cross over from guest to host side.
/// We will run both examples, and check how many milliseconds it takes to issue ckia draw commands from lua and native.
/// On my pc, native was around 0.23 ms, while lua version took an average of 0.3ms.
/// So, roughly 50% slower.  
pub fn main() {
    use std::time::{Duration, Instant};

    use ckia::{canvas::Canvas, Color};
    use mlua::{Function, Table};
    let start_time = Instant::now();
    let mut frame_times = vec![];
    let mut previous_reset = 0.0;
    let mut frame_time = Duration::ZERO;
    let mut paint = ckia::paint::Paint::default();
    paint.set_antialias(true);
    paint.set_stroke_width(7.0);
    paint.set_color(DARK_SLATE_BLUE);
    /// We will just draw lots of tiles of this size
    const TILE_SIZE: f32 = 100.0;

    let mut gradient_shader = Shader::new_linear_gradient(
        &[
            Point {
                x: TILE_SIZE / 2.0,
                y: TILE_SIZE / 2.0,
            },
            Point {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        ],
        &[VIOLET, YELLOW],
        None,
        ShaderTileMode::MIRROR_SK_SHADER_TILEMODE,
        None,
    );
    let backdrop_filter = ImageFilter::new_blur(
        5.0,
        5.0,
        ShaderTileMode::REPEAT_SK_SHADER_TILEMODE,
        None,
        None,
    );
    let trans_black = BLACK.with_alpha(128);
    let mut shadow = {
        let first_shadow =
            ImageFilter::new_drop_shadow(5.0, 5.0, 4.0, 4.0, trans_black, None, None);
        let second_shadow =
            ImageFilter::new_drop_shadow(-5.0, -5.0, 2.0, 2.0, trans_black, None, None);
        first_shadow.compose(&second_shadow)
    };
    let mut only_shadow =
        ImageFilter::new_drop_shadow_only(2.0, 2.0, 5.0, 5.0, trans_black, None, None);
    let mut pb = None;
    let mut para = None;
    let mut text_paint = ckia::paint::Paint::default();
    let mut para_or_not = false;
    let mut tsurface: Option<TextureSurface> = None;
    let background_rect = Rect::new(0., 0., 300., 300.);
    let background_color = LIGHT_SKY_BLUE;
    let background_clip_rect = Rect::new(TILE_SIZE / 4.0, 60., TILE_SIZE * 3. / 4., 30.);

    let background_clip_path = {
        let mut p = SkiaPath::default();
        p.add_rounded_rect(
            &background_clip_rect,
            8.,
            8.,
            PathDirection::CW_SK_PATH_DIRECTION,
        );
        p.close();
        p
    };
    let inner_rect = Rect::new(125., 125., 175., 175.);
    helper::HelperContext::new([800, 600]).enter_event_loop(|htx| {
        let HelperContext {
            surface,
            events,
            glfw_context,
            fontmgr,

            scale,
            fira_font_scaled,
            ..
        } = htx;
        let fira_font = fira_font_scaled;
        if pb.is_none() {
            let mut ps = ParagraphStyle::default();
            ps.set_text_align(ParagraphTextAlign::CENTER_TEXT_ALIGN);
            let mut ts = ps.get_text_style();
            ts.set_foreground(&paint);
            text_paint = paint.clone();
            ts.set_font_style(&FontStyle::new(
                400,
                16,
                FontStyleSlant::UPRIGHT_SK_FONT_STYLE_SLANT,
            ));

            ts.set_font_size(16.0 * scale[0]);
            ps.set_text_style(&ts);
            ps.set_text_align(ckia::ParagraphTextAlign::START_TEXT_ALIGN);
            let mut fc = FontCollection::default();
            fc.set_default_font_manager(fontmgr);
            let mut builder = ParagraphBuider::new(&ps, &fc);
            builder.add_text("Ckia Native");
            let mut new_para = builder.build();
            new_para.layout(1000.0);
            pb = Some(builder);
            para = Some(new_para);
        }

        let current_time = glfw_context.get_time();
        if events.iter().any(|e| match e {
            glfw::WindowEvent::MouseButton(_, glfw::Action::Release, _) => true,
            _ => false,
        }) {
            if para_or_not {
                tracing::info!("switching to canvas.draw_simple_text");
            } else {
                tracing::info!("switching to paragraph painting");
            }
            para_or_not = !para_or_not;
        }
        // let mut tsurface_canvas = tsurface.as_mut().unwrap().get_canvas();
        // let mut tex_canvas = tsurface_canvas.as_mut();
        // assert!(scale[0] >= 1.0);
        // tex_canvas.save();
        // tex_canvas.scale(scale[0], scale[1]);

        // tex_canvas.clear(Color::TRANSPARENT);

        let mut surface_canvas = surface.get_canvas();
        let mut canvas = surface_canvas.as_mut();
        canvas.save();
        canvas.scale(ckia::Vector::new(scale[0], scale[1]));
        canvas.clear(Color::TRANSPARENT);

        let i = Instant::now();
        // let mut p = PictureRecorder::default();
        // {
        //     let mut pcanvas = p.begin_recording(&Rect::new(0.0, 0.0, 3840.0, 2100.0));
        //     let mut canvas = pcanvas.as_mut();
        //     {

        //     }
        // }
        // let pict = p.end_recording();
        let local_clip_bounds = canvas.get_local_clip_bounds().unwrap();
        let x_tiles =
            ((local_clip_bounds.right - local_clip_bounds.left) / TILE_SIZE).ceil() as i32;
        let y_tiles =
            ((local_clip_bounds.bottom - local_clip_bounds.top) / TILE_SIZE).ceil() as i32;
        for x_index in 0..x_tiles {
            let x_offset = x_index as f32 * TILE_SIZE;
            for y_offset in 0..y_tiles {
                let y_offset = y_offset as f32 * TILE_SIZE;
                let skia_stack = canvas.save();
                canvas.translate(ckia::Vector::new(x_offset, y_offset));
                // gl_canvas.draw_picture(&pict, &Matrix::IDENTITY, &paint);
                // let main_canvas = main_canvas;
                // background
                paint.set_color(background_color);
                paint.set_style(PaintStyle::FILL_SK_PAINT_STYLE);
                canvas.draw_rect(&background_rect, &paint);
                // gradient circle
                paint.set_shader(Some(&mut gradient_shader));
                canvas.draw_circle(
                    ckia::Vector::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                    ((current_time as f32).sin().abs() * TILE_SIZE / 2.0),
                    &paint,
                );
                paint.set_shader(None);
                // draw foreground rect that shows how shadow works
                // paint.set_image_filter(Some(&mut shadow));
                paint.set_style(PaintStyle::STROKE_AND_FILL_SK_PAINT_STYLE);
                canvas.draw_round_rect(&inner_rect, 3.0, 3.0, &paint);
                // paint.set_image_filter(None);
                // only shadow
                // paint.set_image_filter(Some(&mut only_shadow));
                // paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
                // canvas.draw_round_rect(&background_clip_rect, 8.0, 8.0, &paint);
                // paint.set_image_filter(None);
                canvas.save();
                {
                    // canvas.clip_path_with_operation(
                    //     &background_clip_path,
                    //     ClipOp::INTERSECT_SK_CLIPOP,
                    //     true,
                    // );
                    // canvas.save_layer(
                    //     Some(&background_clip_rect),
                    //     None,
                    //     Some(&backdrop_filter),
                    //     true,
                    // );
                    paint.set_color(FLORAL_WHITE.with_alpha(80));
                    paint.set_style(PaintStyle::STROKE_AND_FILL_SK_PAINT_STYLE);
                    paint.set_blendmode(BlendMode::SRCOVER_SK_BLENDMODE);
                    canvas.draw_round_rect(&background_clip_rect, 8., 8., &paint);
                    // canvas.draw_color(FLORAL_WHITE.with_alpha(140), BlendMode::SRCOVER_SK_BLENDMODE);
                    // main_canvas.restore();
                }
                canvas.restore();
                {
                    canvas.save();
                    // canvas.scale(2.0, 2.0);
                    let time_text = format!("{:.2}", frame_time.as_secs_f64() * 1000.0);
                    if para_or_not {
                        para.as_mut().unwrap().paint(canvas, 50., 160.);
                        let mut pb = pb.as_mut().unwrap();
                        pb.reset();
                        pb.add_text(&time_text);
                        let mut ft = pb.build();
                        ft.layout(TILE_SIZE);
                        ft.paint(canvas, 0., 0.);
                    } else {
                        canvas.draw_simple_text("Ckia Native", 15., 50., fira_font, &text_paint);
                        canvas.draw_simple_text(&time_text, 0., 0., fira_font, &text_paint);
                    }
                    canvas.restore();
                }
                canvas.restore_to_count(skia_stack);
            }
        }

        // tsurface
        //     .as_mut()
        //     .unwrap()
        //     .draw_to(&mut canvas, 0.0, 0.0, &paint);
        canvas.restore_to_count(0);
        frame_times.push(i.elapsed());
        frame_time = (frame_time + i.elapsed()) / 2;
        if current_time - previous_reset > 1.0 {
            previous_reset = current_time;
            println!(
                "average ckia time (ms): {:.2}",
                frame_time.as_secs_f64() * 1000.0
            );
            // frame_time = Duration::ZERO
        }
    });

    println!("program duration: {}", start_time.elapsed().as_secs_f32());
    println!("total frames: {}", frame_times.len());
    let average_time =
        (frame_times.iter().sum::<Duration>().as_secs_f32() * 1000.0) / frame_times.len() as f32;
    println!("average frame  time: {}", average_time);
}
// const
const ALICE_BLUE: Color = Color(0xFFF0F8FF);
const ANTIQUE_WHITE: Color = Color(0xFFFAEBD7);
const AQUA: Color = Color(0xFF00FFFF);
const AQUAMARINE: Color = Color(0xFF7FFFD4);
const AZURE: Color = Color(0xFFF0FFFF);
const BEIGE: Color = Color(0xFFF5F5DC);
const BISQUE: Color = Color(0xFFFFE4C4);
const BLACK: Color = Color(0xFF000000);
const BLANCHED_ALMOND: Color = Color(0xFFFFEBCD);
const BLUE: Color = Color(0xFF0000FF);
const BLUE_VIOLET: Color = Color(0xFF8A2BE2);
const BROWN: Color = Color(0xFFA52A2A);
const BURLY_WOOD: Color = Color(0xFFDEB887);
const CADET_BLUE: Color = Color(0xFF5F9EA0);
const CHARTREUSE: Color = Color(0xFF7FFF00);
const CHOCOLATE: Color = Color(0xFFD2691E);
const CORAL: Color = Color(0xFFFF7F50);
const CORNFLOWER_BLUE: Color = Color(0xFF6495ED);
const CORNSILK: Color = Color(0xFFFFF8DC);
const CRIMSON: Color = Color(0xFFDC143C);
const CYAN: Color = Color(0xFF00FFFF);
const DARK_BLUE: Color = Color(0xFF00008B);
const DARK_CYAN: Color = Color(0xFF008B8B);
const DARK_GOLDEN_ROD: Color = Color(0xFFB8860B);
const DARK_GRAY: Color = Color(0xFFA9A9A9);
const DARK_GREEN: Color = Color(0xFF006400);
const DARK_KHAKI: Color = Color(0xFFBDB76B);
const DARK_MAGENTA: Color = Color(0xFF8B008B);
const DARK_OLIVE_GREEN: Color = Color(0xFF556B2F);
const DARKORANGE: Color = Color(0xFFFF8C00);
const DARK_ORCHID: Color = Color(0xFF9932CC);
const DARK_RED: Color = Color(0xFF8B0000);
const DARK_SALMON: Color = Color(0xFFE9967A);
const DARK_SEA_GREEN: Color = Color(0xFF8FBC8F);
const DARK_SLATE_BLUE: Color = Color(0xFF483D8B);
const DARK_SLATE_GRAY: Color = Color(0xFF2F4F4F);
const DARK_TURQUOISE: Color = Color(0xFF00CED1);
const DARK_VIOLET: Color = Color(0xFF9400D3);
const DEEP_PINK: Color = Color(0xFFFF1493);
const DEEP_SKY_BLUE: Color = Color(0xFF00BFFF);
const DIM_GRAY: Color = Color(0xFF696969);
const DODGER_BLUE: Color = Color(0xFF1E90FF);
const FIRE_BRICK: Color = Color(0xFFB22222);
const FLORAL_WHITE: Color = Color(0xFFFFFAF0);
const FOREST_GREEN: Color = Color(0xFF228B22);
const FUCHSIA: Color = Color(0xFFFF00FF);
const GAINSBORO: Color = Color(0xFFDCDCDC);
const GHOST_WHITE: Color = Color(0xFFF8F8FF);
const GOLD: Color = Color(0xFFFFD700);
const GOLDEN_ROD: Color = Color(0xFFDAA520);
const GRAY: Color = Color(0xFF808080);
const GREEN: Color = Color(0xFF008000);
const GREEN_YELLOW: Color = Color(0xFFADFF2F);
const HONEY_DEW: Color = Color(0xFFF0FFF0);
const HOT_PINK: Color = Color(0xFFFF69B4);
const INDIAN_RED: Color = Color(0xFFCD5C5C);
const INDIGO: Color = Color(0xFF4B0082);
const IVORY: Color = Color(0xFFFFFFF0);
const KHAKI: Color = Color(0xFFF0E68C);
const LAVENDER: Color = Color(0xFFE6E6FA);
const LAVENDER_BLUSH: Color = Color(0xFFFFF0F5);
const LAWN_GREEN: Color = Color(0xFF7CFC00);
const LEMON_CHIFFON: Color = Color(0xFFFFFACD);
const LIGHT_BLUE: Color = Color(0xFFADD8E6);
const LIGHT_CORAL: Color = Color(0xFFF08080);
const LIGHT_CYAN: Color = Color(0xFFE0FFFF);
const LIGHT_GOLDEN_ROD_YELLOW: Color = Color(0xFFFAFAD2);
const LIGHT_GREY: Color = Color(0xFFD3D3D3);
const LIGHT_GREEN: Color = Color(0xFF90EE90);
const LIGHT_PINK: Color = Color(0xFFFFB6C1);
const LIGHT_SALMON: Color = Color(0xFFFFA07A);
const LIGHT_SEA_GREEN: Color = Color(0xFF20B2AA);
const LIGHT_SKY_BLUE: Color = Color(0xFF87CEFA);
const LIGHT_SLATE_GRAY: Color = Color(0xFF778899);
const LIGHT_STEEL_BLUE: Color = Color(0xFFB0C4DE);
const LIGHT_YELLOW: Color = Color(0xFFFFFFE0);
const LIME: Color = Color(0xFF00FF00);
const LIME_GREEN: Color = Color(0xFF32CD32);
const LINEN: Color = Color(0xFFFAF0E6);
const MAGENTA: Color = Color(0xFFFF00FF);
const MAROON: Color = Color(0xFF800000);
const MEDIUM_AQUA_MARINE: Color = Color(0xFF66CDAA);
const MEDIUM_BLUE: Color = Color(0xFF0000CD);
const MEDIUM_ORCHID: Color = Color(0xFFBA55D3);
const MEDIUM_PURPLE: Color = Color(0xFF9370D8);
const MEDIUM_SEA_GREEN: Color = Color(0xFF3CB371);
const MEDIUM_SLATE_BLUE: Color = Color(0xFF7B68EE);
const MEDIUM_SPRING_GREEN: Color = Color(0xFF00FA9A);
const MEDIUM_TURQUOISE: Color = Color(0xFF48D1CC);
const MEDIUM_VIOLET_RED: Color = Color(0xFFC71585);
const MIDNIGHT_BLUE: Color = Color(0xFF191970);
const MINT_CREAM: Color = Color(0xFFF5FFFA);
const MISTY_ROSE: Color = Color(0xFFFFE4E1);
const MOCCASIN: Color = Color(0xFFFFE4B5);
const NAVAJO_WHITE: Color = Color(0xFFFFDEAD);
const NAVY: Color = Color(0xFF000080);
const OLD_LACE: Color = Color(0xFFFDF5E6);
const OLIVE: Color = Color(0xFF808000);
const OLIVE_DRAB: Color = Color(0xFF6B8E23);
const ORANGE: Color = Color(0xFFFFA500);
const ORANGE_RED: Color = Color(0xFFFF4500);
const ORCHID: Color = Color(0xFFDA70D6);
const PALE_GOLDEN_ROD: Color = Color(0xFFEEE8AA);
const PALE_GREEN: Color = Color(0xFF98FB98);
const PALE_TURQUOISE: Color = Color(0xFFAFEEEE);
const PALE_VIOLET_RED: Color = Color(0xFFD87093);
const PAPAYA_WHIP: Color = Color(0xFFFFEFD5);
const PEACH_PUFF: Color = Color(0xFFFFDAB9);
const PERU: Color = Color(0xFFCD853F);
const PINK: Color = Color(0xFFFFC0CB);
const PLUM: Color = Color(0xFFDDA0DD);
const POWDER_BLUE: Color = Color(0xFFB0E0E6);
const PURPLE: Color = Color(0xFF800080);
const RED: Color = Color(0xFFFF0000);
const ROSY_BROWN: Color = Color(0xFFBC8F8F);
const ROYAL_BLUE: Color = Color(0xFF4169E1);
const SADDLE_BROWN: Color = Color(0xFF8B4513);
const SALMON: Color = Color(0xFFFA8072);
const SANDY_BROWN: Color = Color(0xFFF4A460);
const SEA_GREEN: Color = Color(0xFF2E8B57);
const SEA_SHELL: Color = Color(0xFFFFF5EE);
const SIENNA: Color = Color(0xFFA0522D);
const SILVER: Color = Color(0xFFC0C0C0);
const SKY_BLUE: Color = Color(0xFF87CEEB);
const SLATE_BLUE: Color = Color(0xFF6A5ACD);
const SLATE_GRAY: Color = Color(0xFF708090);
const SNOW: Color = Color(0xFFFFFAFA);
const SPRING_GREEN: Color = Color(0xFF00FF7F);
const STEEL_BLUE: Color = Color(0xFF4682B4);
const TAN: Color = Color(0xFFD2B48C);
const TEAL: Color = Color(0xFF008080);
const THISTLE: Color = Color(0xFFD8BFD8);
const TOMATO: Color = Color(0xFFFF6347);
const TURQUOISE: Color = Color(0xFF40E0D0);
const VIOLET: Color = Color(0xFFEE82EE);
const WHEAT: Color = Color(0xFFF5DEB3);
const WHITE: Color = Color(0xFFFFFFFF);
const WHITE_SMOKE: Color = Color(0xFFF5F5F5);
const YELLOW: Color = Color(0xFFFFFF00);
const YELLOW_GREEN: Color = Color(0xFF9ACD3);
