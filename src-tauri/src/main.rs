// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anki_integration::invoke;
use serde_json::{json, Value};
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        ChatCompletionResponseFormat, ChatCompletionResponseFormatType
    }, Client
};
use serde::Deserialize;
use sync_anki_process::SyncAnkiTask;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Manager, Window};
use tauri::WindowEvent;
use std::sync::mpsc;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct Questions {
    questions: Vec<QA>
} 

#[derive(Debug, Deserialize)]
struct QA {
    question: String,
    answer: String,
}

#[tauri::command]
fn parse_pdf(path: &str) -> String {
    let bytes = std::fs::read(path).unwrap();
    let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    let lines: Vec<&str> = text.lines().filter(|line| !line.trim().is_empty()).collect();

    let mut result = String::new();

    for line in lines {
        result += line;
        result += "\n";
    }

    return result;
}

#[tauri::command]
async fn generate_question_answers(result: String, deck_name: String) {
    let client = Client::new();

    let mut string_prompt = String::new();

    string_prompt += "
    Create questions with their question and answer pairs based on the notes provided. Please create confusing questions that are useful for studying.
    
    Notes: ";
    string_prompt += &result;
    string_prompt += "
        
    Please only provide a RFC8259 compliant JSON response following this format without deviation.
    {
        \"questions\": [
            { 
                \"question\": \"\",
                \"answer\": \"\",
            },
        ]
    }
    The JSON response:
    ";

    let response_format = ChatCompletionResponseFormat {
        r#type: ChatCompletionResponseFormatType::JsonObject,
    };

    // single prompt
    // note: it is important to specify the response format as JSON, otherwise, the API will sometimes include ```json and other times leaving it out
    // leaving another level of parsing of the response necessary
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1024_u16)
        .model("gpt-3.5-turbo")
        .response_format(response_format) // specifies that the response should be a JSON object format
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a teacher creating practice exam questions.")
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(string_prompt)
                .build()
                .unwrap()
                .into(),
        ])
        .build()
        .expect("Parsed");

    let response = client.chat().create(request).await;

    if let Some(first_choice) = response.unwrap().choices.get(0) {
        if let Some(content) = &first_choice.message.content {
            let parsed: Result<Questions, _> = serde_json::from_str(content);

            match parsed {
                Ok(list) => {
                    for item in list.questions {
                        let note = json!({
                            "note": {
                                "deckName": deck_name,
                                "modelName": "Basic",
                                "fields": {
                                    "Front": item.question,
                                    "Back": item.answer
                                },
                                "options": {
                                    "allowDuplicate": false,
                                    "duplicateScope": "deck",
                                    "duplicateScopeOptions": {
                                        "deckName": "Hello",
                                        "checkChildren": false,
                                        "checkAllModels": false
                                    }
                                }
                            }
                        });

                        let map: serde_json::Map<String, Value> = note.as_object().unwrap().clone();
                        println!("{:?}", map);
                        // add to the Anki deck
                        let response = invoke("addNote", Some(map)).await;
                        println!("{:?}", response);
                        println!("{:?}", item);
                    }
                }
                Err(err) => {
                    println!("{:?}", content);
                    eprintln!("Error parsing JSON data: {}", err);
                }
            }
        } else {
            eprintln!("Content is empty for the first choice");
        }
    } else {
        eprintln!("No choices available in response from ChatGPT");
    }
}

#[tauri::command]
async fn score_answer(question: String, user_answer: String, ai_answer: String) -> String {
    let client = Client::new();

    let role = "You are a teacher who is grading a students answer";
    let mut string_prompt = String::new();

    string_prompt += "The student answer is: "; 
    string_prompt += &user_answer;
    
    string_prompt += "The question is"; 
    string_prompt += &question;

    string_prompt += "The answer key is";
    string_prompt += &ai_answer;
    
    string_prompt += "Can you give the answer a score out of 5 with the reasoning?";

    // single prompt
    // note: it is important to specify the response format as JSON, otherwise, the API will sometimes include ```json and other times leaving it out
    // leaving another level of parsing of the response necessary
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1024_u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(role)
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(string_prompt)
                .build()
                .unwrap()
                .into(),
        ])
        .build()
        .expect("Parsed");

    let response = client.chat().create(request).await;

    match response {
        Ok(response) => {
            if let Some(first_choice) = response.choices.get(0) {
                if let Some(content) = &first_choice.message.content {
                    return content.into();
                } else {
                    return "Could not grade question: No content found".into();
                }
            } else {
                return "Could not grade question: No choices found".into();
            }
        }
        Err(_) => {
            return "Could not grade question. Failed to get response from Open AI API. Check your environment variable is set with your key".into();
        }
    }
}

fn init_background_process(window: Window, tx: mpsc::Sender<String>) -> Arc<AtomicBool> {
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    // Spawn the background thread
    let anki_task = SyncAnkiTask::new();
    let _background_anki_thread = anki_task.spawn_background_thread(running.clone(), tx);

    let r = running.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            r.store(false, Ordering::SeqCst);
        }
    });

    return running;
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String
}

#[tauri::command]
fn save_api_key(key: String) {
    let config_dir: std::path::PathBuf = tauri::api::path::config_dir().ok_or("Unable to get config directory").unwrap();
    let config_file_path = config_dir.join("my_app").join("config.json");
  
    fs::create_dir_all(config_file_path.parent().unwrap())
      .map_err(|err| format!("Failed to create config directory: {}", err)).unwrap();
  
    // the message is the API key in this case
    let config_content = serde_json::json!({ "api_key": key });
  
    fs::write(&config_file_path, config_content.to_string())
      .map_err(|err| format!("Failed to write config file: {}", err)).unwrap();
  
    // Optionally set it in the environment for the current session
    // In this case, the 'message' is the API key
    env::set_var("OPENAI_API_KEY", key);
}


#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let window = app.get_window("main").expect("Could not get window");
            let (tx, rx) = mpsc::channel();

            let running: Arc<AtomicBool> = init_background_process(window, tx);

            let handle = app.handle();
            std::thread::spawn(move || {
                for received in rx {
                    let _ = handle.emit_all("new_card", received);
                }
            });

            let resume_clone = running.clone();
            app.listen_global("resume", move |_event| {
                resume_clone.store(true, Ordering::SeqCst);
            });

            let pause_clone = running.clone();
            app.listen_global("pause", move |_event| {
                pause_clone.store(false, Ordering::SeqCst);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![parse_pdf, generate_question_answers, score_answer, save_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
