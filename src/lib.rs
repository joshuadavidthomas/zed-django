mod language_servers;

use std::collections::HashMap;

use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::Worktree;
use zed_extension_api::{
    self as zed,
};

use crate::language_servers::LanguageServerInstance;

struct DjangoExtension {
    language_servers: HashMap<String, LanguageServerInstance>,
}

impl zed::Extension for DjangoExtension {
    fn new() -> Self {
        Self {
            language_servers: HashMap::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let id = language_server_id.as_ref();

        if !self.language_servers.contains_key(id) {
            let server = LanguageServerInstance::from_id(id)?;
            self.language_servers.insert(id.to_string(), server);
        }

        let server = self.language_servers.get_mut(id).unwrap();
        server.language_server_command(language_server_id, worktree)
    }
}

zed::register_extension!(DjangoExtension);
