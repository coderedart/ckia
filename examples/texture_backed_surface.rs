#![allow(unused)]
use std::sync::Arc;

use ckia::{
    bindings::gr_gl_textureinfo_t,
    color::ColorSpace,
    filter::ImageFilter,
    gr_context::{BackendTexture, DirectContext},
    paragraph::{FontCollection, ParagraphBuider, ParagraphStyle},
    path::SkiaPath,
    picture::PictureRecorder,
    shader::Shader,
    surface::{Surface, SurfaceProps, TextureSurface},
    typeface::FontStyle,
    BlendMode, ClipOp, Color, ColorType, FontStyleSlant, Matrix, PaintStyle, ParagraphTextAlign,
    PathDirection, PixelGeometry, Point, Rect, ShaderTileMode, SurfaceOrigin,
};
use glow::HasContext;
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
    paint.set_color(Color::MAGENTA.with_alpha(60));

    let trans_black = Color::BLACK.with_alpha(128);

    let mut tex_surface = None;
    let mut fb_size = (0, 0);
    helper::HelperContext::new([800, 600]).enter_event_loop(|htx| {
        let HelperContext {
            surface,
            events,
            glfw_context,
            fontmgr,
            fira_font,
            lua,
            fira_typface,
            fira_font_huge,
            render_target,
            gl_direct_context,
            gl_interface,
            glow_context,
            events_receiver,
            window,
            scale,
        } = htx;

        // if frame_buffer size has changed, we need to recreate the surface backed by texture
        let current_fb_size = window.get_framebuffer_size();
        if fb_size != current_fb_size {
            fb_size = current_fb_size;
            unsafe {
                tex_surface = Some(create_texture_backed_surface(
                    glow_context.clone(),
                    gl_direct_context,
                    current_fb_size,
                ))
            }
        }

        let current_time = glfw_context.get_time();

        let mut tsurface_canvas = tex_surface.as_mut().unwrap().get_canvas();
        let mut tex_canvas = tsurface_canvas.as_mut();
        assert!(scale[0] >= 1.0);
        tex_canvas.save();
        tex_canvas.scale(scale[0], scale[1]);

        tex_canvas.clear(Color::TRANSPARENT);

        let mut surface_canvas = surface.get_canvas();
        let mut main_canvas = surface_canvas.as_mut();
        main_canvas.save();
        main_canvas.clear(Color::TRANSPARENT);

        let i = Instant::now();
        // let mut p = PictureRecorder::default();
        // {
        //     let mut pcanvas = p.begin_recording(&Rect::new(0.0, 0.0, 3840.0, 2100.0));
        //     let mut canvas = pcanvas.as_mut();
        //     {

        //     }
        // }
        // let pict = p.end_recording();
        for x_offset in [0.0, 300.0, 600.0, 900.0, 1200.0, 1500.0] {
            for y_offset in [0.0, 300.0, 600.0] {
                let skia_stack = tex_canvas.save();
                {
                    tex_canvas.translate(x_offset, y_offset);
                    paint.set_color(Color::WHITE);
                    paint.set_style(PaintStyle::FILL_SK_PAINT_STYLE);
                    tex_canvas.draw_circle(
                        150.0,
                        150.0,
                        ((current_time as f32).sin().abs() * 75.0) + 50.0,
                        &paint,
                    );
                }
                assert_eq!(tex_canvas.get_save_count(), skia_stack + 1);
                tex_canvas.restore_to_count(skia_stack);
            }
        }

        tex_surface
            .as_mut()
            .unwrap()
            .draw_to(&mut main_canvas, 0.0, 0.0, &paint);
        tex_canvas.restore_to_count(0);
        main_canvas.restore_to_count(0);
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

unsafe fn create_texture_backed_surface(
    glow_context: Arc<glow::Context>,
    gl_direct_context: &mut DirectContext,
    fb_size: (i32, i32),
) -> TextureSurface {
    let raw_tex = glow_context.create_texture().unwrap();
    let previous_raw_text = glow_context.get_parameter_i32(glow::TEXTURE_BINDING_2D);
    dbg!(previous_raw_text);
    glow_context.active_texture(glow::TEXTURE0);
    glow_context.bind_texture(glow::TEXTURE_2D, Some(raw_tex));
    let (surface_width, surface_height) = fb_size;
    glow_context.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA8.try_into().unwrap(),
        surface_width,
        surface_height,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&vec![0u8; (surface_width * surface_height * 4) as usize]),
    );

    glow_context.bind_texture(
        glow::TEXTURE_2D,
        (previous_raw_text != 0)
            .then(|| glow::NativeTexture((previous_raw_text as u32).try_into().unwrap())),
    );
    let backend_texture = BackendTexture::new_gl(
        surface_width,
        surface_height,
        false,
        &gr_gl_textureinfo_t {
            fTarget: glow::TEXTURE_2D,
            fID: raw_tex.0.into(),
            fFormat: glow::RGBA8,
            fProtected: false,
        },
        Some(Box::new(move || unsafe {
            glow_context.delete_texture(raw_tex);
        })),
    )
    .unwrap();
    let tex = TextureSurface::new_backend_texture(
        gl_direct_context.as_mut(),
        backend_texture,
        SurfaceOrigin::BOTTOM_LEFT_GR_SURFACE_ORIGIN,
        0,
        ColorType::RGBA_8888_SK_COLORTYPE,
        &mut ColorSpace::new_srgb(),
        &SurfaceProps::new(0, PixelGeometry::UNKNOWN_SK_PIXELGEOMETRY),
    )
    .unwrap();
    return tex;
}
