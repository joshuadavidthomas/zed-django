mod django_language_server;
mod django_template_lsp;

pub use django_language_server::DjangoLanguageServer;
pub use django_template_lsp::DjangoTemplateLsp;
use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::Worktree;
use zed_extension_api::{
    self as zed,
};

pub trait LanguageServer {
    const SERVER_ID: &str;

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command>;
}
