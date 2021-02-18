use riscu::load_and_decode_object_file;
use std::{path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};

fn with_temp_dir<F, R>(f: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let temp_dir = tempdir().unwrap();

    f(&temp_dir)
}

fn compile_selfie(temp_dir: &TempDir) -> PathBuf {
    let dst_file = "selfie.o";
    let dst_file_path = temp_dir.path().join(dst_file);

    Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:/opt/dst", temp_dir.path().display()))
        .arg("cksystemsteaching/selfie")
        .arg("/opt/selfie/selfie")
        .arg("-c")
        .arg("/opt/selfie/selfie.c")
        .arg("-o")
        .arg(format!("/opt/dst/{}", dst_file))
        .output()
        .expect("Selfie C* compile command was not successfull");

    dst_file_path
}

#[test]
fn decode_selfie_binary() {
    let result = with_temp_dir(|dir| {
        let object_file = compile_selfie(dir);

        load_and_decode_object_file(object_file)
    });

    assert!(
        result.is_ok(),
        "can load an decode RISC-U binaries from latest Selfie"
    );
}
