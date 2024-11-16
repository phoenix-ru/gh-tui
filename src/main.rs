use std::process::Command;

use github::file_with_patch::output_file_itself;
use pr_files::list_pr_files;
use structs::AppError;

mod github;
mod pr_files;
mod structs;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let auth_header = get_auth_header()?;

    // Get current repo
    let Some(name_with_owner) = execute_command(
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "-t",
            "{{.nameWithOwner}}",
        ],
    ) else {
        return Err(AppError::NotAGithubRepo);
    };

    println!("{}, {}", auth_header, name_with_owner);

    // TEST
    let name_with_owner = "phoenix-ru/fervid";
    let pull_number = 36;

    let pull_files = list_pr_files(&auth_header, &name_with_owner, pull_number).await?;
    println!("{:#?}", &pull_files);

    if let Some(first_file) = pull_files.first() {
        output_file_itself(&auth_header, &name_with_owner, first_file).await?;
    }

    Ok(())
}

fn get_auth_header() -> Result<String, AppError> {
    // Get token
    let Some(gh_token) = execute_command("gh", &["auth", "token"]) else {
        return Err(AppError::NoToken);
    };

    Ok(format!("Bearer {}", gh_token.trim()))
}

fn execute_command(cmd: &str, args: &[&str]) -> Option<String> {
    let cmd_output = Command::new(cmd).args(args).output().ok();

    let Some(cmd_output) = cmd_output else {
        return None;
    };

    let Some(0) = cmd_output.status.code() else {
        return None;
    };

    String::from_utf8(cmd_output.stdout).ok()
}
