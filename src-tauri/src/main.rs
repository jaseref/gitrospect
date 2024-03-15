// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use git2::{ObjectType, Repository};
use serde::{ser::Serializer, Serialize};
use abstraction::{Branch, Repo};
use thiserror::Error;

mod abstraction;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_git_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_git_data(path: String) -> CommandResult<Repo> {
    let repo = match Repository::open(path) {
        Ok(repo) => {
            println!("repo loaded");
            repo
        }
        Err(_) => {
            panic!("failed to load repo");
        }
    };
    let mut repo_state = Repo::default();

    let branches = repo.branches(None)?;
    for b in branches {
        let b = b?;
        let branch_name = b.0.name()?.unwrap();
        println!("{branch_name} : {:?}", b.1);
        let tip_commit_ref = b.0.get().peel(ObjectType::Commit)?;
        let tip_commit_id = tip_commit_ref.id();
        repo_state.branches.push(Branch {
            name: String::from(branch_name),
            tip_commit_id: tip_commit_id.to_string(),
        });
    }
    println!("{repo_state:?}");
    Ok(repo_state)
}
// println!("Branches: {branches:?}");

// for b in branches {
//     println!("------------------");
//     println!("{b}");
//     println!("------------------");

//     let mut r = repo.revwalk().unwrap();
//     r.push_ref(b.as_str()).unwrap();
//     r.set_sorting(Sort::TIME).unwrap();
//     for oid in r {
//         let oid = oid.unwrap();
//         let commit = repo.find_commit(oid).unwrap();

//         println!("Commit ID: {}", oid);
//         println!("Author: {}", commit.author().name().unwrap_or("Unknown"));
//         println!("Message: {}", commit.message().unwrap_or("No message"));
//     }
// }

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
        serializer.serialize_str(self.to_string().as_ref())
    }
}
