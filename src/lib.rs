mod djls;

use std::fs;
use std::path::PathBuf;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct DjangoExtension {
    djls_path: Option<PathBuf>,
}

impl zed::Extension for DjangoExtension {
    fn new() -> Self {
        Self { djls_path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let needs_refresh = self
            .djls_path
            .as_ref()
            .is_none_or(|path| fs::metadata(path).is_err());

        if needs_refresh {
            let path = djls::get_or_install_djls(language_server_id, worktree)?;
            self.djls_path = Some(path);
        }

        let binary_path = self
            .djls_path
            .as_ref()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: binary_path,
            args: vec!["serve".to_string()],
            env: Vec::default(),
        })
    }
}

zed::register_extension!(DjangoExtension);
