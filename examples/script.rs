use helper::HelperContext;

mod helper;

#[cfg(feature = "mlua")]
pub fn main() {
    use ckia::{canvas::Canvas, Color};
    use mlua::Function;
    let lua_code = std::fs::read_to_string("./examples/code.luau").unwrap();
    let mut load = false;
    helper::HelperContext::new([800, 600]).enter_event_loop(|htx| {
        let HelperContext { lua, surface, .. } = htx;
        if !load {
            load = true;
            lua.load(&lua_code).exec().unwrap();
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
        .unwrap();
    });
}

#[cfg(not(feature = "mlua"))]
pub fn main() {
    eprintln!("This example requires mlua feature to be enabled");
}
