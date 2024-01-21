mod bindings;
pub use bindings::*;

#[cfg(windows)]
#[cfg(not(feature = "disable_embedding_icudtl_dat"))]
pub fn init() {
    use std::env;
    static icudtl: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icudtl.dat"));

    {
        use std::sync::Mutex;

        lazy_static::lazy_static!(
            static ref MUTEX : Mutex<()> = Mutex::new(());
        );

        // Using `Once` does not work for yet unknown reasons.
        // https://github.com/rust-skia/rust-skia/issues/566

        let lock = MUTEX.lock().unwrap();
        assert!(
            unsafe { bindings::sk_icu_set_icudtl_dat(&icudtl[0] as &'static u8 as *const u8 as _) },
            "failed to init icudtl"
        );
        drop(lock);
    }

    // {
    //     use std::fs;

    //     let path = env::current_exe()
    //         .expect("Failed to resolve the current executable's path")
    //         .parent()
    //         .expect("Current executable's parent path does not point to a directory")
    //         .join("icudtl.dat");
    //     if path.exists() {
    //         return;
    //     };
    //     fs::write(path, &icudtl[..])
    //         .expect("Failed to write icudtl.dat into the current executable's directory");
    // }
}

#[cfg(not(windows))]
pub fn init() {}
