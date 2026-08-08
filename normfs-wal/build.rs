use std::env;
use std::path::{Path, PathBuf};

fn uintn_include_dir(manifest_dir: &Path) -> PathBuf {
    match env::var_os("DEP_NORMFS_UINTN_C_INCLUDE") {
        Some(dir) => PathBuf::from(dir),
        None => manifest_dir.join("../uintn-rs/c/include"),
    }
}

fn crc32_target_supported(out_dir: &Path) -> bool {
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("x86_64") {
        return false;
    }

    let probe = out_dir.join("crc32c_target_probe.c");
    if std::fs::write(
        &probe,
        concat!(
            "#include <nmmintrin.h>\n",
            "#include <stdint.h>\n",
            "__attribute__((target(\"sse4.2,crc32\")))\n",
            "uint32_t normfs_crc32c_target_probe(uint32_t c, uint64_t w)\n",
            "{ return (uint32_t)_mm_crc32_u64((uint64_t)c, w); }\n",
        ),
    )
    .is_err()
    {
        return false;
    }

    cc::Build::new()
        .file(&probe)
        .flag("-std=c99")
        .warnings(false)
        .cargo_metadata(false)
        .cargo_warnings(false)
        .try_compile("normfs_crc32c_target_probe")
        .is_ok()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include_dir = manifest_dir.join("c/include");
    let uintn_include_dir = uintn_include_dir(&manifest_dir);
    let sources = [
        manifest_dir.join("c/src/wal_header.c"),
        manifest_dir.join("c/src/crc32c.c"),
        manifest_dir.join("c/src/wal_entry.c"),
    ];

    println!("cargo:rerun-if-changed={}", include_dir.display());
    println!("cargo:rerun-if-changed={}", uintn_include_dir.display());
    for source in &sources {
        println!("cargo:rerun-if-changed={}", source.display());
    }

    let mut build = cc::Build::new();
    build
        .files(&sources)
        .include(&include_dir)
        .include(&uintn_include_dir)
        .flag("-std=c99")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .flag("-pedantic")
        .opt_level(3)
        .warnings(false);

    if crc32_target_supported(&out_dir) {
        build.define("NORMFS_CRC32C_TARGET_CRC32", None);
    }

    build.compile("normfs_wal_c");
}
