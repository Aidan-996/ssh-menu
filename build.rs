// build.rs — generate Windows ICO from assets/icon.svg and embed it into
// the executable as a resource. No external tools required.
//
// Pipeline: resvg rasterizes SVG at multiple sizes → ico crate packs
// them into a single .ico → winresource embeds that .ico into the PE.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.svg");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(windows)]
    {
        if let Err(e) = build_windows_icon() {
            // Don't fail the build if icon generation fails; just warn.
            println!("cargo:warning=icon generation skipped: {}", e);
        }
    }
}

#[cfg(windows)]
fn build_windows_icon() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    let out_dir = std::env::var("OUT_DIR")?;
    let svg_path = PathBuf::from("assets/icon.svg");
    if !svg_path.exists() {
        return Err("assets/icon.svg not found".into());
    }
    let svg_data = std::fs::read(&svg_path)?;

    // Parse SVG once.
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &opt)?;

    // Render to multiple PNG sizes.
    let sizes = [16u32, 24, 32, 48, 64, 128, 256];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &sizes {
        let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size)
            .ok_or("pixmap alloc failed")?;
        let tree_size = tree.size();
        let sx = size as f32 / tree_size.width();
        let sy = size as f32 / tree_size.height();
        let transform = resvg::tiny_skia::Transform::from_scale(sx, sy);
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        let image = ico::IconImage::from_rgba_data(size, size, pixmap.data().to_vec());
        icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
    }

    let ico_path = PathBuf::from(&out_dir).join("ssh-menu.ico");
    let mut file = std::fs::File::create(&ico_path)?;
    icon_dir.write(&mut file)?;
    drop(file);

    // Embed into the PE.
    let mut res = winresource::WindowsResource::new();
    res.set_icon(ico_path.to_str().ok_or("ico path non-utf8")?);
    res.set("ProductName", "ssh-menu");
    res.set("FileDescription", "Interactive TUI SSH connection manager");
    res.set("CompanyName", "Aidan-996");
    res.set("LegalCopyright", "MIT © 2026 Aidan-996");
    res.compile()?;
    Ok(())
}
