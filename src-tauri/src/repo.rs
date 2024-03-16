use git2::BranchType;
use serde::{Deserialize, Serialize};

use crate::{abstraction::Branch, CommandResult};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Repo {
    pub branches: Vec<Branch>,
}

/// A builder struct which is used to build a `Repo`, a convient way if storing data from a
/// `git2::Repository`.
pub struct RepoBuilder {
    repo: git2::Repository,
    branches: Vec<Branch>,
}

impl RepoBuilder {
    pub fn new(path: &str) -> CommandResult<Self> {
        let repo = git2::Repository::open(path)?;

        Ok(Self { repo, branches: Vec::new() })
    }

    pub fn with_branches(mut self) -> CommandResult<Self> {
        // TODO: Consider remote branches later
        for b in self.repo.branches(Some(BranchType::Local))? {
            let b = b?;
            let branch_name = b.0.name()?.unwrap();
            println!("{branch_name} : {:?}", b.1);
            let tip_commit_ref = b.0.get().peel_to_commit()?;
            let tip_commit_id = tip_commit_ref.id();
            self.branches.push(Branch {
                name: String::from(branch_name),
                tip_commit_id: tip_commit_id.to_string(),
            });
        }

        Ok(self)
    }

    pub fn build(self) -> Repo {
        Repo {
            branches: self.branches,
        }
    }
}