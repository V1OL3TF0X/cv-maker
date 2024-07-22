use anyhow::Result;
use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions},
};
use std::io::Write;
use std::path::PathBuf;
use std::{
    env,
    fs::{create_dir_all, read_dir, File},
};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/css");
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut css_root = root.clone();
    css_root.push("src");
    css_root.push("css");
    let mut assets_root = root.clone();
    assets_root.push("assets");
    create_dir_all(assets_root.clone())?;
    read_dir(css_root)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|fp| fp.extension().is_some_and(|e| e == "css"))
        .for_each(|file_path| {
            let fs = FileProvider::new();
            let parse_cfg = ParserOptions {
                flags: ParserFlags::NESTING,
                ..Default::default()
            };
            let mut bundler = Bundler::new(&fs, None, parse_cfg);
            let mut ss = bundler
                .bundle(&file_path)
                .expect("Couldn't bundle properly");
            ss.minify(MinifyOptions::default())
                .expect("Couldn't minify properly");
            let new_fp = assets_root.join(file_path.file_name().unwrap());
            let mut file = File::create(new_fp).unwrap();
            let contents = ss
                .to_css(PrinterOptions {
                    minify: true,
                    ..Default::default()
                })
                .unwrap()
                .code;
            write!(file, "{contents}").expect("Coultn't write minified output to file");
        });
    Ok(())
}
