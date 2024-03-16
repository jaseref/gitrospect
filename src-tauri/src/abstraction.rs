#![allow(unused)]
use serde::{Deserialize, Serialize};

/// A tree object represents a directory in the repository. It contains references to other tree 
/// objects (subdirectories) and blob objects (files).
pub struct Tree {
    pub id: String,
    pub entries: Vec<TreeEntry>,
}

pub struct TreeEntry {
    pub id: String,
    pub name: String,
    pub entry_type: EntryType,
}

pub enum EntryType {
    Blob,
    Tree,
}

// TODO: Store BranchType?
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    /// Points to the top level commit for a branch
    pub tip_commit_id: String,
}

// https://github.com/libgit2/libgit2/blob/main/src/libgit2/commit.h#L19
pub struct Commit {
    pub id: String,
    pub author: Signature,
    pub committer: Signature,
    /// Message describing commit. NOTE: Summary and body are seperated by a newline.
    pub message: String,
    /// Vector of parent commit ids.
    pub parent_ids: Vec<String>,
    /// The tree associated with the commit.
    pub tree_id: String,
    // timestamp: Something,
}

// TODO: Research storing file contents in memory vs not.
/// A blob object represents the content of a file in the repository. 
/// It stores the actual file data, without any metadata or file name information.
pub struct Blob {
    pub id: String,
    pub content: Vec<u8>,
}

// https://github.com/libgit2/libgit2/blob/main/include/git2/types.h#L182
#[derive(Debug)]
pub struct Signature {
    pub name: String,
    pub email: String,
}