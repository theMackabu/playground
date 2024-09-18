use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use tar::Archive;
use zstd::Decoder;

#[derive(Deserialize)]
pub struct Config {
    pub languages: Vec<Language>,
}

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
}

impl Language {
    pub fn compile(&self, languages_dir: &Path) -> Result<()> {
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
        Ok(())
    }
}

fn rerun_if_changed(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().to_str().unwrap());
}

fn extract_languages(languages_dir: &Path) -> Result<()> {
    if languages_dir.exists() {
        println!("Languages directory already exists. Skipping extraction.");
        return Ok(());
    }

    println!("Extracting languages...");
    let languages_archive = Path::new("languages.tar.zst");
    let file = File::open(languages_archive).context("Failed to open languages.tar.zst")?;
    let zstd_decoder = Decoder::new(file)?;
    let buf_reader = BufReader::new(zstd_decoder);
    let mut archive = Archive::new(buf_reader);

    archive.unpack(".").context("Failed to extract languages archive")?;

    println!("Extraction completed successfully.");
    Ok(())
}

fn main() -> Result<()> {
    let profile = std::env::var("PROFILE").unwrap();
    let languages_dir = Path::new("languages");

    match std::env::current_dir() {
        Ok(current_dir) => println!("Current working directory: {:?}", current_dir),
        Err(e) => println!("Failed to get current directory: {}", e),
    }

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?}", entry.path());
    }

    if profile.as_str() == "release" {
        extract_languages(languages_dir)?;

        let config = fs::read_to_string("languages.toml")?;
        let config: Config = toml::from_str(&config)?;

        for lang in config.languages {
            lang.compile(languages_dir).with_context(|| format!("Failed to compile language: {}", lang.name))?;
        }
    }

    Ok(println!("cargo:rerun-if-changed=languages.tar.zst"))
}
