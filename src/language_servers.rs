mod django_language_server;
mod django_template_lsp;

pub use django_language_server::DjangoLanguageServer;
pub use django_template_lsp::DjangoTemplateLsp;
use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::Worktree;
use zed_extension_api::{self as zed};

pub trait LanguageServer {
    const EXECUTABLE_NAME: &str;
    const GITHUB_REPO: &str;
    const PACKAGE_NAME: &str;
    const SERVER_ID: &str;

    fn command(&self) -> Option<&zed::Command>;
    fn set_command(&mut self, command: zed::Command);
    fn get_command_fallback(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command>;

    fn get_command_args(&self) -> Vec<String> {
        Vec::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if let Some(command) = self.command() {
            return Ok(command.clone());
        }

        if let Some(binary_path) = worktree.which(Self::EXECUTABLE_NAME) {
            let command = zed::Command {
                command: binary_path,
                args: self.get_command_args(),
                env: Vec::default(),
            };
            self.set_command(command.clone());
            return Ok(command);
        }

        let command = self.get_command_fallback(language_server_id, worktree)?;
        self.set_command(command.clone());
        Ok(command)
    }

    fn language_server_initialization_options(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default(),
        ))
    }
}
