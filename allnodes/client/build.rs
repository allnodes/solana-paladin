use std::{path::PathBuf, process::Command};

const TAG_ENV_VAR: &str = "TAG";

fn set_client_version(tag: &str) {
    println!("cargo:rustc-env=ALLNODES_CLIENT_VERSION=paladin/{tag}");
}

fn main() {
    let git_root = PathBuf::from(format!(
        "{}/../../",
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set")
    ))
    .canonicalize()
    .expect("Failed to canonicalize git root");

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}/.git/refs/tags",
        git_root.display()
    );
    println!("cargo:rerun-if-env-changed={TAG_ENV_VAR}");
    if let Ok(tag) = std::env::var(TAG_ENV_VAR) {
        set_client_version(&tag);
        return;
    }
    let git_commit_tag = Command::new("git")
        .current_dir(git_root)
        .args(["describe", "--tags"])
        .output()
        .ok()
        .filter(|git_output| git_output.status.success())
        .and_then(|git_output| String::from_utf8(git_output.stdout).ok())
        .unwrap_or_else(|| {
            panic!(
                "Git tag is not set for this commit. Please define git commit tag or set \
                `{TAG_ENV_VAR}` environment variable."
            )
        });

    set_client_version(git_commit_tag.trim());
}
