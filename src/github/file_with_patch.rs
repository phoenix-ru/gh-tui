use std::ffi::OsStr;
use std::path::Path;

use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use syntect::dumps::from_binary;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::pr_files::GitHubPRFilesResponseItem;
use crate::structs::AppError;

pub async fn output_file_itself(
    auth_header: &str,
    owner_and_repo: &str,
    def: &GitHubPRFilesResponseItem,
) -> Result<(), AppError> {
    let Some(file_ext) = Path::new(&def.filename).extension().and_then(OsStr::to_str) else {
        return Err(AppError::UnknownFileExtension);
    };

    let blob_url = format!(
        "https://api.github.com/repos/{owner_and_repo}/git/blobs/{}",
        &def.sha
    );
    dbg!(&blob_url);

    let client = reqwest::Client::new();
    let request = client
        .get(blob_url)
        .header(ACCEPT, "application/vnd.github.raw+json")
        .header(AUTHORIZATION, auth_header)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, "cli")
        .send();

    // Get the file contents
    let Ok(raw_response) = request.await else {
        return Err(AppError::HttpError);
    };
    let Ok(text_contents) = raw_response.text().await else {
        return Err(AppError::MalformedApiResponse);
    };

    // Load these once at the start of your program
    let Ok(packed_syntaxes) = std::fs::read("syntaxes.packdump") else {
        return Err(AppError::SyntaxDumpNotFound);
    };
    let syntax_set: SyntaxSet = from_binary(&packed_syntaxes);

    let ts = ThemeSet::load_defaults();

    dbg!(file_ext);

    let syntax = syntax_set.find_syntax_by_extension(file_ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(&text_contents) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &syntax_set).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }

    Ok(())
}
