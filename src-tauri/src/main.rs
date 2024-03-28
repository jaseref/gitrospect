// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use repo::Repo;
use serde::{ser::Serializer, Serialize};
use thiserror::Error;

mod abstraction;
mod repo;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_repo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_repo(path: String) -> CommandResult<Repo> {
    let repo_state = Repo::new(&path)?;
    // println!("{repo_state:?}");
    Ok(repo_state)
}

// Fix for anyhow: https://github.com/tauri-apps/tauri/discussions/3913

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    GitError(#[from] git2::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        println!("{self}");
        serializer.serialize_str(self.to_string().as_ref())
    }
}
