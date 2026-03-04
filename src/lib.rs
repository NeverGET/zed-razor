use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

const RZLS_GITHUB_REPO: &str = "Crashdummyy/rzls";

struct RazorExtension {
    cached_binary_path: Option<String>,
}

impl RazorExtension {
    /// Find or download the rzls binary.
    /// Returns (path, needs_dotnet) — if needs_dotnet is true, run via `dotnet exec <path>`.
    fn ensure_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<(String, bool)> {
        // 1. Check if rzls is already on PATH
        if let Some(path) = worktree.which("rzls") {
            return Ok((path, false));
        }

        // 2. Check cached path
        if let Some(ref path) = self.cached_binary_path {
            if fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                return Ok((path.clone(), path.ends_with(".dll")));
            }
        }

        // 3. Query latest release from GitHub
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            RZLS_GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true, // rzls releases are tagged as pre-release
            },
        )?;

        let (os, arch) = zed::current_platform();
        let platform = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "osx-arm64",
            (zed::Os::Mac, zed::Architecture::X8664) => "osx-x64",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "linux-arm64",
            (zed::Os::Linux, zed::Architecture::X8664) => "linux-x64",
            (zed::Os::Windows, zed::Architecture::Aarch64) => "win-arm64",
            (zed::Os::Windows, zed::Architecture::X8664) => "win-x64",
            _ => return Err(format!("unsupported platform: {os:?}/{arch:?}")),
        };

        let asset_name = format!("rzls.{platform}.zip");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no rzls asset found for {platform}"))?;

        let version_dir = format!("rzls-{}", release.version);

        // Binary names to check (self-contained exe first, then framework-dependent dll)
        let exe_name = if matches!(os, zed::Os::Windows) {
            "rzls.exe"
        } else {
            "rzls"
        };
        let exe_path = format!("{version_dir}/{exe_name}");
        let dll_path = format!("{version_dir}/rzls.dll");

        // 4. Download if not already present
        if !is_file(&exe_path) && !is_file(&dll_path) {
            // Clean up old versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with("rzls-") && name.as_ref() != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, zed::DownloadedFileType::Zip)
                .map_err(|e| format!("failed to download rzls: {e}"))?;

            // Make executable (no-op on Windows but needed for Linux/macOS)
            zed::make_file_executable(&exe_path).ok();
        }

        // 5. Determine which binary to use
        if is_file(&exe_path) {
            self.cached_binary_path = Some(exe_path.clone());
            Ok((exe_path, false))
        } else if is_file(&dll_path) {
            self.cached_binary_path = Some(dll_path.clone());
            Ok((dll_path, true))
        } else {
            // Search one level deeper in case the zip has a subdirectory
            if let Some(found) = find_binary_recursive(&version_dir, exe_name) {
                self.cached_binary_path = Some(found.clone());
                return Ok((found, false));
            }
            if let Some(found) = find_binary_recursive(&version_dir, "rzls.dll") {
                self.cached_binary_path = Some(found.clone());
                return Ok((found, true));
            }
            Err("rzls binary not found after download. Please report this issue.".into())
        }
    }
}

/// Check if a path is a file.
fn is_file(path: &str) -> bool {
    fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}

/// Search one level of subdirectories for a binary.
fn find_binary_recursive(dir: &str, binary_name: &str) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            let candidate = format!("{}/{}", entry.path().display(), binary_name);
            if is_file(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

impl zed::Extension for RazorExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let (binary_path, needs_dotnet) = self.ensure_binary(language_server_id, worktree)?;

        let mut args = Vec::new();

        let command = if needs_dotnet {
            let dotnet = worktree
                .which("dotnet")
                .ok_or("rzls requires .NET runtime but `dotnet` was not found on PATH.\nInstall the .NET SDK from https://dotnet.microsoft.com/download")?;
            args.push("exec".into());
            args.push(binary_path);
            dotnet
        } else {
            binary_path
        };

        args.extend([
            "--logLevel".into(),
            "Information".into(),
            "--telemetryLevel".into(),
            "off".into(),
        ]);

        Ok(zed::Command {
            command,
            args,
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "hostInfo": "zed",
            "razorAcceptedCapabilities": {
                "documentColor": false,
                "foldingRange": true,
                "inlineCompletion": false
            }
        })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "razor": {
                "format": { "enable": true },
                "completion": { "commitElementsWithSpace": false }
            }
        })))
    }
}

zed::register_extension!(RazorExtension);
