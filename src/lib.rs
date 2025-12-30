use zed_extension_api::{self as zed, Result};

struct McshaderExtension;

impl McshaderExtension {
    const LANGUAGE_SERVER_ID: &'static str = "vscode-mcshader";

    fn language_server_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        worktree
            .which(Self::LANGUAGE_SERVER_ID)
            .ok_or(String::from("vscode-mcshader LSP not found"))

        // TODO: Download the server or something if it doesn't exist.
    }
}

impl zed::Extension for McshaderExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(McshaderExtension);
