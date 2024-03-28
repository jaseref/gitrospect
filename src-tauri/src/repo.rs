use git2::BranchType;
use serde::{Deserialize, Serialize};

use crate::{
    abstraction::{Branch, TryFromGit},
    CommandResult,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Repo {
    pub branches: Vec<Branch>,
}

impl Repo {
    pub fn new(path: &str) -> CommandResult<Self> {
        let repo = git2::Repository::open(path)?;

        // TODO: Consider remote branches later
        let mut branches = Vec::new();
        let raw_branches = repo
            .branches(Some(BranchType::Local))?
            .collect::<Result<Vec<_>, _>>()?;
        for (b, _) in raw_branches {
            println!("\n{}\n", b.name()?.unwrap());
            let branch = Branch::try_from_git(&b, &repo)?;
            // println!("\n{}\n", branch.name);
            // Start at initial commit for a branch
            // let initial_commit = branch.find_initial_commit(&repo)?;
            // println!("{}", initial_commit.message().unwrap_or(""));

            // let root_tree = b.get().peel_to_tree()?;
            // let root_tree = Tree::convert(root_tree, &repo)?;

            branches.push(branch);
        }
        let r = Repo { branches };
        Ok(r)
    }
}

/*
    1.) Start at Initial commit (no parent) and load all diffs
        In this case, the entire Tree is loaded since it has no parent.
    2.) Find commits starting from Initial. Compare commits to parents and load diffs into a Tree.
*/
