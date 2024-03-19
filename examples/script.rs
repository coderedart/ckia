use helper::HelperContext;

mod helper;

#[cfg(feature = "mlua")]
pub fn main() {
    use ckia::{canvas::Canvas, Color};
    use mlua::Function;
    // for first frame
    let mut reload_lua_code = true;
    helper::HelperContext::new([800, 600]).enter_event_loop(|htx| {
        let HelperContext {
            lua,
            surface,
            events,
            ..
        } = htx;
        for ev in events {
            match ev {
                glfw::WindowEvent::Key(k, _, _, _) if *k == glfw::Key::Enter => {
                    reload_lua_code = true
                }
                _ => {}
            }
        }
        if reload_lua_code {
            reload_lua_code = false;
            let lua_code = std::fs::read_to_string("./examples/code.luau")
                .expect("failed to read code.luau from ./examples directory");
            lua.load(&lua_code)
                .exec()
                .expect("failed to load luau code");
        }
        let mut surface_canvas = surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        canvas.clear(Color::TRANSPARENT);
        lua.scope(|sc| {
            let canvas = sc.create_userdata_ref_mut::<Canvas>(canvas).unwrap();
            let tick: Function = lua.globals().get("tick").unwrap();
            let _: () = tick.call(canvas).unwrap();
            Ok(())
        })
        .expect("tick function crashed");
    });
}

#[cfg(not(feature = "mlua"))]
pub fn main() {
    eprintln!("This example requires mlua feature to be enabled");
}
