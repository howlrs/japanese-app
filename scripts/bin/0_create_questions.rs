use std::path::PathBuf;
use std::{env, time::Instant};

use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{Content, Model, Part, Role, request::Request},
};
use log::{error, info, warn};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let start = Instant::now();

    let prompt_dir = "prompts";
    let target_levels = vec!["n3", "n2"];
    let count = 30;
    let max_retries = 3;

    let mut total_success = 0u32;
    let mut total_fail = 0u32;
    let mut total_invalid_json = 0u32;

    for target_level in target_levels {
        let (first_prompt, base_info, detail_prompt) = {
            let current_dir = env::current_dir().expect("カレントディレクトリの取得に失敗");

            let create_filepath = current_dir
                .join(prompt_dir)
                .join("create-question_to_json.md");
            let prepare_filepath = current_dir.join(prompt_dir).join("base-info.md");
            let detail_filepath = current_dir
                .join(prompt_dir)
                .join(target_level)
                .join("ja-question.md");

            for path in [&create_filepath, &prepare_filepath, &detail_filepath] {
                if !path.exists() {
                    error!("必須ファイルが見つかりません: {:?}", path);
                    std::process::exit(1);
                }
            }

            (
                replace_level(read_prompt_file(&create_filepath).as_str(), target_level),
                read_prompt_file(&prepare_filepath),
                read_prompt_file(&detail_filepath),
            )
        };

        let (key, model) = get_key_and_model();
        info!(
            "model: {}, level: {}, count: {}",
            model, target_level, count
        );

        let prompt = format!("{first_prompt}\n\n{base_info}\n\n{detail_prompt}");

        let mut level_success = 0u32;
        let mut level_fail = 0u32;
        let mut level_invalid_json = 0u32;

        for i in 0..count {
            let mut retry_count = 0;
            let result = loop {
                let response =
                    request_gemini_api(key.clone(), model.clone(), prompt.as_str()).await;

                match response {
                    Ok(r) => break Some(r),
                    Err(e) => {
                        retry_count += 1;
                        if retry_count >= max_retries {
                            error!(
                                "[{}/{}] {}回リトライ後も失敗: {}",
                                target_level, i, max_retries, e
                            );
                            break None;
                        }
                        let wait_secs = 60 * retry_count;
                        warn!(
                            "[{}/{}] APIエラー (リトライ {}/{}): {} - {}秒後に再試行",
                            target_level, i, retry_count, max_retries, e, wait_secs
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(wait_secs)).await;
                    }
                }
            };

            match result {
                Some(text) => {
                    // JSON構造の基本検証
                    let cleaned = text
                        .trim()
                        .trim_start_matches("```json")
                        .trim_start_matches("```")
                        .trim_end_matches("```")
                        .trim();

                    if serde_json::from_str::<serde_json::Value>(cleaned).is_ok() {
                        let now_timestamp = chrono::Utc::now();
                        let current_dir = env::current_dir().expect("カレントディレクトリの取得に失敗");
                        let save_filepath = current_dir
                            .join("output")
                            .join("questions")
                            .join(target_level)
                            .join(format!("{}.json", now_timestamp.timestamp()));
                        str_to_save_file(cleaned, &save_filepath);
                        level_success += 1;
                        info!(
                            "[{}/{}] 成功 ({}文字), Elapsed: {:?}",
                            target_level,
                            i,
                            cleaned.len(),
                            start.elapsed()
                        );
                    } else {
                        level_invalid_json += 1;
                        warn!(
                            "[{}/{}] 無効なJSON - スキップ (先頭100文字: {})",
                            target_level,
                            i,
                            cleaned.chars().take(100).collect::<String>()
                        );
                    }
                }
                None => {
                    level_fail += 1;
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        }

        info!(
            "=== {} 完了: 成功={}, 失敗={}, 無効JSON={}, Elapsed: {:?} ===",
            target_level, level_success, level_fail, level_invalid_json, start.elapsed()
        );

        total_success += level_success;
        total_fail += level_fail;
        total_invalid_json += level_invalid_json;
    }

    info!(
        "=== 全体完了: 成功={}, 失敗={}, 無効JSON={}, 総Elapsed: {:?} ===",
        total_success, total_fail, total_invalid_json, start.elapsed()
    );
}

fn read_prompt_file(abs_filename: &PathBuf) -> String {
    std::fs::read_to_string(abs_filename).unwrap_or_else(|e| {
        error!("ファイルの読み込みに失敗しました: {:?} - {}", abs_filename, e);
        std::process::exit(1);
    })
}

fn get_key_and_model() -> (String, String) {
    let key = match env::var("GOOGLE_GEMINI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            panic!("GOOGLE_GEMINI_API_KEY not set");
        }
    };
    let models = match env::var("GEMINI_MODELS") {
        Ok(m) => m,
        Err(_) => {
            panic!("GEMINI_MODELS not set");
        }
    };
    let models = models.split(",").collect::<Vec<&str>>();
    if models.len() != 2 {
        panic!("GEMINI_MODELS must be 2 models");
    }
    let model = models[0];

    (key, model.to_string())
}

// gemini api request
async fn request_gemini_api(key: String, model: String, text: &str) -> Result<String, String> {
    let client = Client::new_from_model(Model::Custom(model.to_string()), key.to_string());
    let request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(text.to_string()),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
        system_instruction: None,
    };

    let response = match client.post(60, &request).await {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    };

    let into_rest = match response.rest() {
        Some(r) => r,
        None => return Err("APIレスポンスのパースに失敗".to_string()),
    };
    match into_rest
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .and_then(|p| p.text.as_ref())
    {
        Some(t) => Ok(t.to_string()),
        None => Err("APIレスポンスにテキストが含まれていません".to_string()),
    }
}

fn str_to_save_file(text: &str, filename: &PathBuf) {
    if let Some(parent) = filename.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            error!("ディレクトリの作成に失敗: {:?} - {}", parent, e);
            std::process::exit(1);
        });
    }
    std::fs::write(filename, text).unwrap_or_else(|e| {
        error!("ファイルの書き込みに失敗: {:?} - {}", filename, e);
        std::process::exit(1);
    });
}

fn replace_level(text: &str, target_level: &str) -> String {
    let replace_text = target_level.to_string().to_uppercase();
    text.replace("**LEVEL**", &format!("**{}**", replace_text))
}
