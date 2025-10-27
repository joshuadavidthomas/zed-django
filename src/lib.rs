mod language_servers;

use language_servers::DjangoLanguageServer;
use language_servers::DjangoTemplateLsp;
use language_servers::LanguageServer;
use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::Worktree;
use zed_extension_api::{
    self as zed,
};

#[derive(Default)]
struct DjangoExtension {
    django_language_server: Option<DjangoLanguageServer>,
    django_template_lsp: Option<DjangoTemplateLsp>,
}

impl zed::Extension for DjangoExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            DjangoLanguageServer::SERVER_ID => {
                let server = self
                    .django_language_server
                    .get_or_insert_with(DjangoLanguageServer::new);
                server.language_server_command(language_server_id, worktree)
            }
            DjangoTemplateLsp::SERVER_ID => {
                let server = self
                    .django_template_lsp
                    .get_or_insert_with(DjangoTemplateLsp::new);
                server.language_server_command(language_server_id, worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match language_server_id.as_ref() {
            DjangoLanguageServer::SERVER_ID => {
                let server = self
                    .django_language_server
                    .get_or_insert_with(DjangoLanguageServer::new);
                server.language_server_initialization_options(language_server_id, worktree)
            }
            DjangoTemplateLsp::SERVER_ID => {
                let server = self
                    .django_template_lsp
                    .get_or_insert_with(DjangoTemplateLsp::new);
                server.language_server_initialization_options(language_server_id, worktree)
            }
            _ => Ok(Some(
                zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                    .ok()
                    .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                    .unwrap_or_default(),
            )),
        }
    }
}

zed::register_extension!(DjangoExtension);
