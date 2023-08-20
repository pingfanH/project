
fn main() {
    slint_build::compile("src/slint/main.slint").unwrap();
    
    //Build::new().compile("tray-example.rc").unwrap();
    //embed_resource::compile("tray-example.rc");
}
