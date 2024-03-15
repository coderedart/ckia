#![allow(unused)]
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
    typeface::{FontMgr, Typeface},
    BlendMode, BlurStyle, Color, ColorType, GlFramebufferInfo, PaintStyle, PixelGeometry, Rect,
    StrokeCap, StrokeJoin, SurfaceOrigin,
};
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use glow::HasContext;
use mlua::Lua;
const FIRA_CODE_REGULAR_BYTES: &[u8] = include_bytes!("fira_code_regular.ttf");

extern "C" fn get_proc_addr(ctx: *mut c_void, sym: *const i8) -> Option<unsafe extern "C" fn()> {
    unsafe {
        assert!(!ctx.is_null(), "context is null in get_proc_addr");

        assert!(!sym.is_null(), "sym pointer is null in get_proc_addr");
        let sym = CStr::from_ptr(sym)
            .to_str()
            .expect("sym is not valid utf-8");
        let p = (ctx as *mut PWindow)
            .as_mut()
            .unwrap()
            .get_proc_address(sym);

        if p.is_null() {
            eprintln!("{sym} is nullptr");
            None
        } else {
            Some(std::mem::transmute(p))
        }
    }
}
fn create_window(size: [u32; 2]) -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
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
        .create_window(
            size[0] as _,
            size[1] as _,
            "skia gl glfw",
            glfw::WindowMode::Windowed,
        )
        .expect("failed to create window");
    window.set_all_polling(true);
    window.make_current();
    (gtx, window, event_receiver)
}
fn create_gl_interface_and_direct_context(window: &mut PWindow) -> (GlInterface, DirectContext) {
    println!("assembling gl interface");
    let interface =
        unsafe { GlInterface::new_load_with(|proc_name| window.get_proc_address(proc_name)) };
    dbg!(interface.validate());
    println!("making direct context");
    let dctx = DirectContext::make_gl(&interface);
    (interface, dctx)
}
fn create_render_target_and_surface(
    window: &mut PWindow,
    glow_context: &glow::Context,
    gl_direct_context: &mut DirectContext,
) -> (BackendRenderTarget, Surface) {
    let mut cs = ColorSpace::new_srgb();
    let props = SurfaceProps::new(0, PixelGeometry::UNKNOWN_SK_PIXELGEOMETRY);
    let id = unsafe { glow_context.get_parameter_i32(glow::FRAMEBUFFER_BINDING) };

    let (width, height) = window.get_framebuffer_size();
    println!("creating backend render target with {width} as width and {height} as height");
    let render_target = unsafe {
        BackendRenderTarget::new_gl(
            width,
            height,
            0,
            8,
            &GlFramebufferInfo::new(id as _, glow::SRGB8_ALPHA8, false),
        )
    };
    let surface = Surface::new_backend_render_target(
        gl_direct_context.as_mut(),
        &render_target,
        SurfaceOrigin::BOTTOM_LEFT_GR_SURFACE_ORIGIN,
        ColorType::SRGBA_8888_SK_COLORTYPE,
        &mut cs,
        &props,
    );
    (render_target, surface)
}
pub struct HelperContext {
    #[cfg(feature = "mlua")]
    pub lua: Lua,
    pub events: Vec<WindowEvent>,
    pub fontmgr: FontMgr,
    pub fira_typface: Typeface,
    pub fira_font: Font,
    pub fira_font_huge: Font,
    pub surface: Surface,
    pub render_target: BackendRenderTarget,
    pub gl_direct_context: DirectContext,
    pub gl_interface: GlInterface,
    pub glow_context: glow::Context,
    pub events_receiver: GlfwReceiver<(f64, WindowEvent)>,
    pub window: PWindow,
    pub glfw_context: Glfw,
    pub scale: [f32; 2],
}
impl HelperContext {
    pub fn new(window_width_height: [u32; 2]) -> Self {
        let (glfw_context, mut window, events_receiver) = create_window(window_width_height);
        let (sx, sy) = window.get_content_scale();
        let glow_context =
            unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s)) };
        let (gl_interface, mut gl_direct_context) =
            create_gl_interface_and_direct_context(&mut window);
        let (render_target, mut surface) =
            create_render_target_and_surface(&mut window, &glow_context, &mut gl_direct_context);

        // for text rendering
        let mut fontmgr = FontMgr::create_custom_dir(".").unwrap();
        let mut fira_data = SkiaData::new_with_copy(FIRA_CODE_REGULAR_BYTES);
        let mut fira_tf = fontmgr.create_from_data(&mut fira_data, 0).unwrap();
        let fira_font = Font::new_with_values(&mut fira_tf, 16.0 * sx, 1.0, 0.0).unwrap();
        let huge_fira_font = Font::new_with_values(&mut fira_tf, 48.0 * sx, 1.0, 0.0).unwrap();
        #[cfg(feature = "mlua")]
        let lua = {
            let lua = Lua::new();
            let ckia_table = ckia::lua::add_bindings(&lua).unwrap();
            lua.globals().set("ckia", ckia_table);
            lua
        };

        Self {
            fontmgr,
            fira_typface: fira_tf,
            fira_font,
            fira_font_huge: huge_fira_font,
            surface,
            render_target,
            gl_direct_context,
            gl_interface,
            glow_context,
            events_receiver,
            window,
            glfw_context,
            scale: [sx, sy],
            events: vec![],
            #[cfg(feature = "mlua")]
            lua,
        }
    }
    pub fn enter_event_loop(mut self, mut painting_fn: impl FnMut(&mut Self)) {
        while !self.window.should_close() {
            self.glfw_context.poll_events();
            let mut events = vec![];
            for (_, ev) in glfw::flush_messages(&self.events_receiver) {
                match &ev {
                    glfw::WindowEvent::Close => {
                        self.window.set_should_close(true);
                    }
                    glfw::WindowEvent::ContentScale(sx, sy) => {
                        self.scale = [*sx, *sy];
                    }
                    glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                        let w = *w;
                        let h = *h;
                        // recreate the render target and surface
                        self.glow_context.viewport(0, 0, w, h);
                        let (render_target, surface) = create_render_target_and_surface(
                            &mut self.window,
                            &self.glow_context,
                            &mut self.gl_direct_context,
                        );
                        self.render_target = render_target;
                        self.surface = surface;
                    },
                    glfw::WindowEvent::Key(k, _, _, _) => match k {
                        glfw::Key::Escape => {
                            self.window.set_should_close(true);
                        }
                        _ => {}
                    },
                    _ => {}
                }
                events.push(ev);
            }
            self.events = events;
            let mut surface_canvas = self.surface.get_canvas();
            let save_count = surface_canvas.as_mut().save();
            surface_canvas.as_mut().scale(self.scale[0], self.scale[1]);
            painting_fn(&mut self);
            self.surface
                .get_canvas()
                .as_mut()
                .restore_to_count(save_count);
            self.gl_direct_context.flush_and_submit(false);
            self.window.swap_buffers();
        }
    }
}
