use std::path::PathBuf;

use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::{
    self as zed,
};

const GITHUB_REPO: &str = "joshuadavidthomas/django-language-server";
const BINARY_NAME: &str = "djls";
const PACKAGE_NAME: &str = "django-language-server";

enum Os {
    Darwin,
    Linux,
    Windows,
}

enum Arch {
    Arm64,
    X64,
}

struct ReleaseArtifact {
    os: Os,
    arch: Arch,
    version: String,
}

impl ReleaseArtifact {
    fn new(os: zed::Os, arch: zed::Architecture, version: String) -> Self {
        let os = match os {
            zed::Os::Mac => Os::Darwin,
            zed::Os::Linux => Os::Linux,
            zed::Os::Windows => Os::Windows,
        };

        let arch = match arch {
            zed::Architecture::Aarch64 => Arch::Arm64,
            zed::Architecture::X8664 => Arch::X64,
            zed::Architecture::X86 => panic!("x86 architecture is not supported"),
        };

        Self { os, arch, version }
    }

    fn os(&self) -> &'static str {
        match self.os {
            Os::Darwin => "darwin",
            Os::Linux => "linux",
            Os::Windows => "windows",
        }
    }

    fn arch(&self) -> &'static str {
        match self.arch {
            Arch::Arm64 => "arm64",
            Arch::X64 => "x64",
        }
    }

    fn archive_extension(&self) -> &'static str {
        match self.os {
            Os::Windows => "zip",
            _ => "tar.gz",
        }
    }

    fn download_file_type(&self) -> zed::DownloadedFileType {
        match self.os {
            Os::Windows => zed::DownloadedFileType::Zip,
            _ => zed::DownloadedFileType::GzipTar,
        }
    }

    fn base_name(&self) -> String {
        format!(
            "{}-v{}-{}-{}",
            PACKAGE_NAME,
            self.version,
            self.os(),
            self.arch()
        )
    }

    fn asset_name(&self) -> String {
        format!("{}.{}", self.base_name(), self.archive_extension())
    }

    fn binary_path(&self) -> String {
        format!("{}/{}", self.base_name(), BINARY_NAME)
    }
}

pub fn get_or_install_djls(
    language_server_id: &LanguageServerId,
    worktree: &zed::Worktree,
) -> Result<PathBuf> {
    if let Some(path) = worktree.which(BINARY_NAME.as_ref()) {
        return Ok(PathBuf::from(path));
    }

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );

    let release = zed::latest_github_release(
        GITHUB_REPO,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let (platform, arch) = zed::current_platform();
    let artifact = ReleaseArtifact::new(platform, arch, release.version);

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == artifact.asset_name())
        .ok_or_else(|| format!("No asset found matching {}", artifact.asset_name()))?;

    let binary_path = artifact.binary_path();

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    zed::download_file(
        &asset.download_url,
        &binary_path,
        artifact.download_file_type(),
    )
    .map_err(|e| format!("Failed to download djls: {e}"))?;

    zed::make_file_executable(&binary_path)?;

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::None,
    );

    Ok(PathBuf::from(binary_path))
}
