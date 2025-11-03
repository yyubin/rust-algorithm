use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_bin_path = Path::new(&manifest_dir).join("src/bin");
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");

    // 기존 Cargo.toml에서 이미 등록된 bin 이름 읽기
    let existing_bins = read_existing_bin_names(&cargo_toml_path);

    // 새로 추가할 bin 섹션들 생성
    let rs_files = collect_rs_files(&src_bin_path);
    let mut new_entries = String::new();

    for file_path in rs_files {
        let rel_path = file_path.strip_prefix(&manifest_dir).unwrap();
        let name = file_path.file_stem().unwrap().to_string_lossy().to_string();

        // 이미 있는 항목은 스킵
        if existing_bins.contains(&name) {
            println!("Skipping existing bin: {}", name);
            continue;
        }

        new_entries.push_str(&format!(
            "\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n",
            name,
            rel_path.display()
        ));
    }

    // 새 항목이 있으면 Cargo.toml에 병합
    if !new_entries.is_empty() {
        println!("Adding new binaries to Cargo.toml...");
        let original = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
        let mut merged = original.trim_end().to_string();
        merged.push_str("\n");
        merged.push_str(&new_entries);

        fs::write(&cargo_toml_path, merged).expect("Failed to update Cargo.toml");

        println!("✅ Cargo.toml updated with new [[bin]] entries.");
    } else {
        println!("No new binaries found. Cargo.toml unchanged.");
    }

    println!("cargo:rerun-if-changed=src/bin");
}

// 기존 Cargo.toml 내 [[bin]] 이름 추출
fn read_existing_bin_names(path: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if !path.exists() {
        return names;
    }

    let file = File::open(path).expect("Failed to read Cargo.toml");
    let reader = BufReader::new(file);
    let mut in_bin_block = false;

    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.starts_with("[[bin]]") {
            in_bin_block = true;
            continue;
        }
        if in_bin_block && trimmed.starts_with("name") {
            if let Some(name) = trimmed.split('"').nth(1) {
                names.push(name.to_string());
            }
            in_bin_block = false; // 한 블록 끝
        }
    }

    names
}

// src/bin 하위 모든 .rs 파일 탐색
fn collect_rs_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if dir.exists() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_rs_files(&path));
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                files.push(path);
            }
        }
    }
    files
}
