use helper::HelperContext;

mod helper;

#[cfg(feature = "mlua")]
pub fn main() {
    use std::time::{Duration, Instant};

    use ckia::{canvas::Canvas, Color};
    use mlua::{Function, Table};
    let start_time = Instant::now();
    // for first frame
    let mut reload_lua_code = true;
    let mut data = None;
    let mut previous_reset = 0.0;
    let mut frame_time = Duration::ZERO;
    let mut frame_times = vec![];
    helper::HelperContext::new([1920, 1050]).enter_event_loop(|htx| {
        let HelperContext {
            lua,
            surface,
            events,
            glfw_context,
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
        if data.is_none() {
            let t = lua.create_table().unwrap();
            t.set(
                "frame_time",
                format!("{:.2}", frame_time.as_secs_f64() * 1000.0),
            )
            .unwrap();
            data = Some(lua.create_registry_value(t).unwrap());
        }

        let data_table: Table = lua.registry_value(data.as_ref().unwrap()).unwrap();
        let mut surface_canvas = surface.get_canvas();
        let canvas = surface_canvas.as_mut();
        canvas.clear(Color::TRANSPARENT);
        let i = Instant::now();

        for x_offset in [0.0, 300.0, 600.0, 900.0, 1200.0, 1500.0] {
            for y_offset in [0.0, 300.0, 600.0] {
                let skia_stack = canvas.save();
                canvas.translate(x_offset, y_offset);
                {
                    lua.scope(|sc| {
                        let canvas = sc.create_userdata_ref_mut::<Canvas>(canvas).unwrap();
                        let tick: Function = lua.globals().get("tick").unwrap();
                        let _: () = tick.call((canvas, &data_table)).unwrap();
                        Ok(())
                    })
                    .expect("tick function crashed");
                }
                canvas.restore_to_count(skia_stack);
            }
        }
        frame_times.push(i.elapsed());
        frame_time = (frame_time + i.elapsed()) / 2;
        let current_time = glfw_context.get_time();
        if current_time - previous_reset > 1.0 {
            previous_reset = current_time;
            data_table
                .set(
                    "frame_time",
                    format!("{:.2}", frame_time.as_secs_f64() * 1000.0),
                )
                .unwrap();
            println!(
                "{} average ckia time (ms): {:.2}",
                frame_times.len(),
                frame_time.as_secs_f64() * 1000.0
            ); // frame_time = Duration::ZERO
        }
    });
    println!("program duration: {}", start_time.elapsed().as_secs_f32());
    println!("total frames: {}", frame_times.len());
    let average_time =
        (frame_times.iter().sum::<Duration>().as_secs_f32() * 1000.0) / frame_times.len() as f32;
    println!("average frame  time: {}", average_time);
}

#[cfg(not(feature = "mlua"))]
pub fn main() {
    eprintln!("This example requires mlua feature to be enabled");
}
