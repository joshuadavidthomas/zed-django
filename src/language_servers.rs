mod django_language_server;
mod django_template_lsp;

use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

pub use django_language_server::DjangoLanguageServer;
pub use django_template_lsp::DjangoTemplateLsp;

pub trait LanguageServer {
    const SERVER_ID: &str;

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command>;
}
