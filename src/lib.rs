use std::path::Path;

use zed_extension_api::{self as zed, Result, serde_json, settings::LspSettings};

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct McshaderExtension {
    cached_binary_path: Option<Box<str>>,
}

impl McshaderExtension {
    const LANGUAGE_SERVER_ID: &'static str = "mcshader";

    fn language_server_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        Ok(
            if let Some(cached_binary_path) = &self.cached_binary_path
                && Path::new(cached_binary_path.as_ref()).exists()
            {
                (**cached_binary_path).to_owned()
            } else {
                let cached_binary_path = worktree
                    .which(Self::LANGUAGE_SERVER_ID)
                    .ok_or(String::from(
                        "`mcshader` language server binary not found in environment `PATH`",
                    ))?
                    .into_boxed_str();

                // TODO: Download the server or something if we can't find it.

                self.cached_binary_path = Some(cached_binary_path.clone());

                cached_binary_path.into_string()
            },
        )
    }
}

impl zed::Extension for McshaderExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(worktree)?,
            args: Default::default(),
            env: Default::default(),
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
