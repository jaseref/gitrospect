use std::vec;

use anyhow::Context;
use git2::ObjectType;
use serde::{Deserialize, Serialize};

pub trait TryFromGit<T> {
    type Error;
    fn try_from_git(value: T, repo: &git2::Repository) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// A tree object represents a directory in the repository. It contains references to other tree
/// objects (subdirectories) and blob objects (files).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tree {
    pub id: String,
    #[serde(skip)]
    pub entry_type: EntryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Tree>>,
}

impl TryFromGit<&git2::Tree<'_>> for Tree {
    type Error = anyhow::Error;

    fn try_from_git(t: &git2::Tree, repo: &git2::Repository) -> anyhow::Result<Self> {
        let mut root = Tree::default();

        Tree::walk_tree(t, repo, &mut root)?;

        Ok(root)
    }
}

impl Tree {
    fn walk_tree(
        node: &git2::Tree,
        repo: &git2::Repository,
        parent: &mut Tree,
    ) -> anyhow::Result<()> {
        for c in node.iter() {
            let mut new_tree = Tree {
                id: c.name().context("unknown name")?.to_string(),
                ..Default::default()
            };
            match c.kind().context("get object type")? {
                ObjectType::Tree => {
                    // TODO: Cannot use ? here..
                    let t = c
                        .to_object(repo)?
                        .into_tree()
                        .expect("should convert object into tree");
                    Tree::walk_tree(&t, repo, &mut new_tree)?;
                }
                ObjectType::Blob => {
                    let b = c.to_object(repo).unwrap().into_blob().unwrap();
                    new_tree.entry_type = EntryType::Blob(b.content().to_vec());
                }
                _ => continue,
            }
            if let Some(e) = &mut parent.entries {
                e.push(new_tree)
            } else {
                parent.entries = Some(vec![new_tree])
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum EntryType {
    Blob(Vec<u8>),
    #[default]
    Tree,
}

// TODO: Store BranchType?
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    /// Points to the top level commit for a branch
    pub tip_commit_id: String,
    pub commits: Vec<Commit>,
}

impl Branch {
    pub fn load_commits(
        tip_commit_id: git2::Oid,
        repo: &git2::Repository,
    ) -> anyhow::Result<Vec<Commit>> {
        let mut commits: Vec<Commit> = vec![];
        let mut rw = repo.revwalk()?;
        rw.push(tip_commit_id)?;

        for id in rw {
            let id = id?;
            let commit = repo.find_commit(id)?;
            let commit_tree = commit.tree()?;
            match commit.parent_count() {
                0 => {
                    // Found initial commit, load entire tree
                    let mut converted_initial_commit = Commit::from(commit);
                    // // Test
                    // let n = &converted_initial_commit.message.clone().unwrap();
                    // println!("Initial Commit: {:?}", n);
                    // //
                    let diffs = Commit::load_diffs(None, commit_tree, repo)?;
                    converted_initial_commit.diff_tree = diffs;
                    commits.push(converted_initial_commit);
                }
                1 => {
                    // Found a child commit
                    let parent_tree = commit.parent(0)?.tree()?;
                    let mut converted_child_commit = Commit::from(commit);
                    // //Test
                    // let n = &converted_child_commit.message.clone().unwrap();
                    // let nid = converted_child_commit.id.clone();
                    // println!("Finding diff for: {:?} | {:?}", n, &nid[..7]);
                    // //
                    let diffs = Commit::load_diffs(Some(parent_tree), commit_tree, repo)?;
                    converted_child_commit.diff_tree = diffs;
                    commits.push(converted_child_commit);
                }
                _ => {
                    // https://stackoverflow.com/questions/40986518/how-to-git-show-the-diffs-for-a-merge-commit
                    // For now, we compare merge commits with the latest, single parent
                    // println!("Merge commit (ignoring)");
                }
            }
        }
        Ok(commits)
    }
}

impl TryFromGit<&git2::Branch<'_>> for Branch {
    type Error = anyhow::Error;

    fn try_from_git(b: &git2::Branch, repo: &git2::Repository) -> anyhow::Result<Self> {
        let name = b.name()?.context("get branch name")?.to_string();
        let tip_commit_id = b.get().peel_to_commit()?.id();
        let commits = Branch::load_commits(tip_commit_id, repo)?;
        Ok(Branch {
            name,
            tip_commit_id: tip_commit_id.to_string(),
            commits,
        })
    }
}

// https://github.com/libgit2/libgit2/blob/main/src/libgit2/commit.h#L19
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub author: Signature,
    pub committer: Signature,
    /// Message describing commit. NOTE: Summary and body are seperated by a newline.
    pub message: Option<String>,
    /// Vector of parent commit ids.
    pub parent_ids: Vec<String>,
    /// The tree associated with the commit.
    pub tree_id: String,
    pub diff_tree: Tree,
    // timestamp: Something,
}

impl Commit {
    // TODO: Not finished
    pub fn load_diffs(
        parent: Option<git2::Tree>,
        child: git2::Tree,
        repo: &git2::Repository,
    ) -> anyhow::Result<Tree> {
        match parent {
            Some(p) => {
                // println!("Parent: {} Child: {}", p.id(), child.id());
                let diff = repo.diff_tree_to_tree(Some(&p), Some(&child), None)?;
                let deltas = diff.deltas();
                for d in deltas {
                    let blob_id = d.new_file().id();
                    let _blob = repo.find_blob(blob_id)?;

                    // println!("{nf:?}");
                }
                Ok(Tree::default())
            }
            None => {
                let converted_tree = Tree::try_from_git(&child, repo)?;
                // println!("{}", serde_json::to_string_pretty(&converted_tree)?);
                Ok(converted_tree)
            }
        }
    }
}

impl From<git2::Commit<'_>> for Commit {
    fn from(c: git2::Commit) -> Self {
        Commit {
            id: c.id().to_string(),
            author: c.author().into(),
            committer: c.committer().into(),
            message: c.message().map(|m| m.to_string()),
            parent_ids: c.parents().map(|c| c.id().to_string()).collect(),
            tree_id: c.tree_id().to_string(),
            diff_tree: Tree::default(),
        }
    }
}

// https://github.com/libgit2/libgit2/blob/main/include/git2/types.h#L182
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

impl From<git2::Signature<'_>> for Signature {
    fn from(s: git2::Signature) -> Self {
        Signature {
            name: String::from(s.name().unwrap_or("Unknown")),
            email: String::from(s.email().unwrap_or("Unknown")),
        }
    }
}
