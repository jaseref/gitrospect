use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Repo {
    pub branches: Vec<Branch>,
}

// TODO: Store BranchType?
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    // points to the top level commit for a branch
    pub tip_commit_id: String,
}

// // https://github.com/libgit2/libgit2/blob/main/include/git2/types.h#L182
// #[derive(Debug)]
// struct Signature {
//     name: String,
//     email: String,
//     // TODO: https://github.com/libgit2/libgit2/blob/main/include/git2/types.h#L175
//     time: String,
// }

// // https://github.com/libgit2/libgit2/blob/main/src/libgit2/commit.h#L19
// #[derive(Debug)]
// struct Commit {
//     root_tree_id: String,
//     parent_ids: Option<Vec<String>>,
//     author: Signature,
//     committer: Signature,
//     // NOTE: Summary and body are seperated by a newline.
//     message: String,
// }

// // Needs more research: File data in memory vs not
// #[derive(Debug)]
// struct Blob {
//     id: String,
//     data: Vec<u8>,
// }
