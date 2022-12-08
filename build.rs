const LAME_DIR: &str = "lame-3.100";

#[cfg(feature = "build-bindgen")]
fn generate_bindings() {
    use std::path::PathBuf;

    #[derive(Debug)]
    struct ParseCallbacks;

    impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
        fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
            if name.starts_with("OPUS") {
                Some(bindgen::callbacks::IntKind::Int)
            } else {
                None
            }
        }
    }

    const PREPEND_LIB: &'static str = "
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
";

    let mut out = PathBuf::new();
    out.push("src");
    out.push("lib.rs");
    let mut lame_header = PathBuf::new();
    lame_header.push(LAME_DIR);
    lame_header.push("include");
    lame_header.push("lame.h");

    let bindings = bindgen::Builder::default().header(lame_header.display().to_string())
                                              .raw_line(PREPEND_LIB)
                                              .parse_callbacks(Box::new(ParseCallbacks))
                                              .generate_comments(false)
                                              .layout_tests(false)
                                              .ctypes_prefix("libc")
                                              .constified_enum_module("*")
                                              .allowlist_function("lame.+")
                                              .allowlist_var("lame.+")
                                              .allowlist_function("hip.+")
                                              .allowlist_var("hip.+")
                                              .allowlist_function("id3tag.+")
                                              .allowlist_var("id3tag.+")
                                              .use_core()
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");
}

#[cfg(not(feature = "build-bindgen"))]
fn generate_bindings() {
}

#[cfg(unix)]
fn build() {
    let mut config = autotools::Config::new(LAME_DIR);

    #[cfg(feature = "decoder")]
    config.enable("decoder", None);
    #[cfg(not(feature = "decoder"))]
    config.disable("decoder", None);

    let res = config.disable_shared()
                    .enable_static()
                    .disable("rpath", None)
                    .disable("frontend", None)
                    .disable("gtktest", None)
                    .with("pic", None)
                    .fast_build(true)
                    .build();

    //libraries are installed in <out>/lib
    println!("cargo:rustc-link-search=native={}/lib", res.display());
    println!("cargo:rustc-link-lib=static=mp3lame");
}

//On windows we cannot just use `nmake` as VS solution there doesn't have x64 files
//so instead just directly compile it.
//On unix targets we just rely on autotools to figure shit out
#[cfg(windows)]
fn build() {
    const INCLUDE_MSVC: &str = "include_msvc";

    let lame_dir = std::path::Path::new(LAME_DIR);
    //copy config.h
    let _ = std::fs::create_dir(lame_dir.join(INCLUDE_MSVC));
    std::fs::copy(lame_dir.join("configMS.h"), lame_dir.join(INCLUDE_MSVC).join("config.h")).expect("Copy config.h");

    let mut cc = cc::Build::new();
    cc.warnings(false)
      .extra_warnings(false)
      .file(lame_dir.join("libmp3lame/bitstream.c"))
      .file(lame_dir.join("libmp3lame/encoder.c"))
      .file(lame_dir.join("libmp3lame/fft.c"))
      .file(lame_dir.join("libmp3lame/gain_analysis.c"))
      .file(lame_dir.join("libmp3lame/id3tag.c"))
      .file(lame_dir.join("libmp3lame/lame.c"))
      .file(lame_dir.join("libmp3lame/newmdct.c"))
      .file(lame_dir.join("libmp3lame/presets.c"))
      .file(lame_dir.join("libmp3lame/psymodel.c"))
      .file(lame_dir.join("libmp3lame/quantize_pvt.c"))
      .file(lame_dir.join("libmp3lame/vector/xmm_quantize_sub.c"))
      .file(lame_dir.join("libmp3lame/quantize.c"))
      .file(lame_dir.join("libmp3lame/reservoir.c"))
      .file(lame_dir.join("libmp3lame/set_get.c"))
      .file(lame_dir.join("libmp3lame/tables.c"))
      .file(lame_dir.join("libmp3lame/takehiro.c"))
      .file(lame_dir.join("libmp3lame/util.c"))
      .file(lame_dir.join("libmp3lame/vbrquantize.c"))
      .file(lame_dir.join("libmp3lame/VbrTag.c"))
      .file(lame_dir.join("libmp3lame/version.c"))
      .include(lame_dir.join("include"))
      .include(lame_dir.join(INCLUDE_MSVC))
      .include(lame_dir.join("libmp3lame"))
      .define("TAKEHIRO_IEEE754_HACK", None)
      .define("FLOAT8", Some("float"))
      .define("REAL_IS_FLOAT", Some("1"))
      .define("BS_FORMAT", Some("BINARY"))
      .define("HAVE_CONFIG_H", None)
      .shared_flag(false)
      .pic(false)
      .warnings(false);

    #[cfg(feature = "decoder")]
    {
        cc.define("HAVE_MPGLIB", None)
          .include(lame_dir.join("mpglib"))
          .file(lame_dir.join("mpglib/common.c"))
          .file(lame_dir.join("mpglib/dct64_i386.c"))
          .file(lame_dir.join("mpglib/decode_i386.c"))
          .file(lame_dir.join("mpglib/interface.c"))
          .file(lame_dir.join("mpglib/layer1.c"))
          .file(lame_dir.join("mpglib/layer2.c"))
          .file(lame_dir.join("mpglib/layer3.c"))
          .file(lame_dir.join("mpglib/tabinit.c"))
          .file(lame_dir.join("libmp3lame/mpglib_interface.c"));
    }

    if let Ok(compiler) = std::env::var("CC") {
        let compiler = std::path::Path::new(&compiler);
        let compiler = compiler.file_stem().expect("To have file name in CC").to_str().unwrap();
        match compiler {
            //because `cc` crate is retarded and cannot handle clang-cl correctly
            "clang-cl" => {
                cc.flag("/W0");
            },
            _ => (),
        }
    }

    cc.compile("mp3lame")
}

fn main() {
    if std::env::var("DOCS_RS").map(|docs| docs == "1").unwrap_or(false) {
        //skip docs.rs build
        return;
    }

    generate_bindings();

    build();
}
