use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub languages: Vec<Language>,
}

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
}

impl Language {
    pub fn compile(&self) {
        let path = Path::new("languages").join(&self.name).join("src");

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

fn main() -> anyhow::Result<()> {
    let profile = std::env::var("PROFILE").unwrap();

    if profile.as_str() == "release" {
        let config = std::fs::read_to_string("languages/index.toml")?;
        let config = toml::from_str::<Config>(&config)?;
        let handles: Vec<_> = config.languages.into_iter().map(|lang| std::thread::spawn(move || lang.compile())).collect();

        for handle in handles {
            handle.join().expect("Compilation thread should not panic")
        }
    }

    Ok(())
}
