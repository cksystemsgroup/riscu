use riscu::load_and_decode_object_file;
use std::{env, fs::File, path::PathBuf, process::Command};
use tempfile::{tempdir, TempDir};
use which::which;

fn compile_selfie(temp_dir: &TempDir) -> PathBuf {
    let selfie_path = ensure_selfie_installed();

    let mut selfie_source = selfie_path.clone();
    selfie_source.set_extension("c");

    let dst_file_path = temp_dir.path().join("selfie.o");

    File::create(&dst_file_path).unwrap();

    let mut cmd = Command::new(selfie_path);

    cmd.arg("-c")
        .arg(selfie_source)
        .arg("-o")
        .arg(&dst_file_path);

    println!("command: {:?}", cmd);

    cmd.status()
        .expect("Selfie C* compile command was not successfull");

    dst_file_path
}

fn ensure_selfie_build_tools_installed() {
    let _ =
        which("make").expect("Can not find make in $PATH. Make has to be installed on your system");

    if cfg!(target_os = "windows") {
        let _ = which("gcc")
            .expect("Can not find gcc in $env:PATH. Mingw64 has to be installed on your system");
    } else {
        let _ = which("cc")
            .expect("Can not find cc in $PATH. A C compiler has to be installed on your system");
    }
}

fn ensure_selfie_installed() -> PathBuf {
    ensure_selfie_build_tools_installed();

    let repo_dir = env::temp_dir().join("riscu-crate-selfie-installation");

    if !repo_dir.exists() {
        std::fs::create_dir(&repo_dir).unwrap();

        Command::new("git")
            .arg("clone")
            .arg("https://github.com/christianmoesl/selfie")
            .arg(&repo_dir)
            .status()
            .expect("Selfie Git repository could not be cloned from Github");
    }

    let mut cmd = Command::new("make");

    if cfg!(target_os = "windows") {
        cmd.env("CC", "gcc");
    }

    cmd.arg("selfie")
        .current_dir(&repo_dir)
        .status()
        .expect("Selfie can not be compiled");

    if cfg!(target_os = "windows") {
        repo_dir.join("selfie.exe")
    } else {
        repo_dir.join("selfie")
    }
}

fn with_temp_dir<F, R>(f: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let temp_dir = tempdir().unwrap();

    f(&temp_dir)
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
