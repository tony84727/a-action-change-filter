use std::{
    process,
    io,
};
use std::path::{PathBuf};
use std::process::Stdio;

#[derive(Debug)]
pub enum CommandError {
    CannotSpawn(io::Error),
    NoStdOut,
    CannotReadStdOut(io::Error),
    CommandFail(io::Error),
    CommandExitCode(Option<i32>),
}

fn git_command(args: Vec<&str>, working_dir: Option<&PathBuf>) -> Result<String, CommandError> {
    let mut command = process::Command::new("git");
        command.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    }
    let child = command.spawn().map_err(|x| CommandError::CannotSpawn(x))?;
    let output = child.wait_with_output().map_err(|x| CommandError::CommandFail(x))?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else {
        Err(CommandError::CommandExitCode(output.status.code()))
    }
}

pub fn get_changed_files(base_sha: &str, head_sha: &str, working_dir: Option<&PathBuf>) -> Result<Vec<String>, CommandError> {
    let command_output= git_command(vec!["diff", format!("{}..{}", base_sha, head_sha).as_str(), "--name-only"], working_dir)?;
    Ok(command_output.split("\n").map(|x| String::from(x)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_get_changed_files() {
        let test_zone = env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        fs::create_dir(&test_zone).unwrap();
        git_command(vec!["init"], Some(&test_zone)).unwrap();
        let testing_repo = test_zone;
        let put_and_commit = |path: &str, file_content: &str, commit_msg: &str| {
            let file_path = testing_repo.join(path);
            fs::write(&file_path, file_content);
            git_command(vec!["add", &file_path.to_str().unwrap()], Some(&testing_repo)).unwrap();
            git_command(vec!["commit", "-m", commit_msg], Some(&testing_repo)).unwrap();
            let output = git_command(vec!["rev-parse", "HEAD"], Some(&testing_repo)).unwrap();
            println!("created commit: {}", output);
            output
        };
        let first_commit = put_and_commit("first","first", "first commit");
        let second_commit = put_and_commit("second", "second", "second commit");
        let third_commit = put_and_commit("third","thrid", "third commit");
        assert_eq!(vec!["second"],get_changed_files(first_commit.as_str(), second_commit.as_str(), Some(&testing_repo)).unwrap());
        assert_eq!(vec!["third"],get_changed_files(second_commit.as_str(), third_commit.as_str(), Some(&testing_repo)).unwrap());
        assert_eq!(vec!["second", "third"], get_changed_files(first_commit.as_str(), third_commit.as_str(), Some(&testing_repo)).unwrap());
    }
}