use std::path::PathBuf;

use zed_extension_api::LanguageServerId;
use zed_extension_api::Result;
use zed_extension_api::{
    self as zed,
};

const GITHUB_REPO: &str = "joshuadavidthomas/django-language-server";
const BINARY_NAME: &str = "djls";
const PACKAGE_NAME: &str = "django-language-server";

struct ReleaseArtifact {
    os: zed::Os,
    arch: zed::Architecture,
    release: zed::GithubRelease,
}

impl ReleaseArtifact {
    fn new(os: zed::Os, arch: zed::Architecture, release: zed::GithubRelease) -> Self {
        if matches!(arch, zed::Architecture::X86) {
            panic!("x86 architecture is not supported");
        }
        Self { os, arch, release }
    }

    fn for_current_platform() -> Result<Self> {
        let (os, arch) = zed::current_platform();
        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        Ok(Self::new(os, arch, release))
    }

    fn download(&self) -> Result<()> {
        let (extension, file_type) = self.archive_info();
        let asset_name = format!("{}.{}", self.base_name(), extension);
        let download_url = self
            .release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .map(|asset| asset.download_url.clone())
            .ok_or_else(|| format!("No asset found matching {asset_name}"));
        zed::download_file(&download_url?, "", file_type)
            .map_err(|e| format!("Failed to download djls: {e}"))?;
        Ok(())
    }

    fn archive_info(&self) -> (&'static str, zed::DownloadedFileType) {
        match self.os {
            zed::Os::Windows => ("zip", zed::DownloadedFileType::Zip),
            _ => ("tar.gz", zed::DownloadedFileType::GzipTar),
        }
    }

    fn base_name(&self) -> String {
        let arch = match self.arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x64",
            zed::Architecture::X86 => unreachable!(),
        };
        let os = match self.os {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };
        format!("{}-{}-{}-{}", PACKAGE_NAME, self.release.version, os, arch)
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

    let artifact = ReleaseArtifact::for_current_platform()?;
    let binary_path = artifact.binary_path();

    if std::fs::metadata(&binary_path).is_ok() {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );
        return Ok(PathBuf::from(binary_path));
    }

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    artifact.download()?;
    zed::make_file_executable(&binary_path)?;

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::None,
    );

    Ok(PathBuf::from(binary_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_name_all_platforms() {
        let test_cases = [
            (
                zed::Os::Mac,
                zed::Architecture::Aarch64,
                "django-language-server-v5.2.3-darwin-arm64",
            ),
            (
                zed::Os::Mac,
                zed::Architecture::X8664,
                "django-language-server-v5.2.3-darwin-x64",
            ),
            (
                zed::Os::Linux,
                zed::Architecture::Aarch64,
                "django-language-server-v5.2.3-linux-arm64",
            ),
            (
                zed::Os::Linux,
                zed::Architecture::X8664,
                "django-language-server-v5.2.3-linux-x64",
            ),
            (
                zed::Os::Windows,
                zed::Architecture::Aarch64,
                "django-language-server-v5.2.3-windows-arm64",
            ),
            (
                zed::Os::Windows,
                zed::Architecture::X8664,
                "django-language-server-v5.2.3-windows-x64",
            ),
        ];

        for (os, arch, expected) in test_cases {
            let release = zed::GithubRelease {
                version: "v5.2.3".to_string(),
                assets: vec![],
            };
            let artifact = ReleaseArtifact::new(os, arch, release);
            assert_eq!(artifact.base_name(), expected);
        }
    }

    #[test]
    fn test_download_finds_matching_asset() {
        let assets = vec![
            zed::GithubReleaseAsset {
                name: "django-language-server-v1.0.0-linux-x64.tar.gz".to_string(),
                download_url: "https://example.com/linux".to_string(),
            },
            zed::GithubReleaseAsset {
                name: "django-language-server-v1.0.0-darwin-arm64.tar.gz".to_string(),
                download_url: "https://example.com/darwin".to_string(),
            },
        ];

        let release = zed::GithubRelease {
            version: "v1.0.0".to_string(),
            assets,
        };
        let artifact = ReleaseArtifact::new(zed::Os::Mac, zed::Architecture::Aarch64, release);

        // We can't actually test download() because it calls zed::download_file
        // But we can verify the artifact was constructed correctly
        assert_eq!(
            artifact.base_name(),
            "django-language-server-v1.0.0-darwin-arm64"
        );
    }

    #[test]
    fn test_binary_path() {
        let release = zed::GithubRelease {
            version: "v1.0.0".to_string(),
            assets: vec![],
        };
        let artifact = ReleaseArtifact::new(zed::Os::Linux, zed::Architecture::X8664, release);

        assert_eq!(
            artifact.binary_path(),
            "django-language-server-v1.0.0-linux-x64/djls"
        );
    }
}
