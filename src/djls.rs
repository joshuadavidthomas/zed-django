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
            "{}-{}-{}-{}",
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

fn find_matching_asset<'a>(
    assets: &'a [zed::GithubReleaseAsset],
    artifact: &ReleaseArtifact,
) -> Result<&'a zed::GithubReleaseAsset> {
    assets
        .iter()
        .find(|asset| asset.name == artifact.asset_name())
        .ok_or_else(|| format!("No asset found matching {}", artifact.asset_name()))
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

    let asset = find_matching_asset(&release.assets, &artifact)?;

    let binary_path = artifact.binary_path();

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    zed::download_file(
        &asset.download_url,
        "",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_name_all_platforms() {
        let test_cases = [
            (
                Os::Darwin,
                Arch::Arm64,
                "django-language-server-v5.2.3-darwin-arm64",
            ),
            (
                Os::Darwin,
                Arch::X64,
                "django-language-server-v5.2.3-darwin-x64",
            ),
            (
                Os::Linux,
                Arch::Arm64,
                "django-language-server-v5.2.3-linux-arm64",
            ),
            (
                Os::Linux,
                Arch::X64,
                "django-language-server-v5.2.3-linux-x64",
            ),
            (
                Os::Windows,
                Arch::Arm64,
                "django-language-server-v5.2.3-windows-arm64",
            ),
            (
                Os::Windows,
                Arch::X64,
                "django-language-server-v5.2.3-windows-x64",
            ),
        ];

        for (os, arch, expected) in test_cases {
            let artifact = ReleaseArtifact {
                os,
                arch,
                version: "v5.2.3".to_string(),
            };
            assert_eq!(artifact.base_name(), expected);
        }
    }

    #[test]
    fn test_asset_name_all_platforms() {
        let test_cases = [
            (
                Os::Darwin,
                Arch::Arm64,
                "django-language-server-v5.2.3-darwin-arm64.tar.gz",
            ),
            (
                Os::Darwin,
                Arch::X64,
                "django-language-server-v5.2.3-darwin-x64.tar.gz",
            ),
            (
                Os::Linux,
                Arch::Arm64,
                "django-language-server-v5.2.3-linux-arm64.tar.gz",
            ),
            (
                Os::Linux,
                Arch::X64,
                "django-language-server-v5.2.3-linux-x64.tar.gz",
            ),
            (
                Os::Windows,
                Arch::Arm64,
                "django-language-server-v5.2.3-windows-arm64.zip",
            ),
            (
                Os::Windows,
                Arch::X64,
                "django-language-server-v5.2.3-windows-x64.zip",
            ),
        ];

        for (os, arch, expected) in test_cases {
            let artifact = ReleaseArtifact {
                os,
                arch,
                version: "v5.2.3".to_string(),
            };
            assert_eq!(artifact.asset_name(), expected);
        }
    }

    #[test]
    fn test_binary_path_all_platforms() {
        let test_cases = [
            (
                Os::Darwin,
                Arch::Arm64,
                "django-language-server-v1.0.0-darwin-arm64/djls",
            ),
            (
                Os::Darwin,
                Arch::X64,
                "django-language-server-v1.0.0-darwin-x64/djls",
            ),
            (
                Os::Linux,
                Arch::Arm64,
                "django-language-server-v1.0.0-linux-arm64/djls",
            ),
            (
                Os::Linux,
                Arch::X64,
                "django-language-server-v1.0.0-linux-x64/djls",
            ),
            (
                Os::Windows,
                Arch::Arm64,
                "django-language-server-v1.0.0-windows-arm64/djls",
            ),
            (
                Os::Windows,
                Arch::X64,
                "django-language-server-v1.0.0-windows-x64/djls",
            ),
        ];

        for (os, arch, expected) in test_cases {
            let artifact = ReleaseArtifact {
                os,
                arch,
                version: "v1.0.0".to_string(),
            };
            assert_eq!(artifact.binary_path(), expected);
        }
    }

    #[test]
    fn test_find_matching_asset_success() {
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

        let artifact = ReleaseArtifact {
            os: Os::Darwin,
            arch: Arch::Arm64,
            version: "v1.0.0".to_string(),
        };

        let result = find_matching_asset(&assets, &artifact);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().name,
            "django-language-server-v1.0.0-darwin-arm64.tar.gz"
        );
    }

    #[test]
    fn test_find_matching_asset_not_found() {
        let assets = vec![zed::GithubReleaseAsset {
            name: "django-language-server-v1.0.0-linux-x64.tar.gz".to_string(),
            download_url: "https://example.com/linux".to_string(),
        }];

        let artifact = ReleaseArtifact {
            os: Os::Darwin,
            arch: Arch::Arm64,
            version: "v1.0.0".to_string(),
        };

        let result = find_matching_asset(&assets, &artifact);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No asset found matching django-language-server-v1.0.0-darwin-arm64.tar.gz"
        );
    }

    #[test]
    fn test_find_matching_asset_empty_list() {
        let assets = vec![];

        let artifact = ReleaseArtifact {
            os: Os::Linux,
            arch: Arch::X64,
            version: "v1.0.0".to_string(),
        };

        let result = find_matching_asset(&assets, &artifact);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No asset found matching django-language-server-v1.0.0-linux-x64.tar.gz"
        );
    }
}
