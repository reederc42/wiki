use std::{collections::HashMap, ffi::OsStr, fs::{self, File}, io::Write, path::Path, process::Command};

const UI_SRC: &str = "../ui";
const DIST_DIR: &str = "../ui/dist";
const DIST_FILE: &str = "src/dist.rs";
const MIME_TYPES: &str = "mime_types.yaml";
const BUILD_OPTIONS_ENV: &str = "WIKI_CI_UI_BUILD_OPTIONS";

fn main() {
    for result in ignore::WalkBuilder::new("../")
        .filter_entry(|dent| {
            let p = dent.clone().into_path();
            p.starts_with(UI_SRC)
        })
        .build().flatten() {
        let p = result.into_path();
        if !p.is_dir() {
            println!("cargo:rerun-if-changed={}", p.display());
        }
    }
    println!("cargo:rerun-if-env-changed={}", BUILD_OPTIONS_ENV);

    let build_options_env = std::env::var(BUILD_OPTIONS_ENV)
        .unwrap_or(String::from(""));
    let build_options: Vec<&str> = build_options_env
        .split(' ')
        .collect();
    build_ui(&build_options);

    let assets = build_assets(Path::new(DIST_DIR));
    let mut m = phf_codegen::Map::new();
    for asset in assets {
        m.entry(
            asset.path,
            &format!(
                "crate::spa_server::Asset {{content_type: \"{}\", content: &{:?}}}",
                asset.content_type,
                asset.content,
            ),
        );
    }

    let mut out_file = File::create(Path::new(DIST_FILE)).unwrap();
    writeln!(
        &mut out_file,
        "pub static DIST: phf::Map<&'static str, crate::spa_server::Asset> = \n{};\n",
        m.build(),
    ).unwrap();
}

fn walk_dir(dir: &Path, f: &mut dyn FnMut(&Path)) {
    f(dir);

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            walk_dir(&entry.unwrap().path(), f);
        }
    }
}

fn build_ui(build_options: &[&str]) {
    Command::new("npm")
        .current_dir(UI_SRC)
        .arg("run").arg("build").arg("--")
        .args(build_options)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

struct Asset {
    path: String,
    content_type: String,
    content: Vec<u8>,
}

fn build_assets(dir: &Path) -> Vec<Asset> {
    let content_types_str = fs::read_to_string(MIME_TYPES).unwrap();
    let content_types: HashMap<&str, &str> =
        serde_yaml::from_str(&content_types_str).unwrap();

    let mut assets = Vec::new();

    walk_dir(dir, &mut |p: &Path| {
        if p.is_dir() {
            return
        }

        let content = match fs::read(p) {
            Err(_) => return,
            Ok(c) => c,
        };

        let ext = p.extension().and_then(OsStr::to_str).unwrap();
        let content_type = String::from(*content_types.get(ext).unwrap());

        let path = String::from(
            p.to_str().unwrap()
                .strip_prefix(dir.to_str().unwrap()).unwrap()
                .trim_start_matches('/')
        );

        assets.push(Asset{
            path,
            content_type,
            content,
        })
    });

    assets
}
