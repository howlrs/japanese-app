use std::{
    env, fs,
    path::{Path, PathBuf},
};

use log::{error, info};
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let target_dir = "questions";
    let target_levels = ["n3", "n2"];

    // レベルごとの実行
    // 対象ディレクトリを指定し、ファイルを読み込む
    for level in target_levels {
        let target_level_dir = {
            let current_dir = env::current_dir().unwrap();
            current_dir.join("output").join(target_dir).join(level)
        };
        // ディレクトリ内のファイルを走査
        let target_files = walk_dir(&target_level_dir);

        if target_files.is_empty() {
            error!("does not exists in {}", target_level_dir.display());
            continue;
        }

        // concat file string to one file
        let mut concat_content = String::new();
        for target_file in target_files {
            let read_content = read_file(target_file);
            concat_content.push_str(&read_content);
        }

        // save new file
        let new_file = "concat_all.md";
        let new_file_path = target_level_dir.join(new_file);
        write_file(new_file_path, &concat_content);
    }
    info!("done");
}

#[allow(unused)]
// 指定ディレクトリのファイルを走査する
fn walk_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        } else {
            files.push(path);
        }
    }
    files
}

fn read_file(abs_filename: PathBuf) -> String {
    std::fs::read_to_string(abs_filename).unwrap_or_else(|e| {
        panic!("ファイルの読み込みに失敗しました: {}", e);
    })
}

#[allow(unused)]
fn write_file(abs_filename: PathBuf, content: &str) {
    std::fs::write(abs_filename, content).unwrap_or_else(|e| {
        panic!("ファイルの書き込みに失敗しました: {}", e);
    });
}

#[allow(unused)]
fn replace_target(target: &str, line: &str) -> String {
    line.replace(target, "")
}
