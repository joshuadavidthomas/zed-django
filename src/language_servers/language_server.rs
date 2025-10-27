use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

pub trait LanguageServer {
    const SERVER_ID: &str;

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command>;
}
