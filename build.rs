use std::env;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
use libbpf_cargo::SkeletonBuilder;
#[cfg(target_family = "unix")]
const SRC: &str = "src/linux/bpf/sensor.bpf.c";

fn main() {
  if cfg!(target_family = "unix") {
    let mut out = PathBuf::from(env::var_os("OUT_DIR").expect("No OUT_DIR env var set"));
    out.push("sensor.skel.rs");
    SkeletonBuilder::new()
        .source(SRC)
        .build_and_generate(&out)
        .unwrap();
    println!("cargo:rerun-if-changed={SRC}");
  }
}
