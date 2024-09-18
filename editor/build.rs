use anyhow::Context;
use serde::Deserialize;
use std::fs::{self, File};
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;

#[derive(Deserialize)]
pub struct Config {
    pub languages: Vec<Language>,
}

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
}

impl Language {
    pub fn compile(&self, languages_dir: &Path) {
        let path = languages_dir.join(&self.name).join("src");

        let has_scanner = path.join("scanner.c").exists() || path.join("scanner.cc").exists();
        let scanner_is_cpp = path.join("scanner.cc").exists();

        let mut build = cc::Build::new();

        let parser_path = path.join("parser.c");

        let build = build.include(&path).flag_if_supported("-w").flag_if_supported("-s").flag_if_supported("-O2").file(&parser_path);

        rerun_if_changed(&parser_path);

        if has_scanner && !scanner_is_cpp {
            let scanner_path = path.join("scanner.c");
            rerun_if_changed(&scanner_path);
            build.file(&scanner_path);
        } else if scanner_is_cpp {
            let mut build = cc::Build::new();

            let scanner_path = path.join("scanner.cc");
            rerun_if_changed(&scanner_path);

            build
                .cpp(true)
                .include(&path)
                .flag_if_supported("-w")
                .flag_if_supported("-s")
                .flag_if_supported("-O2")
                .file(&scanner_path)
                .compile(&format!("{}-scanner", self.name));
        }

        build.compile(&format!("{}-parser", self.name));
    }
}

fn rerun_if_changed(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().to_str().unwrap());
}

fn extract_languages(languages_dir: &Path) -> anyhow::Result<()> {
    if languages_dir.exists() {
        println!("Languages directory already exists. Skipping extraction.");
        return Ok(());
    }

    let languages_archive = Path::new("languages.xz");
    let file = File::open(languages_archive).context("Failed to open languages.xz")?;
    let xz_decoder = XzDecoder::new(file);
    let mut archive = Archive::new(xz_decoder);

    archive.unpack(languages_dir).context("Failed to extract languages archive")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let profile = std::env::var("PROFILE").unwrap();
    let languages_dir = Path::new("languages");

    if profile.as_str() == "release" {
        extract_languages(languages_dir)?;

        let config = fs::read_to_string(languages_dir.join("index.toml"))?;
        let config = toml::from_str::<Config>(&config)?;
        let handles: Vec<_> = config.languages.into_iter().map(|lang| std::thread::spawn(move || lang.compile(&languages_dir))).collect();

        for handle in handles {
            handle.join().expect("Compilation thread should not panic")
        }
    }

    Ok(println!("cargo:rerun-if-changed=languages.xz"))
}
