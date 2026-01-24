// 构建脚本
fn main() {
    // 设置 Windows 子系统为 windows（无控制台）
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    }
}
