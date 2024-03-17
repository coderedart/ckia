## Ckia
Ckia is a 2D vector graphics library based on `Skia` with `SkiaSharp`'s "C" bindings. 

**NOTE**: Please also read [ckia_sys README](./ckia_sys/README.md) which explains build requirements and customization. 

### Luau (mlua) bindings
For Luau type definitions, 
1. please install `luau-lsp` extension
2. copy the `types.d.luau` file to your project (preferably at workspace root).
3. add the following snippet to your vscode config with the path to your copied type definitions file
```json
"luau-lsp.types.definitionFiles": [
  "types.d.luau"
]
```

### Documentation
Skia's documentation is split over multiple places.

1. [Skia api](https://api.skia.org/) : contains the c++ source docs. If you want docs about a particular function or struct, this could be useful.
2. [Skia Discuss](https://groups.google.com/g/skia-discuss) : Skia forum. Most of the questions that you have are probably answered here.
3. [SkiaSharp](https://learn.microsoft.com/en-us/xamarin/xamarin-forms/user-interface/graphics/skiasharp/) : It shows you how to *use* all the functionality that Skia provides. Contains example images, which help a lot when you are starting with Skia.
4. [Flutter API](https://api.flutter.dev/flutter/dart-ui/Canvas-class.html) : Flutter draws its UI using Skia, and it has *a lot* of user friendly documentation that can be understood by UI devs. In particular, this is the only resouce which actually contains the docs for text api of Skia. It also has the best docs for explaining [blend modes](https://api.flutter.dev/flutter/dart-ui/BlendMode.html) with composite image samples.
5. [React Native Skia API](https://shopify.github.io/react-native-skia/docs/canvas/overview/) : Because of its xml like api, its *very* easy to understand teh docs. The docs unlike Flutter, are short and to the point. Great for a quick overview of most of the Skia features.
6. [Fiddle](https://fiddle.skia.org/named/) : provides a sandboxed playground online, to play with Skia. Provides lots of examples to explain various concepts, and you can edit the code to run and see the changes live in browser. 
7. [React Native Playlist](https://www.youtube.com/playlist?list=PLkOyNuxGl9jxyFx9YSRvy6dumPhofM3fs) : This is an excellent channel which shows how you can use the drawing commands to create pretty looking widgets. It uses react native, but you will still learn a lot about skia if you just follow along.

#### Why not rust-skia (skia-safe)?
The main reason is compile times, and the table will outline the differences
| Step | rust-skia | ckia |
| -----| --------- | ---- |
| **bindings generation** | uses `bindgen` in `build.rs` to generate bindings from `C++` headers every time you start a fresh build. adds atleast 6+ seconds of compile time. | uses `bindgen_cli` to generate bindings from `C` headers when publishing a version and include the bindings.rs in sources. |
| **pre-built static libs** | yes. for *many* platforms. improves compile times, but needs to redownload for every fresh build | only for windows and linux for now. but we cache them in user's cache directory, so we only download them for the very first time you use a crate version. all cargo projects can then share the libs without redownloading. | 
| **performance** | As rust-skia generates bindings directly from `C++`, it can inline structs as fixed size opaque blobs of bytes, which allows you to build a struct on stack and use it. | As ckia prefers to use stable FFI bindings, it allocates most c++ structs on heap as pointers to opaque structs. This will **definitely** be slower at runtime than rust-skia's approach. |
| **shared libs** | ??? | ??? |
| **documentation** | rust-skia has better docs, as it parses them from the `C++` source files. | ckia will probably need to write a lot of custom docs. | 
| **stability** | rust-skia has been solid for years. | ckia is still new, and the maintainer (yours truly) doesn't have indepth knowledge of c++ yet, so the bindings might need more polishing (and breaking changes).  | 
| **API coverage** | covers most of skia api (except skottie/skscenegraph) | covers limited api from skia, paragraph and skottie. will require more time to write bindings for other modules (scenegraph/shaper/pdf/svg etc..) | 
| **dependencies** | rust-skia doesn't have many runtime deps, but has a lot of build deps. It uses `ureq`, `tar`, `bindgen`, `flate`, `cc`, `serde_json`, `toml` etc.. which will all lead to *atleast* 15 seconds of compile time | This is where `ckia` shines and is the entire reason for this crate's existence. ckia has just `paste` as build-dependency with less than 2 seconds of build time. |

**NOTE**: The first time you add `ckia` to your build, we will cache the download artefacts. They are roughly 20 MB-ish, so it will be bottlenecked on your network bandwidth. But after caching that once, all your builds (different cargo projects or fresh buids within the same project after `cargo clean`) should take less than two seconds to compile `ckia`.

**NOTE2**: Our crate tries to use system utilities like `curl` or `tar` to avoid dependencies in build script. So, that would exclude older platforms like windows 7 from being supported (unless you install them). But win 10, mac and most linux distributions do have `curl` and `tar` installed by default.

### ckia_sys

This project will take Skia and generate rust bindings for it. It also takes care of building Skia.

fast compile time is the highest priority. and we do this by:
1. providing prebuilt static and dynamic libraries, which can be downloaded by build script for common configurations
2. maintain manual C bindings and use pre-generated bindings, instead of generating bindings at build time. avoids bindgen dependency and cuts at least `6` seconds of build time. 

### Generate Bindings
1. install bindgen with `cargo install bindgen`
2. run 
```sh
# merge extern blocks creates a smaller and denser bindings.rs
# no-layout-tests avoids a lot of tests and makes bindings.rs cleaner. But for sanity, we might generate layout tests first, run cargo test, and then generate bindings without tests to publish. 
# -ISkia is necessary to let clang know where to search for include paths.
bindgen --merge-extern-blocks --default-enum-style rust --no-layout-tests -o bindings.rs Skia/ckia/src/sk_all.c -- -ISkia/
```
3. Finally, fix the functions that need a different ABI on Windows.
    1. `gr_vk_get_proc`. change its ABI to `extern "system"`, so that it can use `__stdcall` ABI on Windows platforms.

### Build Dependencies
1. `curl` -> to download the pre-built libs
2. `tar` -> to extract the archives after downloading.
Both Windows 10 / Linux / macOS have these by default. and are enough if you are using pre-built libs.

If you are building from scratch:
1. `git` -> to clone Skia repo and dependencies
2. `python` -> to download build dependencies and run build scripts
3. `tar` (optional) -> to package libs before copying them to `SKIA_COPY_LIBS` (see below)
4. `curl` ->  download source tarballs from Github.
5. `clang` and `clang++`. (LLVM). 
6. `sccache` or `ccache` (optional) -> for caching compiled objects, as `cargo clean` will remove everything from `/target` eventually. highly recommended.
7. Lots of patience. Skia is going through a phase of changing build systems from `gn` to `bazel`. So, you *will* probably hit some issues.
8. Even more patience. While Skia is a joy to use when drawing, it is absolutely horrible when compiling. 

### How is Skia built?
Skia requires some complex build steps. Here's a rough outline which you will need to understand if you want to make custom builds.

1. There are two different repositories involved. 
    1. https://github.com/coderedart/Skia which contains Skia source + `SkiaSharp`'s stable FFI bindings + pre-build static and dynamic Skia libraries, which can be used by `ckia_sys` to skip building from source.
    2. https://github.com/coderedart/ckia_sys which provides
        1. rust FFI bindings using bindgen 
        2. `ckia_sys`'s `build.rs` for building Skia
2. Skia build consists of the following steps
    1. download/clone the Skia source (from step 1.1). change the current working directory into the Skia project root.
    3. ensure that required build dependencies are installed from the Dependencies section above. On `apt`(Debian) based systems, you can run the script `./tools/install_dependencies.sh --yes`
    4. run `python ./tools/git-sync-deps`, so that Skia can download sources of all its third_party dependencies into `./third_party/externals`. It will also download `gn` (generate ninja) tool into `./bin` based on the current platform.
    5. now, you set your build configuration and build directory with the command `./bin/gn gen path_to_build_dir --args='option1=true option2="clang"` where `option*=value` is the build option you want to configure. Some common configs are
        1. `is_official_build=bool`: whether this is a release build or not. this enables debug info (useful for debugging), Spirv validation, extra debug asserts etc.
        2. `is_component_build=bool`: whether we want dynamic/shared libraries or static libraries. we usually want to link statically, so this is often false. 
        3. `Skia_use_system_X=bool`: `X` can be any of `expat`,`freetype2`,`harfbuzz`,`icu`,`libjpeg_turbo`,`libpng`,`libwebp`,`zlib`. whether we want `X` to be linked with the system library. If it is false, then we build from scratch and statically link it. recommended to just build from src, because there could be version mismatches etc..
        4. `Skia_use_no_X_encode=bool`: `X` can be `jpeg`, `png` or `webp`. If you enable this, encoding into those formats will be disabled. If you have your custom image encoding libs, then you would disable these. 
        5. `Skia_use_X=bool` or `Skia_enable_X`: `X` can be many things. 
            1. `gl`, `vulkan`, `metal`, `direct3d`: which apis to enable.
            2. `ganesh`, `graphite`, `dawn`: which backend to use. `ganesh` is the mature backend. `graphite` is the next successor that will eventually replace `ganesh`. idk about dawn.
            3. `harfbuzz`, `freetype`, `icu` etc.. all the dependencies and features. 
        6. `target_X`: where X can be `cpu`, `arch`, platform etc.. which are useful for cross-compilation. you will probably use some additional options like `ndk_dir="/path/to/android/ndk"` etc.. that may be useful.
        7. `clang_win="X"` and `clang_win_version="X"`: The first is the path to LLVM installation (eg: `C:\\Program Files\\LLVM`) and the second is the LLVM version installed (eg: `16`).
        8. `cc="X"` is C compiler (`clang` or `cc`). `cxx="X"` is a C++ compiler. `cc_wrapper="X"` is a wrapper like `ccache` or `sccache`.
    5. Finally, you build the binaries with `ninja -C path_to_build_dir thing_to_build`. `thing_to_build` is optional, as it will just build the default "thing" which happens to be `Skia`. built objects will be in the build directory.
3. `ckia_sys`'s `build.rs` doesn't want to be *too* complex in trying to support *all* these options. Especially when the maintainer (your truly) doesn't even have hardware like mac/ios to verify that the builds are working properly. So, we simplify the build system a lot for the common case, and expect people to use custom "from source" builds for their niche use cases. We will of course do our best to gather documentation (like this current README) to help people get their custom builds working.
    1. By default, this repo provides prebuilt static and dynamic Skia libs, with as many options enabled as possible. So, when you use it on `windows`, `linux` or `mac`,
        1. the build script checks if there's already an archive downloaded in `SKIA_CACHE_DIR` (if env var is set) or user's cache (eg: `$HOME/.cache/ckia_sys`).
        2. If it doesn't exist, we will just download the archive from github, place it in the cache directory.
        3. Now that we have the libs archive, we extract it into our `OUT_DIR` (provided by cargo) under `target`.
        4. We will just print the relevant `rust-link-lib` directives to help cargo decide which libs to link and where to find them.
        5. The advantage is that we only have to download the libs once and keep reusing the cached archive between different cargo project builds or after `cargo clean` in the same project. 
        6. After the first libs archive download, the next build should take less than a second. 
    2. If you enable `disable_download_pre_built_Skia_libs` feature, then we will skip previous step. And try to build from source. 
        1. If `SKIA_SRC_DIR` is set, we will use that as the source directory. If not, We download Skia source tarball into cache directory, extract the sources in cache directory. If `SKIA_SRC_ARCHIVE_URL` is set, we will use that to download source archive, otherwise we default to `coderedart/Skia` repo's releases. 
        2. pull third_party dependencies and `gn` using python (step 2.4). If you want us to use a specific python executable, then set `SKIA_PYTHON=/path/to/python`. Otherwise, we will just use `python` and let the shell look up the default python installation.   
        3. Just like caching downloaded pre-built libs, this will ensure that all of your cargo projects will reuse the same Skia source and its third_party lib sources which will easily reach **hundreds of MB** worth bandwidth!!!
        4. Now that we have the required sources, we will generate build config with build_directory set to `OUT_DIR`.
        5. We will do our best to use `clang`/`clang++` as compilers or `ccache`/`sccache` as cc_wrappers if we can find them. To make it easier, you can set `SKIA_*` vars like `SKIA_CC`,`SKIA_CC_WRAPPER`,`SKIA_CXX`,`SKIA_CLANG_WIN`,`SKIA_CLANG_WIN_VERSION`. 
        6. Instead of using Skia fetched `./bin/gn`, if you want us to use a specific executable, just set `SKIA_GN=/path/to/gn`.
        7. If you want to add any special build configurations, then just set `SKIA_GN_ARGS='key1=value1;key2="value2";...'` where key/value pairs are separated by `;`. These will be added towards the end, so they will override any of the settings that build.rs sets by default.
        8. We will also print the final args using `gn args build_dir --list --short`, for easy debugging of the build script on errors. 
        9. We call `ninja -C build_dir` to build it. If you want to provide a path for `ninja`, then just set `SKIA_NINJA=/path/to/ninja`. 
        10. we will ask gn what libs Skia needs, and we will check which libs are built in `OUT_DIR`. For all libs we built from src, build script will print cargo directives like `rustc-link-lib=static=libname` for static linking. And for libs that Skia needs, but didn't build (eg: `GL`, `X11`, `stdc++`, etc..), we will print cargo directives for dynamic linking from system. 
    3. If you enable `disable_build_from_src` feature, we will skip the previous step. If both previous steps are skipped
        1. IF `dynamic_linking` feature is enabled, then we will just print cargo directives for dynamic linking of Skia library
        2. If `dynamic_linking` is NOT enabled, then we don't do anything :)
    4. You can use the `disable_caching_downloads_in_user_cache` feature to skip caching the download artifacts (eg: in CI). 

### Build 
Env Vars that affect build are organized in a nice table here.
| name              | default               | purpose                                                                               |
| ------            | -----                 | -------                                                                               |
| SKIA_PYTHON       | python                | path to python, which we will use when building Skia from src                         |
| SKIA_GN           | ./bin/gn              | path to `gn` binary. Skia will download it for us, but you can specify a custom path  |
| SKIA_CC           | clang*                | c compiler. clang is preferred by Skia                                                |
| SKIA_CC_WRAPPER   | sccache* or ccache*   | improves compile times by using a global cache. highly recommended                    |
| SKIA_CXX          | clang++*              | c++ compiler. clang++ is preferred by Skia                                            |
| SKIA_CLANG_WIN    | C:\\Program Files\\LLVM* | path to LLVM installation. required for using clang on windows                    |
| SKIA_CLANG_WIN_VERSION | SKIA_CLANG_WIN/lib/clang/version** | version of clang to use. ckia_sys will try to use the most recent version in SKIA_CLANG_WIN/lib/clang/. required for using clang on windows |
| SKIA_GN_ARGS      | ""                    | addditional GN args to pass to Skia build for custom builds from src.  |
| SKIA_COPY_LIBS    | ""                    | tells ckia_sys build system to copy the built binaries to the directory based on the path set in this var. useful in workflows to distribute libs after building from src |
| SKIA_SRC_DIR      | ""                    | If set, we will use this as the source for the build instead of downloading source tarball from github |
| SKIA_SRC_ARCHIVE_URL | ""                 | If set, we will use curl to download this source tarball (.tar.gz), and extract it in cache dir, and use that as the Skia src directory. Otherwise, we will just download from github |
| SKIA_CACHE_DIR    | `$LOCALAPPDATA` on windows; `$HOME/Library/Caches` on mac; `$XDG_CONFIG_DIR` or `$HOME/.cache` on other platforms | If set, we will cache the download artifacts/Skia sources in this directory. or we will just use the default user cache dirs based on platform    |

\* We will only use that default IF it is available. Otherwise, we will avoid setting that gn arg. eg: we will only use `clang` for `SKIA_CC` if `clang --version` works. otherwise, we would just let Skia use the default `cc`.

\*\* We will look into `LLVM/lib/clang` directory, read the versions and set the version to latest we can find. Otherwise, we will just skip setting it.  
