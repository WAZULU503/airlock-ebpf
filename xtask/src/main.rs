use std::process::Command;

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some("build-ebpf") => build_ebpf(),
        Some("run") => run(),
        _ => {
            eprintln!("usage:");
            eprintln!("  cargo run -p xtask -- build-ebpf");
            eprintln!("  cargo run -p xtask -- run");
            std::process::exit(1);
        }
    }
}

fn build_ebpf() {
    let status = Command::new("cargo")
        .args([
            "+nightly",
            "build",
            "-Z",
            "build-std=core",
            "--target",
            "bpfel-unknown-none",
            "--release",
            "-p",
            "airlock-clean-ebpf",
        ])
        .status()
        .expect("failed to execute cargo build");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run() {
    let status = Command::new("cargo")
        .args([
            "+nightly",
            "run",
            "-p",
            "airlock-clean",
        ])
        .status()
        .expect("failed to execute cargo run");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
