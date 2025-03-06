use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub level_id: u32,
    pub level_name: String,
    #[serde(default)]
    pub category_id: u32,
    pub category_name: String,

    pub chapter: String,
    pub sentence: String,
    pub prerequisites: String,
    pub sub_questions: Vec<SubQuestion>,
}

type SelectAnswer = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubQuestion {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub hint_id: u32,
    #[serde(default)]
    pub answer_id: u32,

    pub sentence: String,
    pub select_answer: Vec<SelectAnswer>,
    pub answer: String,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let target_dir = "questions";
    let target_levels = ["n2", "n3"];
    let target_file = "concat_all.json";

    // レベルごとの実行
    // 対象ディレクトリを指定し、ファイルを読み込む
    for level in target_levels {
        let target_level_dir = {
            let current_dir = env::current_dir().unwrap();
            current_dir.join("output").join(target_dir).join(level)
        };
        let target_level_filepath = target_level_dir.join(target_file);

        if !target_level_dir.exists() || !target_level_dir.is_dir() {
            error!("does not exists in {}", target_level_dir.display());
            continue;
        }

        if !target_level_filepath.exists() {
            error!("does not exists in {}", target_file);
            continue;
        }

        let read_content = read_file(target_level_filepath.clone());
        let to_questions = match serde_json::from_str::<Vec<Question>>(&read_content) {
            Ok(questions) => questions,
            Err(e) => {
                panic!(
                    "JSONのパースに失敗しました: {:?},  {}",
                    target_level_filepath, e
                );
            }
        };

        // save new file
        let new_file = "concat_with_struct.json";
        let new_file_path = target_level_dir.join(new_file);
        let new_content = serde_json::to_string_pretty(&to_questions).unwrap();
        write_file(new_file_path, &new_content);
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
