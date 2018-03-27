extern crate cc;
extern crate pkg_config;

fn main() {
    let mut gcc = cc::Build::new();

    let lib = pkg_config::Config::new()
        .atleast_version("2.0.0")
        .statik(true)
        .cargo_metadata(false)
        .probe("libevent")
        .unwrap();
    for inc_path in lib.include_paths.iter() {
        gcc.include(inc_path);
    }

    gcc.include("libbtc/include")
        .file("libbtc/src/net.c")
        .file("libbtc/src/vector.c")
        .file("libbtc/src/memory.c")
        .file("libbtc/src/chainparams.c")
        .file("libbtc/src/cstr.c")
        .static_flag(true)
        .compile("libbtc.a")
}
