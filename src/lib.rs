use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

/// Locations where rzls might live, in priority order.
///
/// rzls ships with the official C# VS Code extension (ms-dotnettools.csharp).
/// It is NOT on PATH by default, so we search known directories.
const RZLS_BINARY_LOCATIONS: &[&str] = &[
    // VS Code extension (macOS / Linux)
    "~/.vscode/extensions",
    "~/.vscode-insiders/extensions",
    // VS Code extension (Windows)
    "~/AppData/Roaming/Code/extensions",
    // dotnet global tools (if someone installs a preview package)
    "~/.dotnet/tools",
    // Cursor editor
    "~/.cursor/extensions",
];

/// The rzls binary name (platform-agnostic; we'll try both).
const RZLS_BINARY_NAMES: &[&str] = &["rzls", "rzls.exe"];

struct RazorExtension {
    /// Cached path to the rzls binary once found.
    rzls_path: Option<String>,
}

impl RazorExtension {
    /// Walk well-known extension directories to find the rzls binary.
    ///
    /// Inside the VS Code C# extension the layout is roughly:
    ///   ms-dotnettools.csharp-X.Y.Z-PLATFORM/
    ///     .razor/
    ///       rzls          <- the binary
    fn find_rzls_binary(&self) -> Option<String> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()?;

        for base in RZLS_BINARY_LOCATIONS {
            let base = base.replace('~', &home);

            // Check direct location (e.g. ~/.dotnet/tools/rzls)
            for bin in RZLS_BINARY_NAMES {
                let direct = format!("{}/{}", base, bin);
                if fs::metadata(&direct).map(|m| m.is_file()).unwrap_or(false) {
                    return Some(direct);
                }
            }

            // Walk one level of subdirectories (VS Code extensions folder)
            if let Ok(entries) = fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    let name = entry.file_name();
                    let name = name.to_string_lossy();

                    // Only look inside ms-dotnettools.csharp-* directories
                    if !name.starts_with("ms-dotnettools.csharp") {
                        continue;
                    }

                    // Try <ext>/.razor/rzls  and  <ext>/rzls
                    let sub_dirs = [
                        entry_path.join(".razor"),
                        entry_path.clone(),
                        entry_path.join("bin"),
                    ];

                    for sub in &sub_dirs {
                        for bin in RZLS_BINARY_NAMES {
                            let candidate = sub.join(bin);
                            if candidate.is_file() {
                                return Some(candidate.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl zed::Extension for RazorExtension {
    fn new() -> Self {
        Self { rzls_path: None }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        // Resolve and cache the rzls path on first call.
        if self.rzls_path.is_none() {
            self.rzls_path = self.find_rzls_binary();
        }

        let rzls = self.rzls_path.clone().ok_or_else(|| {
            concat!(
                "rzls binary not found.\n\n",
                "Install the C# extension for VS Code, which bundles rzls:\n",
                "  https://marketplace.visualstudio.com/items?itemName=ms-dotnettools.csharp\n\n",
                "Or install the .NET preview tool:\n",
                "  dotnet tool install --global rzls\n\n",
                "Then restart Zed.",
            )
        })?;

        Ok(zed::Command {
            command: rzls,
            args: vec![
                "--logLevel".into(),
                "Information".into(),
                "--telemetryLevel".into(),
                "off".into(),
            ],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let opts = serde_json::json!({
            "hostInfo": "zed",
            "razorAcceptedCapabilities": {
                "documentColor": false,
                "foldingRange": true,
                "inlineCompletion": false
            }
        });

        Ok(Some(opts))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let config = serde_json::json!({
            "razor": {
                "format": {
                    "enable": true
                },
                "completion": {
                    "commitElementsWithSpace": false
                }
            }
        });

        Ok(Some(config))
    }
}

// Register the extension with Zed's WASM runtime.
zed::register_extension!(RazorExtension);
