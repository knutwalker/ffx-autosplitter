fn main() {
    println!("cargo::rerun-if-env-changed=FFX_AS_DEBUG");
    println!("cargo::rerun-if-env-changed=FFX_AS_TESTING");
    println!("cargo::rustc-check-cfg=cfg(debugging)");
    println!("cargo::rustc-check-cfg=cfg(testing)");

    if let Ok("1") = std::env::var("FFX_AS_DEBUG").as_deref() {
        println!("cargo::rustc-cfg=debugging");
    }
    if let Ok("1") = std::env::var("FFX_AS_TESTING").as_deref() {
        println!("cargo::rustc-cfg=testing");
    }
}
