use std::ffi::{c_void, CStr};

use ckia::{
    color::ColorSpace,
    data::SkiaData,
    filter::MaskFilter,
    font::Font,
    gr_context::*,
    paint::Paint,
    path_effect::PathEffect,
    surface::{Surface, SurfaceProps},
    typeface::FontMgr,
    BlendMode, BlurStyle, Color, ColorType, GlFramebufferInfo, PaintStyle, PixelGeometry, Rect,
    StrokeCap, StrokeJoin, SurfaceOrigin,
};
use glfw::{Context, PWindow};
use glow::HasContext;
const FIRA_CODE_REGULAR_BYTES: &[u8] = include_bytes!("fira_code_regular.ttf");
fn main() {
    let mut gtx = glfw::init(glfw::fail_on_errors).expect("failed to init glfw");
    gtx.window_hint(glfw::WindowHint::ContextCreationApi(
        glfw::ContextCreationApi::Egl,
    ));
    gtx.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    gtx.window_hint(glfw::WindowHint::SRgbCapable(true));
    gtx.window_hint(glfw::WindowHint::StencilBits(Some(8)));
    gtx.window_hint(glfw::WindowHint::Samples(Some(0)));
    gtx.window_hint(glfw::WindowHint::ScaleToMonitor(true));
    let (mut window, event_receiver) = gtx
        .create_window(800, 600, "skia gl glfw", glfw::WindowMode::Windowed)
        .expect("failed to create window");
    window.set_all_polling(true);
    window.make_current();
    let gl = unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s)) };
    println!("assembling gl interface");
    let interface = unsafe {
        GlInterface::assemble_gl_interface(&mut window as *mut PWindow as _, Some(get_proc_addr))
    };
    println!("making direct context");
    let mut dctx = DirectContext::make_gl(&interface);
    let mut cs = ColorSpace::new_srgb();
    let props = SurfaceProps::new(0, PixelGeometry::UNKNOWN_SK_PIXELGEOMETRY);
    let id = unsafe { gl.get_parameter_i32(glow::FRAMEBUFFER_BINDING) };

    let (width, height) = window.get_framebuffer_size();
    let mut render_target = unsafe {
        BackendRenderTarget::new_gl(
            width,
            height,
            0,
            8,
            &GlFramebufferInfo::new(id as _, glow::SRGB8_ALPHA8, false),
        )
    };
    dbg!(&render_target);
    let mut surface = Surface::new_backend_render_target(
        dctx.as_mut(),
        &render_target,
        SurfaceOrigin::BOTTOM_LEFT_GR_SURFACE_ORIGIN,
        ColorType::SRGBA_8888_SK_COLORTYPE,
        &mut cs,
        &props,
    );
    dbg!(&surface);

    dbg!(interface.validate());
    // for text rendering
    let mut fontmgr = FontMgr::create_custom_dir(".").unwrap();
    let mut fira_data = SkiaData::new_with_copy(FIRA_CODE_REGULAR_BYTES);
    dbg!(fira_data.get_size());
    let mut fira_tf = fontmgr.create_from_data(&mut fira_data, 0).unwrap();
    let fira_font = Font::new_with_values(&mut fira_tf, 64.0, 1.0, 0.0).unwrap();
    let huge_fira_font = Font::new_with_values(&mut fira_tf, 128.0, 2.0, 0.0).unwrap();
    // for special effects
    let blur_mask = MaskFilter::new_blur(BlurStyle::NORMAL_SK_BLUR_STYLE, 1.8);
    let mut dash_effect = PathEffect::create_dash(&[32.0, 4.0], 0.0);
    while !window.should_close() {
        gtx.poll_events();
        for (_, ev) in glfw::flush_messages(&event_receiver) {
            dbg!(&ev);
            match ev {
                glfw::WindowEvent::Close => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                    // recreate the render target and surface
                    gl.viewport(0, 0, w, h);
                    render_target = BackendRenderTarget::new_gl(
                        w,
                        h,
                        0,
                        8,
                        &GlFramebufferInfo::new(id as _, glow::SRGB8_ALPHA8, false),
                    );
                    surface = Surface::new_backend_render_target(
                        dctx.as_mut(),
                        &render_target,
                        SurfaceOrigin::BOTTOM_LEFT_GR_SURFACE_ORIGIN,
                        ColorType::SRGBA_8888_SK_COLORTYPE,
                        &mut cs,
                        &props,
                    );
                },
                glfw::WindowEvent::Key(k, _, _, _) => match k {
                    glfw::Key::Escape => {
                        window.set_should_close(true);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
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
            // canvas.scale(1.5, 1.0);
            paint.set_color(Color::new(128, 0, 0, 0));
            paint.set_path_effect(&mut dash_effect);
            paint.set_stroke_width(10.0);
            paint.set_style(PaintStyle::STROKE_SK_PAINT_STYLE);
            paint.set_stroke_cap(StrokeCap::BUTT_SK_STROKE_CAP);
            paint.set_stroke_join(StrokeJoin::ROUND_SK_STROKE_JOIN);

            canvas.draw_simple_text("Skia", 0.0, 500.0, &huge_fira_font, &paint);
            paint.set_maskfilter(&blur_mask);
            paint.set_stroke_width(20.0);
            canvas.draw_round_rect(&Rect::new(200.0, 100.0, 400.0, 400.0), 64.0, 64.0, &paint);
        }

        dctx.flush_and_submit(true);
        window.swap_buffers();
    }
}
extern "C" fn get_proc_addr(ctx: *mut c_void, sym: *const i8) -> Option<unsafe extern "C" fn()> {
    unsafe {
        assert!(!ctx.is_null(), "context is null in get_proc_addr");

        assert!(!sym.is_null(), "sym pointer is null in get_proc_addr");
        let sym = CStr::from_ptr(sym)
            .to_str()
            .expect("sym is not valid utf-8");
        dbg!(sym);
        let p = (ctx as *mut PWindow)
            .as_mut()
            .unwrap()
            .get_proc_address(sym);

        if p.is_null() {
            println!("{sym} is nullptr");
            None
        } else {
            Some(std::mem::transmute(p))
        }
    }
}
