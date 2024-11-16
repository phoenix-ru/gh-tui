#[derive(Debug)]
pub enum AppError {
    NoToken,
    NotAGithubRepo,
    HttpError,
    MalformedApiResponse,
    UnknownFileExtension,
    SyntaxDumpNotFound,
}
