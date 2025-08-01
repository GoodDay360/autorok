use {
    std::{env, io},
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if cfg!(debug_assertions) == false { 
        if target == "windows"{
            let mut res = WindowsResource::new();
            res.set_manifest_file("app.window.manifest");
            res.set_icon("resource/icon.ico");
            res.compile()?;
            println!("Embedding Windows manifest + icon...");
        } else {
            println!("Skipping Windows-specific resource embedding for OS: {target}");
        }
    }
    

    Ok(())
}
