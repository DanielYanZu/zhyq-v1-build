fn main() {
    // 设置 macOS 最低部署目标为 10.14
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
    }

    tauri_build::build()
}
