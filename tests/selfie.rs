use riscu::load_and_decode_object_file;
use std::{path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

pub fn compile_selfie() -> (TempDir, PathBuf) {
    let temp_dir = tempdir().unwrap();

    let dst_file = "selfie.o";
    let dst_file_path = temp_dir.path().join(dst_file);

    Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:/opt/dst", temp_dir.as_ref().display()))
        .arg("cksystemsteaching/selfie")
        .arg("/opt/selfie/selfie")
        .arg("-c")
        .arg("/opt/selfie/selfie.c")
        .arg("-o")
        .arg(format!("/opt/dst/{}", dst_file))
        .output()
        .expect("Selfie C* compile command was not successfull");

    (temp_dir, dst_file_path)
}

#[test]
fn decode_selfie_binary() {
    let (_, object_file) = compile_selfie();

    let result = load_and_decode_object_file(object_file);

    assert!(
        result.is_ok(),
        "can load an decode RISC-U binaries from latest Selfie"
    );
}
