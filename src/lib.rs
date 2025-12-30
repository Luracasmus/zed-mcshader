use zed_extension_api::{self as zed, Result, serde_json, settings::LspSettings};

struct McshaderExtension;

impl McshaderExtension {
    const LANGUAGE_SERVER_ID: &'static str = "mcshader";

    fn language_server_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        worktree
            .which(Self::LANGUAGE_SERVER_ID)
            .ok_or(String::from("mcshader LSP not found"))

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
            env: vec![(String::from("RUST_BACKTRACE"), String::from("1"))],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>, String> {
        let settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or(serde_json::json!({
                "logLevel": "info",
                "extraExtension": ["inc"],
                "tempLint": false
            }));

        Ok(Some(serde_json::json!({
            Self::LANGUAGE_SERVER_ID: settings
        })))
    }
}

zed::register_extension!(McshaderExtension);
