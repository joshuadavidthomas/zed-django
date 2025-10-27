use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::Worktree;
use zed_extension_api::{self as zed};

use super::LanguageServer;

pub struct DjangoTemplateLsp {
    command: Option<zed::Command>,
}

impl DjangoTemplateLsp {
    pub fn new() -> Self {
        Self { command: None }
    }
}

impl LanguageServer for DjangoTemplateLsp {
    const EXECUTABLE_NAME: &str = "djlsp";
    const GITHUB_REPO: &str = "fourdigits/django-template-lsp";
    const PACKAGE_NAME: &str = "django-template-lsp";
    const SERVER_ID: &str = "django-template-lsp";

    fn get_command_fallback(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        if let Some(uvx_path) = worktree.which("uvx") {
            return Ok(zed::Command {
                command: uvx_path,
                args: vec![
                    "--from".to_string(),
                    Self::PACKAGE_NAME.to_string(),
                    Self::EXECUTABLE_NAME.to_string(),
                ],
                env: Vec::default(),
            });
        }

        Err(format!(
            "{} not found. See installation instructions: https://github.com/{}",
            Self::EXECUTABLE_NAME,
            Self::GITHUB_REPO,
        ))
    }

    fn command(&self) -> Option<&zed::Command> {
        self.command.as_ref()
    }

    fn set_command(&mut self, command: zed::Command) {
        self.command = Some(command);
    }
}
