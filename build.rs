use windres::Build;
fn main() {
    Build::new().compile("tray-example.rc").unwrap();
    slint_build::compile("src/slint/main.slint").unwrap();
    
    //Build::new().compile("tray-example.rc").unwrap();
    //embed_resource::compile("tray-example.rc");
}
