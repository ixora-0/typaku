// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data_file_io;
mod data_structs;
mod log4rs_config;
mod words_loader;

use std::iter::repeat_with;

use data_structs::AppData;
use log::{error, trace, warn};
use words_loader::load_most_common_200;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            init_log,
            load_data,
            gen_txt_from_mc200
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_log() {
    match log4rs_config::init_logger() {
        None => warn!("Already had logger set"),
        Some((_handler, Some(e))) => error!("{}", e),
        Some((_handler, None)) => {}
    }
    trace!("Initialized logger");
}

/// represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn load_data() -> Result<AppData, Error> {
    Ok(data_file_io::load_data()?) // this converts io::Error into Error thanks to the ? notation
}

#[tauri::command(rename_all = "snake_case")]
fn gen_txt_from_mc200(handle: tauri::AppHandle, num_words: usize) -> Result<String, Error> {
    let word_bank = load_most_common_200(handle)?;
    Ok(generate_text_from_bank(&word_bank, num_words))
}

fn generate_text_from_bank(word_bank: &[String], num_words: usize) -> String {
    repeat_with(|| word_bank[fastrand::usize(..word_bank.len())].clone())
        .take(num_words)
        .collect::<Vec<_>>()
        .join(" ")
}
