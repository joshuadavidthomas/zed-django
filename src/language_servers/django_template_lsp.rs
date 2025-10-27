use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

use super::LanguageServer;

const BINARY_NAME: &str = "djlsp";
const PACKAGE_NAME: &str = "django-template-lsp";

pub struct DjangoTemplateLsp {
    cached_command: Option<zed::Command>,
}

impl DjangoTemplateLsp {
    pub fn new() -> Self {
        Self {
            cached_command: None,
        }
    }
}

impl LanguageServer for DjangoTemplateLsp {
    const SERVER_ID: &str = "django-template-lsp";

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        // Return cached command if available
        if let Some(ref command) = self.cached_command {
            return Ok(command.clone());
        }

        // Check if already installed locally
        if let Some(binary_path) = worktree.which(BINARY_NAME) {
            let command = zed::Command {
                command: binary_path,
                args: Vec::default(),
                env: Vec::default(),
            };
            self.cached_command = Some(command.clone());
            return Ok(command);
        }

        // Check if uv is available - use uvx (ephemeral, no install needed)
        if let Some(uvx_path) = worktree.which("uvx") {
            let command = zed::Command {
                command: uvx_path,
                args: vec![
                    "--from".to_string(),
                    PACKAGE_NAME.to_string(),
                    BINARY_NAME.to_string(),
                ],
                env: Vec::default(),
            };
            self.cached_command = Some(command.clone());
            return Ok(command);
        }

        // Neither djlsp nor uvx found - provide helpful error
        Err(format!(
            "{} not found. Install uv (recommended): https://docs.astral.sh/uv/getting-started/installation/ or manually install {}: pip install {}",
            BINARY_NAME, PACKAGE_NAME, PACKAGE_NAME
        ))
    }
}
