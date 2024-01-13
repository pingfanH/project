use windres::Build;
fn main() {
    Build::new().compile("tray-example.rc").unwrap();
    slint_build::compile("src/slint/main.slint").unwrap();
}
