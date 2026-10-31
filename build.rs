#![allow(clippy::single_match)]

use std::env;

const LAME_DIR: &str = "lame-4.0";

#[cfg(unix)]
fn build() {
    let mut config = autotools::Config::new(LAME_DIR);

    config.disable("decoder", None);
    config.disable("frontend", None);

    let host = env::var("HOST").expect("To have env:HOST");
    let target = env::var("TARGET").expect("To have env:TARGET");
    if let Ok(override_host) = env::var("MP3LAME_SYS_OVERRIDE_HOST") {
        config.config_option("host", Some(override_host.as_str()));
    } else if host != target {
        #[cfg(not(feature = "target_host"))]
        {
            if target.contains("android") {
                //Assume cross-compilation for android target
                config.config_option("host", Some(target.as_str()));
            } else if target.contains("apple") {
                if target.starts_with("aarch64") {
                    config.config_option("host", Some("arm-apple-darwin"));
                } else if target.starts_with("x86_64") {
                    config.config_option("host", Some("x86_64-apple-darwin"));
                } else {
                    println!("cargo:warning=Unsupported Apple target");
                }
            } else {
                println!("cargo:warning=Cross-compilation may not be supported");
            }
        }
        #[cfg(feature = "target_host")]
        {
            config.config_option("host", Some(target.as_str()));
        }
    }

    //Android cross compilation may require this flag
    if target.contains("android") || target.contains("ios") {
        config.cflag("-DSTDC_HEADERS");
    }

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
    let lame_dir = std::path::Path::new(LAME_DIR);
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
      .file(lame_dir.join("libmp3lame/quantize.c"))
      .file(lame_dir.join("libmp3lame/reservoir.c"))
      .file(lame_dir.join("libmp3lame/set_get.c"))
      .file(lame_dir.join("libmp3lame/tables.c"))
      .file(lame_dir.join("libmp3lame/takehiro.c"))
      .file(lame_dir.join("libmp3lame/util.c"))
      .file(lame_dir.join("libmp3lame/vbrquantize.c"))
      .file(lame_dir.join("libmp3lame/VbrTag.c"))
      .file(lame_dir.join("libmp3lame/version.c"))
      .include("src/include")
      .include(lame_dir.join("include"))
      .include(lame_dir.join("libmp3lame"))
      .define("HAVE_CONFIG_H", None)
      .pic(false)
      .warnings(false);

    let target = env::var("TARGET").expect("To have env:TARGET");
    //Only x64 guarantees SSE2 availability
    if target.contains("x86_64") {
        cc.file(lame_dir.join("libmp3lame/vector/xmm_quantize_sub.c"))
          .define("MIN_ARCH_SSE", None);
    }

    if let Ok(compiler) = env::var("CC") {
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

    build();
}
