## Ckia
Ckia is a 2D vector graphics library based on `Skia` with `SkiaSharp`'s "C" bindings. 

**NOTE**: Please also read [ckia_sys README](./ckia_sys/README.md) which explains build requirements and customization. 

#### Why not rust-skia (skia-safe)?
The main reason is compile times, and the table will outline the differences
| Step | rust-skia | ckia |
| -----| --------- | ---- |
| **bindings generation** | uses `bindgen` in `build.rs` to generate bindings from `C++` headers every time you start a fresh build. adds atleast 6+ seconds of compile time. | uses `bindgen_cli` to generate bindings from `C` headers when publishing a version and include the bindings.rs in sources. |
| **pre-built static libs** | yes. for *many* platforms. improves compile times, but needs to redownload for every fresh build | only for windows and linux for now. but we cache them in user's cache directory, so we only download them for the very first time you use a crate version. all cargo projects can then share the libs without redownloading. | 
| **performance** | As rust-skia generates bindings directly from `C++`, it can inline structs as fixed size opaque blobs of bytes, which allows you to build a struct on stack and use it. | As ckia prefers to use stable FFI bindings, it allocates most c++ structs on heap as points to opaque structs. This will **definitely** be slower at runtime than rust-skia's approach. |
| **shared libs** | ??? | ??? |
| **documentation** | rust-skia has better docs, as it parses them from the `C++` source files. | ckia will probably need to write a lot of custom docs. | 
| **stability** | rust-skia has been solid for years. | ckia is still new, and the maintainer (yours truly) doesn't have indepth knowledge of c++ yet, so the bindings might need more polishing (and breaking changes).  | 
| **API coverage** | covers most of skia api (except skottie/skscenegraph) | covers limited api from skia and skottie. will require more time to write bindings for other modules (scenegraph/paragraph/shaper etc..) | 
| **dependencies** | rust-skia doesn't have many runtime deps, but has a lot of build deps. It uses `ureq`, `tar`, `bindgen`, `flate`, `cc`, `serde_json`, `toml` etc.. which will all lead to *atleast* 15 seconds of compile time | This is where `ckia` shines and is the entire reason for this crate's existence. ckia has just `paste` as build-dependency with less than 2 seconds of build time. |

**NOTE**: The first time you add `ckia` to your build, we will cache the download artefacts. They are roughly 20 MB-ish, so it will be bottlenecked on your network bandwidth. But after caching that once, all your builds (different cargo projects or fresh buids within the same project after `cargo clean`) should take less than two seconds to compile `ckia`.
**NOTE2**: Our crate tries to use system utilities like `curl` or `tar` to avoid dependencies in build script. So, that would exclude older platforms like windows 7 from being supported (unless you install them). But win 10, mac and most linux distributions do have `curl` and `tar` installed by default.
