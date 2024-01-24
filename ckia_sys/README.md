### ckia_sys

This project will take skia and generate rust bindings for it. It also takes care of building skia.

fast compile times is the highest priority. and we do this by:
1. providing prebuilt static and dynamic libraries, which can be downloaded by build script for common configurations
2. maintain manual C bindings and use pre-generated bindings, instead of generating bindings at build time. avoids bindgen dependency and cuts atleast `6` seconds of build time. 

### Generate Bindings
1. install bindgen with `cargo install bindgen`
2. run 
```sh
# merge extern blocks creates a smaller and denser bindings.rs
# no-layout-tests avoids a lot of tests and makes bindings.rs cleaner. But for sanity, we might generate layout tests first, run cargo test, and then generate bindings without tests to publish. 
# -Iskia is necessary to let clang know where to search for include paths.
bindgen --merge-extern-blocks --default-enum-style rust --no-layout-tests -o bindings.rs skia/ckia/src/sk_all.c -- -Iskia/
```
3. Finally, fix the functions which need a different ABI on windows.
    1. `gr_vk_get_proc`. change its ABI to `extern "system"`, so that it can use `__stdcall` ABI on windows platforms.

### Build Dependencies
1. `curl` -> to download the pre-built libs
2. `tar` -> to extract the archives after downloading.

Both windows 10 / linux / macos have these by default. and are enough if you are using pre-built libs.

If you are building from source:
1. `git` -> to clone skia repo and deps 
2. `python` -> to download build deps and run build scripts
3. `tar` (optional) -> to package libs before copying them to `SKIA_COPY_LIBS` (see below)
4. `curl` ->  download source tarballs from github.
5. `clang` and `clang++`. (LLVM). 
6. `sccache` or `ccache` (optional) -> for caching compiled objects, as `cargo clean` will remove everything from target eventually. highly recommended.
7. Lots of patience. skia is going through a phase of changing build systems from `gn` to `bazel`. So, you *will* probably hit some issues.
8. Even more patience. While skia is a joy to use when drawing, it is absolutely horrible when compiling. 

### How is Skia built?
skia requries some complex build steps. Here's a rough outline which you will need to understand, if you want to make custom builds.

1. There are two different repositories involved. 
    1. https://github.com/coderedart/skia which contains skia source + `SkiaSharp`'s stable FFI bindings + pre-build static and dynamic skia libraries, which can be used by above `ckia_sys` to skip building from source.
    2. https://github.com/coderedart/ckia_sys which provides
        1. rust ffi bindings using bindgen 
        2. `ckia_sys`'s `build.rs` for building skia
2. Skia build consists of the following steps
    1. download/clone the skia source (from step 1.1). 
    2. change current working directory into the skia project root.
    3. ensure that required build dependencies are installed from the Dependencies section above. on `apt`(debian) based systems, you can run the script `./tools/install_dependencies.sh --yes`
    4. run `python ./tools/git-sync-deps`, so that skia can download sources of all its third_party dependencies into `./third_party/externals`. It will also download `gn` (generate ninja) tool into `./bin` based on the current platform.
    5. now, you set your build configuration and build directory with the command `./bin/gn gen path_to_build_dir --args='option1=true option2="clang"` where `option*=value` are the build configs. Some common configs are
        1. `is_official_build=bool`: whether this is a release build or not. this enables debug info (useful for debugging), spirv validation, extra debug asserts etc..
        2. `is_component_build=bool`: whether we want dynamic/shared libraries or static libraries. we usually want to link statically, so this is often false. 
        3. `skia_use_system_X=bool`: `X` can be any of `expat`,`freetype2`,`harfbuzz`,`icu`,`libjpeg_turbo`,`libpng`,`libwebp`,`zlib`. whether we want `X` to be linked with the system library. If it is false, then we build from source and statically link it. recommended to just build from src, because there could be version mismatches etc..
        4. `skia_use_no_X_encode=bool`: `X` can be `jpeg`, `png` or `webp`. If  you enable this, encoding into those formats will be disabled. If you have your own image encoding libs, then you would disable these. 
        5. `skia_use_X=bool` or `skia_enable_X`: `X` can be many things. 
            1. `gl`, `vulkan`, `metal`, `direct3d`: which apis to enable.  
            2. `ganesh`, `graphite`, `dawn`: which backend to use. `ganesh` is the mature backend. `graphite` is the next successor that will eventually replace `ganesh`. idk about dawn.
            3. `harfbuzz`, `freetype`, `icu` etc.. all the depedencies and features. 
        6. `target_X`: where X can be cpu or arch or platform etc.. which are useful for cross-compilation. you will probably use some additional options like `ndk_dir="/path/to/android/ndk"` etc.. that maybe useful.
        7. `clang_win="X"` and `clang_win_version="X"`: The first is path to LLVM installation (eg: `C:\\Program Files\\LLVM`) and the second is the llvm version installed (eg: `16`).
        8. `cc="X"` is C compiler (`clang` or `cc`). `cxx="X"` is C++ compiler. `cc_wrapper="X"` is a wrapper like `ccache` or `sccache`.
    5. Finally, you build the binaries with `ninja -C path_to_build_dir thing_to_build`. `thing_to_build` is optional, as it will just build the default "thing" which happens to be `skia`. built objects will be in the build directory.
3. `ckia_sys`'s `build.rs` doesn't want to be *too* complex in trying to support *all* these options. Especially when the maintainer (your truly) doesn't even have hardware like mac/ios to verify that the builds are working properly. So, we simplify the build system a lot for the common case, and expect people to use custom "from source" builds for their niche usecases. We will ofcourse do our best to gather documentation (like this current README) to help people get their custom builds working.
    1. By default, this repo provides prebuilt static and dynamic skia libs, with as many options enabled as possible. So, when you use it on `windows`, `linux` or `mac`,
        1. the build script checks if there's already an archive downloaded in `SKIA_CACHE_DIR` (if env var is set) or user's cache (eg: `$HOME/.cache/ckia_sys`).
        2. If it doesn't exist, we will just download the archive from github, place it in the cache directory.
        3. Now that we have the libs archive, we extract it into our `OUT_DIR` (provided by cargo) under `target`.
        4. We will just print the relevant `rust-link-lib` directives to help cargo decide which libs to link and where to find them.
        5. The advantage is that, we only have to download the libs once and keep reusing the cached archive between different cargo project builds or after `cargo clean` in the same project. 
        6. After the first libs archive download, the next build should take less than a second. 
    2. If you enable `disable_download_pre_built_skia_libs` feature, then we will skip previous step. And try to build from source. 
        1. If `SKIA_SRC_DIR` is set, we will use that as the source directory. If not, We download skia source tarball into cache directory, extract the sources in cache directory. If `SKIA_SRC_ARCHIVE_URL` is set, we will use that to download source archive, otherwise we default to `coderedart/skia` repo's releases. 
        2. pull third_party dependencies and `gn` using python (step 2.4). If you want us to use a specific python executable, then set `SKIA_PYTHON=/path/to/python`. Otherwise, we will just use `python` and let the shell look up the default python installation.   
        3. Just like caching downloaded pre-built libs, this will ensure that all of your cargo projects will reuse the same skia source and its third_party lib sources which will easily reach **hundreds of MB** worth bandwidth!!!
        4. Now that we have the required sources, we will generate build config with build_directory set to `OUT_DIR`.
        5. We will do our best to use `clang`/`clang++` as compilers or `ccache`/`sccache` as cc_wrappers if we can find them. To make it easier, you can set `SKIA_*` vars like `SKIA_CC`,`SKIA_CC_WRAPPER`,`SKIA_CXX`,`SKIA_CLANG_WIN`,`SKIA_CLANG_WIN_VERSION`. 
        6. Instead of using skia fetched `./bin/gn`, if you want us to use a specific executable, just set `SKIA_GN=/path/to/gn`.
        7. If you want to add any special build configurations, then just set `SKIA_GN_ARGS='key1=value1;key2="value2";...'` where key/value pairs are separated by `;`. These will be added towards the end, so they will override any of the settings that build.rs sets by default.
        8. We will also print the final args using `gn args build_dir --list --short`, for easy debugging of the build script on errors. 
        9. We call `ninja -C build_dir` to build it. If you want to provide a path for `ninja`, then just set `SKIA_NINJA=/path/to/ninja`. 
        10. we will ask gn what libs skia needs, and we will check which libs are built in `OUT_DIR`. For all libs we built from src, build script will print cargo directives like `rustc-link-lib=static=libname` for static linking. And for libs that skia needs, but didn't build (eg: `GL`, `X11`, `stdc++`, etc..), we will print cargo directives for dynamic linking from system. 
    3. If you enable `disable_build_from_src` feature, we will skip the previous step. If both previous steps are skipped
        1. IF `dynamic_linking` feature is enabled, then we will just print cargo directives for dynamic linking of skia library
        2. If `dynamic_linking` is NOT enabled, then we don't do anything :)
    4. You can use the `disable_caching_downloads_in_user_cache` feature to skip caching the download artifacts (eg: in CI). 

### Build 
Env Vars that affect build are organized in a nice table here.
| name              | default               | purpose                                                                               |
| ------            | -----                 | -------                                                                               |
| SKIA_PYTHON       | python                | path to python, which we will use when building skia from src                         |
| SKIA_GN           | ./bin/gn              | path to `gn` binary. skia will download it for us, but you can specify a custom path  |
| SKIA_CC           | clang*                | c compiler. clang is preferred by skia                                                |
| SKIA_CC_WRAPPER   | sccache* or ccache*   | improves compile times by using a global cache. highly recommended                    |
| SKIA_CXX          | clang++*              | c++ compiler. clang++ is preferred by skia                                            |
| SKIA_CLANG_WIN    | C:\\Program Files\\LLVM* | path to LLVM installation. required for using clang on windows                    |
| SKIA_CLANG_WIN_VERSION | SKIA_CLANG_WIN/lib/clang/version** | version of clang to use. ckia_sys will try to use the most recent version in SKIA_CLANG_WIN/lib/clang/. required for using clang on windows |
| SKIA_GN_ARGS      | ""                    | addditional GN args to pass to skia build for custom builds from src.  |
| SKIA_COPY_LIBS    | ""                    | tells ckia_sys build system to copy the built binaries to the directory based on the path set in this var. useful in workflows to distribute libs after building from src |
| SKIA_SRC_DIR      | ""                    | If set, we will use this as the source for the build instead of downloading source tarball from github |
| SKIA_SRC_ARCHIVE_URL | ""                 | If set, we will use curl to download this source tarball (.tar.gz), and extract it in cache dir, and use that as the skia src directory. Otherwise, we will just download from github |
| SKIA_CACHE_DIR    | `$LOCALAPPDATA` on windows; `$HOME/Library/Caches` on mac; `$XDG_CONFIG_DIR` or `$HOME/.cache` on other platforms | If set, we will cache the download artifacts/skia sources in this directory. or we will just use the default user cache dirs based on platform    |

\* We will only use that default IF it is avaialable. Otherwise, we will avoid setting that gn arg. eg: we will only use `clang` for `SKIA_CC` if `clang --version` works. otherwise, we would just let skia use the default `cc`.

\*\* We will look into LLVM/lib/clang directory, read the versions and set the version to latest we can find. Otherwise, we will just skip setting it.  






