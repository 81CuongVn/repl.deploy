use {
    anyhow::{bail, Result},
    constants::{GIT_FETCH_FAILED_ERROR, GIT_RESET_FAILED_ERROR},
    std::process::Command,
};

pub fn update_git_from_remote() -> Result<()> {
    let git_fetch = Command::new("git").args(&["fetch", "--all"]).output();

    if git_fetch.is_err() {
        bail!(GIT_FETCH_FAILED_ERROR);
    }

    let git_reset = Command::new("git")
        .args(&["reset", "--hard", "origin/main"])
        .output();

    if git_reset.is_err() {
        bail!(GIT_RESET_FAILED_ERROR);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{env, fs, path::Path},
    };

    #[test]
    fn try_reset() {
        prepare_repos();
        assert!(env::set_current_dir(Path::new("./test_repo2")).is_ok());
        update_git_from_remote().unwrap();
        let file_contents = fs::read_to_string("./temp").unwrap();
        assert_eq!(file_contents, "hi")
    }

    fn prepare_repos() {
        Command::new("./test_prep.sh").output().unwrap();
    }
}
