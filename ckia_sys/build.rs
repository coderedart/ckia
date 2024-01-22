use std::{
    env::var,
    fs,
    path::{Path, PathBuf},
    process::*,
};
pub const SKIA_PYTHON: &str = "SKIA_PYTHON";
pub const SKIA_GN: &str = "SKIA_GN";
pub const SKIA_NINJA: &str = "SKIA_NINJA";
pub const SKIA_CC: &str = "SKIA_CC";
pub const SKIA_CC_WRAPPER: &str = "SKIA_CC_WRAPPER";
pub const SKIA_CXX: &str = "SKIA_CXX";
pub const SKIA_CLANG_WIN: &str = "SKIA_CLANG_WIN";
pub const SKIA_CLANG_WIN_VERSION: &str = "SKIA_CLANG_WIN_VERSION";
pub const SKIA_BUILD_FROM_SRC: &str = "SKIA_BUILD_FROM_SRC";
pub const SKIA_GN_ARGS: &str = "SKIA_GN_ARGS";
pub const SKIA_COPY_LIBS: &str = "SKIA_COPY_LIBS";
pub const SKIA_SRC_DIR: &str = "SKIA_SRC_DIR";
pub const SKIA_SRC_ARCHIVE_URL: &str = "SKIA_SRC_ARCHIVE_URL";
pub const SKIA_CACHE_DIR: &str = "SKIA_CACHE_DIR";
pub const ALL_ENV_VARS: &[&str] = &[
    SKIA_PYTHON,
    SKIA_GN,
    SKIA_NINJA,
    SKIA_CC,
    SKIA_CC_WRAPPER,
    SKIA_CXX,
    SKIA_CLANG_WIN,
    SKIA_CLANG_WIN_VERSION,
    SKIA_BUILD_FROM_SRC,
    SKIA_GN_ARGS,
    SKIA_COPY_LIBS,
    SKIA_SRC_DIR,
    SKIA_SRC_ARCHIVE_URL,
    SKIA_CACHE_DIR,
];
pub const PRE_BUILT_MAC_DEFAULT_FRAMEWORKS: &[&str] = &["ApplicationServices", "OpenGL", "AppKit"];
pub const PRE_BUILT_MAC_DEFAULT_LIBS: &[&str] = &["dl"];
pub const PRE_BUILT_LINUX_DEFAULT_LIBS: &[&str] = &["dl", "pthread", "fontconfig", "GL", "EGL"];
pub const PRE_BUILT_WINDOWS_DEFAULT_LIBS: &[&str] = &[
    "Ole32", "OleAut32", "FontSub", "User32", "Usp10", "Gdi32", "OpenGL32",
];
// feature names
pub const DISABLE_CACHE_FEATURE_NAME: &str = "disable_caching_downloads_in_user_cache";
pub const DISABLE_DOWNLOAD_PRE_BUILT_SKIA_LIBS: &str = "disable_download_pre_built_skia_libs";
pub const DISABLE_BUILD_FROM_SRC: &str = "disable_build_from_src";

fn main() {
    println!("starting ckia_sys build script");
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=early exit because DOCS_RS is set");
        return;
    }

    for e in ALL_ENV_VARS {
        println!("cargo:rerun-if-env-changed={e}");
    }

    let build_from_src: bool = !cfg!(feature = "disable_build_from_src");
    let download_pre_built_skia_libs: bool =
        !cfg!(feature = "disable_download_pre_built_skia_libs");
    let component_build = cfg!(feature = "component_build");

    let target_triple = var("TARGET").expect("failed to get build target triple");
    let out_dir = PathBuf::from(var("OUT_DIR").expect("failed to get out_dir"));
    let prefix = if component_build { "shared" } else { "static" };

    check_command_version("curl", out_dir.to_str().unwrap()).expect("failed to find curl");
    check_command_version("tar", out_dir.to_str().unwrap()).expect("failed to find tar");

    let major: u32 = var("CARGO_PKG_VERSION_MAJOR").unwrap().parse().unwrap();
    let minor: u32 = var("CARGO_PKG_VERSION_MINOR").unwrap().parse().unwrap();

    let skia_cache_dir = get_cache_dir(&out_dir, major, minor);
    assert!(skia_cache_dir.exists(), "{skia_cache_dir:?} doesn't exist");
    if cfg!(not(windows)) {
        println!("cargo:rustc-link-lib=stdc++");
    }
    if cfg!(feature = "dynamic_linking") {
        println!("cargo:warning= early exit because of dynamic linking");

        let libs_dir = skia_cache_dir.join(format!("{prefix}_{target_triple}"));
        if libs_dir.exists() {
            link_libs_present_in_the_directory(&libs_dir, &target_triple, prefix);
        }
        return;
    }
    let mut pre_built_skia_libs_success = false;
    if download_pre_built_skia_libs {
        pre_built_skia_libs_success = try_build_with_skia_pre_built_libs(
            &skia_cache_dir,
            &target_triple,
            prefix,
            major,
            minor,
            &out_dir,
        )
        .map_err(|e| {
            println!("cargo:warning=failed to download prebuilt libs of skia due to error: {e}");
            e
        })
        .is_ok();
    }
    if build_from_src && !pre_built_skia_libs_success {
        try_build_from_src(
            &skia_cache_dir,
            &out_dir,
            &target_triple,
            prefix,
            major,
            minor,
        )
        .expect("failed to build from src");
    }
}
pub fn try_build_with_skia_pre_built_libs(
    cache_dir: &Path,
    target_triple: &str,
    prefix: &str,
    major: u32,
    minor: u32,
    out_dir: &Path,
) -> Result<(), String> {
    let cache_dir = cache_dir.join(format!("{prefix}_{target_triple}"));
    std::fs::create_dir_all(&cache_dir).expect("failed to create libs cache dir");
    println!("using {cache_dir:?} as the libs cache directory for downloads.");
    let archive_name = format!("{prefix}_{target_triple}.tar.gz");
    let archive_path = cache_dir.join(&archive_name);
    let extract_status_success_value = "success";
    let extract_status_path = cache_dir.join("extract_status.txt");
    if fs::read_to_string(&extract_status_path).unwrap_or_default() != extract_status_success_value
    {
        println!(
            "extract status is not success, so starting to check if downloaded archive exists"
        );
        {
            let download_status_success_value = "success";
            let download_status_path = cache_dir.join("download_status.txt");
            if std::fs::read_to_string(&download_status_path).unwrap_or_default()
                != download_status_success_value
            {
                println!("download status doesn't exist. downloading files from github");
                let url = format!(
                    "https://github.com/coderedart/skia/releases/download/ckia_{major}.{minor}/{archive_name}"
                );
                println!("downloading binaries from {url}");
                download(&url, archive_path.to_str().unwrap())?;
                println!("downloaded skia libs archive to {archive_path:?}. writing {download_status_success_value} to {download_status_path:?}");
                fs::write(download_status_path, download_status_success_value)
                    .expect("failed to write success status");
            }
        }
        println!("using tar to extract the downloaded archive at {archive_path:?}");
        if Command::new("tar")
            .current_dir(&cache_dir)
            .arg("-xzvf")
            .arg(&archive_path)
            .status()
            .map_err(|e| format!("failed to run tar command to extract skia libs. {e}"))?
            .success()
        {
            println!("extraction is successful, writing status to {extract_status_path:?} so that future builds can skip this step");
            fs::write(&extract_status_path, extract_status_success_value)
                .expect("failed to write extract status success value");
        } else {
            return Err(format!("tar command failed to extract skia binaries from {archive_path:?} to {cache_dir:?}"));
        }
    } else {
        println!("extract status is already success. skipping download/extraction");
    }
    {
        let native_libs = if target_triple.contains("windows") {
            PRE_BUILT_WINDOWS_DEFAULT_LIBS
        } else if target_triple.contains("darwin") {
            for fw in ["ApplicationServices", "OpenGL", "AppKit"] {
                println!("cargo:rustc-link-lib=framework={fw}")
            }
            PRE_BUILT_MAC_DEFAULT_LIBS
        } else if target_triple.contains("linux") {
            PRE_BUILT_LINUX_DEFAULT_LIBS
        } else {
            println!("cargo:warning=unknown platform. not printing any default libs");
            &[]
        };
        for native_lib in native_libs {
            println!("cargo:rustc-link-lib={native_lib}")
        }
    }
    if target_triple.contains("windows") {
        println!("checking if icudtl.dat already exists in {out_dir:?}");
        let icudtl_src_path = cache_dir.join("icudtl.dat");
        let icudtl_dst_path = out_dir.join("icudtl.dat");
        if icudtl_dst_path.try_exists().unwrap_or_default() {
            println!("icudtl.dat already exists. nothing to do");
        } else {
            assert!(
                icudtl_src_path.try_exists().unwrap_or_default(),
                "icudtl.dat is not available at {icudtl_src_path:?}. did extraction fail?"
            );
            println!("icudtl.dat doesn't exist in out_dir, so copying it from {icudtl_src_path:?}");
            match fs::copy(&icudtl_src_path, &icudtl_dst_path) {
                Ok(_) => {
                    println!("successfully copied icudtl.dat to {out_dir:?}");
                }
                Err(e) => {
                    println!("cargo:warning=faield to copy icudtl.dat from {cache_dir:?}: {e:?}");
                }
            }
        }
    }
    link_libs_present_in_the_directory(&cache_dir, target_triple, prefix);
    Ok(())
}
pub fn try_build_from_src(
    cache_dir: &Path,
    out_dir: &Path,
    target_triple: &str,
    prefix: &str,
    major: u32,
    minor: u32,
) -> Result<(), String> {
    let skia_dir = var("SKIA_SRC_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        println!("failed to get SKIA_SRC_DIR. trying to download skia src tarball");
        let src_archive_url: String = var(SKIA_SRC_ARCHIVE_URL).unwrap_or_else(|_| {
            println!("failed to get {SKIA_SRC_ARCHIVE_URL}. using default skia url");
            format!("https://github.com/coderedart/skia/releases/download/ckia_{major}.{minor}/src.tar.gz")
        });
        println!("using {src_archive_url} as src archive url");
        let (skia_src_dir, archive_path)  = {
            let skia_src_dir_name = "skia_src";
            let skia_src_dir = cache_dir.join(skia_src_dir_name);
            std::fs::create_dir_all(&skia_src_dir).expect("failed to create skia src directory");
            let skia_src_tar_name = format!("{skia_src_dir_name}.tar.gz");
            let skia_src_tar_path = cache_dir.join(skia_src_tar_name);
            (skia_src_dir, skia_src_tar_path)
        };

    let extract_status_success_value = "success";
    let extract_status_path = cache_dir.join("src_extract_status.txt");
    if fs::read_to_string(&extract_status_path).unwrap_or_default() != extract_status_success_value
    {
        println!(
            "extract status is not success, so starting to check if downloaded archive exists"
        );
        {
            let download_status_success_value = "success";
            let download_status_path = cache_dir.join("src_download_status.txt");
            if std::fs::read_to_string(&download_status_path).unwrap_or_default()
                != download_status_success_value
            {
                println!("src download status doesn't exist. downloading files from {src_archive_url}");
                download(&src_archive_url, archive_path.to_str().unwrap()).expect("failed to download skia src archive");
                println!("downloaded skia src archive to {archive_path:?}. writing {download_status_success_value} to {download_status_path:?}");
                fs::write(download_status_path, download_status_success_value)
                    .expect("failed to write success status");
            }
        }
        println!("using tar to extract the downloaded archive at {archive_path:?}");
        if Command::new("tar")
            .current_dir(&skia_src_dir)
            .arg("-xzvf")
            .arg(&archive_path)
            .status()
            .expect("failed to run tar command to extract skia src")
            .success()
        {
            println!("extraction is successful, writing status to {extract_status_path:?} so that future builds can skip this step");
            fs::write(&extract_status_path, extract_status_success_value)
                .expect("failed to write extract status success value");
        } else {
            panic!("tar command failed to extract skia binaries from {archive_path:?} to {skia_src_dir:?}");
        }
    } else {
        println!("extract status is already success. skipping download/extraction");
    }
        skia_src_dir
    });
    assert!(
        skia_dir
            .try_exists()
            .expect("failed to check if skia src dir exists"),
        "skia_dir {skia_dir:?} doesn't exist :("
    );
    let skia_dir_str = skia_dir.to_str().unwrap();
    let ninja = {
        println!("running git-sync-deps");

        let python: String = var(SKIA_PYTHON).unwrap_or("python".to_string());
        println!("using {python} as python");
        check_command_version(&python, skia_dir_str).expect("failed ot find python");
        assert!(
            Command::new(&python)
                .current_dir(&skia_dir)
                .args(&["tools/git-sync-deps"])
                .env("GIT_SYNC_DEPS_SKIP_EMSDK", "True")
                .env("GIT_SYNC_DEPS_SHALLOW_CLONE", "True")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("failed to run git-sync-deps command")
                .success(),
            "Cannot download skia depenedencies"
        );
        var(SKIA_NINJA).unwrap_or_else(|_| {
            assert!(
                Command::new(python)
                    .current_dir(&skia_dir)
                    .arg("./bin/fetch-ninja")
                    .status()
                    .expect("failed to run python to fetch ninja")
                    .success(),
                "cannot fetch ninja"
            );
            skia_dir
                .join("third_party")
                .join("ninja")
                .join("ninja")
                .to_str()
                .unwrap()
                .to_string()
        })
    };

    let gn = var(SKIA_GN).unwrap_or_else(|_| {
        skia_dir
            .join("bin")
            .join("gn")
            .to_str()
            .unwrap()
            .to_string()
    });
    check_command_version(&ninja, skia_dir_str).expect("failed to find ninja");
    check_command_version(&gn, skia_dir_str).expect("failed to find gn");

    let cc = var("SKIA_CC").unwrap_or_else(|e| {
        println!("failed to get SKIA_CC: {e}");
        if check_command_version("clang", ".").is_ok() {
            println!("found clang. using it as cc");
            return "clang".to_string();
        }
        println!("couldn't find clang. using default cc");
        String::new()
    });

    let cc_wrapper: String = var("SKIA_CC_WRAPPER").unwrap_or_else(|e| {
        println!("failed to get SKIA_CC_WRAPPER: {e}.");
        for wrapper in ["sccache", "ccache"] {
            if check_command_version(wrapper, ".").is_ok() {
                println!("found {wrapper}. using it as cc_wrapper");
                return wrapper.to_string();
            }
        }
        println!("failed to find ccache or sccache. continuing without cc_wrapper");
        String::new()
    });
    let cxx: String = var("SKIA_CXX").unwrap_or_else(|e| {
        println!("failed to get SKIA_CXX: {e}");
        if check_command_version("clang++", ".").is_ok() {
            println!("found clang++. using it as cxx");
            return "clang++".to_string();
        }
        println!("couldn't find clang++. using default cxx");
        String::new()
    });
    #[cfg(windows)]
    let clang_win: String = var("SKIA_CLANG_WIN").unwrap_or_else(|e| {
        {
            println!("couldn't get SKIA_CLANG_WIN: {e}");
            if std::path::Path::new("C:\\Program Files\\LLVM").try_exists().unwrap_or_default() {
                println!("found C:\\Program Files\\LLVM. using it as clang_win");
                return "C:\\Program Files\\LLVM".to_string();
            }
            println!("couldn't find C:\\Program Files\\LLVM. using empty clang_win. compilation could fail");
        }
        String::new()
    });
    #[cfg(windows)]
    let clang_win_version: String = var("SKIA_CLANG_WIN_VERSION").unwrap_or_else(|e| {
        println!("couldn't get SKIA_CLANG_WIN_VERSION: {e}");
        if !clang_win.is_empty() {
            println!("reading {clang_win}/lib/clang for clang versions");
            let mut max_version = [0, 0, 0];
            let mut max_version_dir_name = "".to_string();
            if let Ok(dir_iter) = std::fs::read_dir(format!("{clang_win}\\lib\\clang")) {
                for f in dir_iter {
                    let name = f.unwrap().file_name().to_str().unwrap().to_string();
                    println!("found {name}");
                    let mut version = [0u32; 3];
                    let mut parts = name.split('.');
                    version
                        .fill_with(|| parts.next().unwrap_or_default().parse().unwrap_or_default());
                    println!("parsed version: {version:?}");
                    if version > max_version {
                        max_version = version;
                        max_version_dir_name = name;
                    }
                }
                if max_version[0] != 0 {
                    println!("setting clang_win_version to {max_version_dir_name}");
                    return max_version_dir_name;
                } else {
                    eprintln!("failed ot find a clang version in {clang_win}. build might fail");
                }
            } else {
                println!(
                    "cargo:warning=failed to read {clang_win}/lib/clang to get clang version. build might fail"
                );
            }
        }
        String::new()
    });
    // unless explicitly specified as debug build via the env var, we will use an official build for performance.
    let is_official_build = var("DEBUG").unwrap_or_default();
    let is_official_build =
        is_official_build != "2" && is_official_build != "true" && is_official_build != "full"; // if it is 2 or true or full, then we do debug builds

    let mut args = String::new();

    args.push_str("is_official_build=");
    args.push_str(if is_official_build {
        "true\n"
    } else {
        "false\n"
    });
    if !cc.is_empty() {
        args.push_str(&format!("cc=\"{}\"\n", cc));
    }
    if !cc_wrapper.is_empty() {
        args.push_str(&format!("cc_wrapper=\"{}\"\n", cc_wrapper));
    }
    if !cxx.is_empty() {
        args.push_str(&format!("cxx=\"{}\"\n", cxx));
    }
    #[cfg(windows)]
    {
        if !clang_win.is_empty() {
            args.push_str(&format!("clang_win=\"{}\"\n", clang_win));
        }
        if !clang_win_version.is_empty() {
            args.push_str(&format!("clang_win_version=\"{}\"\n", clang_win_version));
        }
    }
    println!("not using system libs");
    // to avoid the whole dependency mess.
    for arg in [
        "skia_use_system_expat=false\n",
        "skia_use_system_freetype2=false\n",
        "skia_use_system_harfbuzz=false\n",
        "skia_use_system_icu=false\n",
        "skia_use_system_libjpeg_turbo=false\n",
        "skia_use_system_libpng=false\n",
        "skia_use_system_libwebp=false\n",
        "skia_use_system_zlib=false\n",
    ] {
        args.push_str(arg);
    }
    args.push_str("skia_enable_tools=false\n");
    if let Ok(extra_args) = var("SKIA_GN_ARGS") {
        println!("found SKIA_GN_ARGS var: {extra_args}");
        for arg in extra_args.split(';') {
            println!("adding arg: {arg}");
            args.push_str(arg);
            args.push('\n');
        }
    }
    println!("gn args:\n{args}");
    std::fs::write(out_dir.join("args.gn"), &args).expect("failed to write gn args");

    assert!(
        Command::new(&gn)
            .current_dir(&skia_dir)
            .args(["gen", out_dir.to_str().unwrap(),])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed to run gn command")
            .success(),
        "Cannot generate build files with gn gen"
    );
    assert!(
        Command::new(&gn)
            .current_dir(&skia_dir)
            .args(&["args", "--list", "--short", out_dir.to_str().unwrap(),])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed to run gn args -list")
            .success(),
        "failed to get list of gn args"
    );
    let output = Command::new(&gn)
        .current_dir(&skia_dir)
        .args(["desc", &out_dir.to_str().unwrap(), "//:skia", "libs"])
        .output()
        .expect("failed to run gn desc libs ");
    assert!(
        output.status.success(),
        "failed to get libs with gn desc {}",
        String::from_utf8(output.stderr).unwrap()
    );
    let output = String::from_utf8(output.stdout).unwrap();
    // with extension names included
    let skia_needs_libs: Vec<String> = output.lines().map(|s| s.to_string()).collect();
    if target_triple.contains("darwin") {
        let output = Command::new(&gn)
            .current_dir(&skia_dir)
            .args(["desc", &out_dir.to_str().unwrap(), "//:skia", "frameworks"])
            .output()
            .expect("failed to run gn desc libs ");
        assert!(
            output.status.success(),
            "failed to get libs with gn desc {}",
            String::from_utf8(output.stderr).unwrap()
        );
        let output = String::from_utf8(output.stdout).unwrap();
        for framework in output.lines() {
            let framework = framework.trim_end_matches(".framework");
            println!("cargo:rustc-link-lib=framework={framework}");
        }
    }
    println!("running ninja");
    assert!(
        Command::new("ninja")
            .current_dir(&skia_dir)
            .args(["-C", &out_dir.to_str().unwrap()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("failed to run ninja build command")
            .success(),
        "Cannot build skia with ninja"
    );
    link_libs_present_in_the_directory(out_dir, target_triple, prefix);
    println!("remaining_libs: {skia_needs_libs:#?}");
    for lib in skia_needs_libs {
        // for some reason libs are printed as "OpenGL32.lib" on windows with gn. But linux/mac are printed without any prefix/suffix.
        let lib = lib.trim_end_matches(".lib");
        println!("cargo:rustc-link-lib={lib}");
    }

    if let Ok(p) = var(SKIA_COPY_LIBS) {
        println!("copying libs to {p}");
        let mut files_to_archive = vec![];
        for f in std::fs::read_dir(&out_dir).unwrap() {
            let f = f.unwrap().file_name().to_str().unwrap().to_string();
            if f.ends_with(".lib")
                || f.ends_with(".dat")
                || f.ends_with(".a")
                || f.ends_with(".dll")
                || f.ends_with(".so")
                || f.ends_with(".dylib")
            {
                files_to_archive.push(f);
            }
        }
        let zip_name = format!("{prefix}_{target_triple}.tar.gz");

        assert!(
            Command::new("tar")
                .current_dir(out_dir)
                .arg("-czvf")
                .arg(&zip_name)
                .args(files_to_archive)
                .status()
                .expect("failed to run tar command")
                .success(),
            "failed to archive libs"
        );
        std::fs::copy(out_dir.join(&zip_name), format!("{p}/{zip_name}"))
            .expect("failed to copy archive");
    }
    Ok(())
}
fn download(url: &str, output_file_path: &str) -> Result<(), String> {
    println!("using curl to download {url} and save to {output_file_path}");
    if std::process::Command::new("curl")
        // follow redirects
        .arg("-L")
        // fail fast with no "error pages" output. more of a hint though, so we might still get error on stdout.
        // so make sure to check the actual status returned.
        .arg("-f")
        .arg(url)
        .arg("-o")
        .arg(output_file_path)
        .status()
        .map_err(|e| format!("failed to run curl command to download {url} due to {e:?}"))?
        .success()
    {
        Ok(())
    } else {
        Err(format!(
            "curl command failed to download {url} to {output_file_path}"
        ))
    }
}
pub fn link_libs_present_in_the_directory(dir: &Path, target_triple: &str, _prefix: &str) {
    println!("starting to search for libs in {dir:?}");
    println!("cargo:rustc-link-search={}", dir.display());
    // if prefix == "shared" {
    //     println!("cargo:rustc-link-lib=skia")
    // } else {
    //     println!("cargo:rustc-link-lib=static=skia")
    // }
    if !target_triple.contains("linux") {
        println!("cargo:warning=unknown platform. just printing any static libs for linking like it is linux");
    }
    for f in std::fs::read_dir(dir).expect("failed to read dir for linking libs") {
        let f = f.expect("failed to get dir entry when reading dir for libs");
        let libname = f
            .file_name()
            .to_str()
            .expect("non utf-8 libname")
            .to_string();

        println!("found file {libname}");
        if target_triple.contains("windows") {
            if libname.ends_with(".lib") {
                let libname = libname.strip_suffix(".lib").unwrap();
                println!("cargo:rustc-link-lib=static={libname}");
            }
            if libname.ends_with(".dll") {
                let libname = libname.strip_suffix(".dll").unwrap();
                println!("cargo:rustc-link-lib=dylib={libname}");
            }
        } else if target_triple.contains("darwin") {
            if libname.ends_with(".a") {
                let libname = libname
                    .strip_suffix(".a")
                    .unwrap()
                    .strip_prefix("lib")
                    .unwrap();
                println!("cargo:rustc-link-lib=static={libname}");
            }
            if libname.ends_with(".dylib") {
                let libname = libname
                    .strip_suffix(".dylib")
                    .unwrap()
                    .strip_prefix("lib")
                    .unwrap();
                println!("cargo:rustc-link-lib=dylib={libname}");
            }
        } else {
            if libname.ends_with(".a") {
                let libname = libname
                    .strip_suffix(".a")
                    .unwrap()
                    .strip_prefix("lib")
                    .unwrap();
                println!("cargo:rustc-link-lib=static={libname}");
            }
            if libname.ends_with(".so") {
                let libname = libname
                    .strip_suffix(".so")
                    .unwrap()
                    .strip_prefix("lib")
                    .unwrap();
                println!("cargo:rustc-link-lib=dylib={libname}");
            }
        }
    }
}
pub fn check_command_version(cmd: &str, cd: &str) -> Result<(), String> {
    if Command::new(cmd)
        .current_dir(cd)
        .arg("--version")
        .status()
        .map_err(|e| format!("failed to run {cmd} --version: {e}"))?
        .success()
    {
        Ok(())
    } else {
        Err(format!("{cmd} --version failed for some reason"))
    }
}
fn get_cache_dir(out_dir: &Path, major: u32, minor: u32) -> PathBuf {
    let fake_cache_dir = out_dir.join("cache");
    std::fs::create_dir_all(&fake_cache_dir)
        .expect("failed to create fake cache directory inside out_dir");
    if cfg!(feature = "disable_caching_downloads_in_user_cache") {
        println!("caching is disabled. using out_dir as cache instead");
        return fake_cache_dir;
    }
    if let Ok(cache_dir) = var(SKIA_CACHE_DIR).map(PathBuf::from) {
        create_sub_cache_version_dir(&cache_dir, major, minor).unwrap_or_else(|e| {
            panic!("failed to use directory set in {SKIA_CACHE_DIR}: {cache_dir:?}: {e}")
        })
    } else {
        match get_user_cache_dir() {
            Ok(user_cache_dir) => {
                println!("using user cache dir: {user_cache_dir:?}");
                match create_sub_cache_version_dir(&user_cache_dir, major, minor) {
                    Ok(cache_dir) => {
                        return cache_dir;
                    }
                    Err(e) => {
                        println!(
                            "cargo:warning=failed to create sub cache dir in {user_cache_dir:?}: {e}"
                        );
                    }
                }
            }
            Err(e) => println!("cargo:warning=failed to get user cache dir {e}"),
        }
        println!("cargo:warning=failed to find a suitable cache dir, so using out_dir {fake_cache_dir:?} as fallback.");
        fake_cache_dir
    }
}
pub fn create_sub_cache_version_dir(
    cache_dir: &Path,
    major: u32,
    minor: u32,
) -> Result<PathBuf, String> {
    let sub_cache_dir = Path::new("ckia_sys").join(format!("{major}.{minor}"));
    match cache_dir.try_exists() {
        Ok(exists) => {
            if exists {
                println!("cache_dir: {cache_dir:?} exists. checking for existence of previous subcache dir");
                let cache_dir = cache_dir.join(sub_cache_dir);
                match cache_dir.try_exists() {
                    Ok(exists) => {
                        if exists {
                            println!("sub cache directory exists: {cache_dir:?}. reusing it");
                            Ok(cache_dir)
                        } else {
                            println!("sub cache directory doesn't exist: {cache_dir:?}. trying to create it");
                            match fs::create_dir_all(&cache_dir) {
                                Ok(_) => Ok(cache_dir),
                                Err(e) => {
                                    Err(format!(
                                        "failed to create subcache dir {cache_dir:?}. {e}"
                                    ))
                                }
                            }
                        }
                    }
                    Err(e) => {
                        Err(format!("failed to check if previously created sub cache dir {cache_dir:?} existed. {e}"))
                    }
                }
            } else {
                Err(format!("{cache_dir:?} doesn't exist"))
            }
        }
        Err(e) => Err(format!(
            "failed to check if cache_dir {cache_dir:?} exists :(: {e}"
        )),
    }
}
pub fn get_user_cache_dir() -> Result<PathBuf, String> {
    #[cfg(windows)]
    return var("LOCALAPPDATA")
        .map(PathBuf::from)
        .map_err(|_| format!("LOCALAPPDATA env var is not found."));
    #[cfg(mac)]
    return var("HOME")
        .map(|s| PathBuf::from(s).join("Library/Caches"))
        .map_err(|_| format!("failed to find $HOME var"));
    var("XDG_CACHE_HOME").map(PathBuf::from).or_else(|_| {
        println!(
            "failed to find XDG_CACHE_HOME, so using $HOME/.cache for caching downloaded archives"
        );
        var("HOME")
            .map(|s| PathBuf::from(s).join(".cache"))
            .map_err(|_| "$HOME is not set".to_string())
    })
}
/*

ndk
    Current value (from the default) = ""
      From //gn/BUILDCONFIG.gn:13

ndk_api
    Current value (from the default) = 21
      From //gn/BUILDCONFIG.gn:16

    Android 5.0, Lollipop

target_ar
    Current value (from the default) = "ar"
      From //gn/toolchain/BUILD.gn:33

target_cc
    Current value (from the default) = "clang"
      From //gn/toolchain/BUILD.gn:34

target_cpu
    Current value (from the default) = ""
      (Internally set; try `gn help target_cpu`.)

target_cxx
    Current value (from the default) = "clang++"
      From //gn/toolchain/BUILD.gn:35

target_link
    Current value (from the default) = "clang++"
      From //gn/toolchain/BUILD.gn:55

*/
