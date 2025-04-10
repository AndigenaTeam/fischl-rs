use std::fs;
use std::fs::File;
use std::path::Path;
use compress_tools::Ownership;
use reqwest::header::USER_AGENT;
use crate::utils::git_structs::GithubRelease;

pub mod git_structs;

pub fn get_github_release(repo_owner: String, repo_name: String) -> Option<GithubRelease> {
    if repo_name.is_empty() || repo_owner.is_empty() {
        None
    } else {
        let url = format!("https://api.github.com/repos/{}/{}/releases/latest", repo_owner, repo_name);
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).header(USER_AGENT, "KeqingLauncher/tauri-app").send();
        if response.is_ok() {
            let list = response.unwrap();
            let jsonified: GithubRelease = list.json().unwrap();
            Some(jsonified)
        } else {
            None
        }
    }
}

pub fn extract_archive(archive_path: String, extract_path: String) -> Option<bool> {
    let src = Path::new(&archive_path);
    let dest = Path::new(&extract_path);

    if !src.exists() {
        None
    } else if !dest.exists() {
        fs::create_dir_all(&dest).unwrap();
        let mut file = File::open(&src).unwrap();
        compress_tools::uncompress_archive(&mut file, &dest, Ownership::Preserve).unwrap();
        fs::remove_file(&src).unwrap();
        Some(true)
    } else {
        let mut file = File::open(&src).unwrap();
        compress_tools::uncompress_archive(&mut file, &dest, Ownership::Preserve).unwrap();
        fs::remove_file(&src).unwrap();
        Some(true)
    }
}

/*#[derive(Debug, Clone)]
pub struct GameManifest {
    pub version: i32,
    pub display_name: String,
    pub biz: String,
    pub latest_version: String,
    pub game_versions: Vec<GameVersion>,
    pub telemetry_hosts: Vec<String>,
    pub paths: GamePaths,
    pub assets: VersionAssets,
    pub extra: GameExtras
}

#[derive(Debug, Clone)]
pub struct GameVersion {
    pub metadata: VersionMetadata,
    pub assets: VersionAssets,
    pub game: VersionGameFiles,
    pub audio: VersionAudioFiles
}

#[derive(Debug, Clone)]
pub struct GamePaths {
    pub exe_filename: String,
    pub installation_dir: String,
    pub screenshot_dir: String,
    pub screenshot_dir_relative_to: String
}

#[derive(Debug, Clone)]
pub struct VersionMetadata {
    pub versioned_name: String,
    pub version: String,
    pub game_hash: String
}

#[derive(Debug, Clone)]
pub struct VersionAssets {
    pub game_icon: String,
    pub game_background: String
}

#[derive(Debug, Clone)]
pub struct VersionGameFiles {
    pub full: Vec<FullGameFile>,
    pub diff: Vec<DiffGameFile>
}

#[derive(Debug, Clone)]
pub struct FullGameFile {
    pub file_url: String,
    pub compressed_size: String,
    pub decompressed_size: String,
    pub file_hash: String,
    pub file_path: String
}

#[derive(Debug, Clone)]
pub struct DiffGameFile {
    pub file_url: String,
    pub compressed_size: String,
    pub decompressed_size: String,
    pub file_hash: String,
    pub diff_type: String,
    pub original_version: String,
    pub delete_files: Vec<String>
}

#[derive(Debug, Clone)]
pub struct VersionAudioFiles {
    pub full: Vec<FullAudioFile>,
    pub diff: Vec<DiffAudioFile>
}

#[derive(Debug, Clone)]
pub struct FullAudioFile {
    pub file_url: String,
    pub compressed_size: String,
    pub decompressed_size: String,
    pub file_hash: String,
    pub language: String
}

#[derive(Debug, Clone)]
pub struct DiffAudioFile {
    pub file_url: String,
    pub compressed_size: String,
    pub decompressed_size: String,
    pub file_hash: String,
    pub diff_type: String,
    pub original_version: String,
    pub language: String
}

#[derive(Debug, Clone)]
pub struct GamePreload {
    pub metadata: Option<VersionMetadata>,
    pub game: Option<VersionGameFiles>,
    pub audio: Option<VersionAudioFiles>
}

#[derive(Debug, Clone)]
pub struct GameTweakSwitches {
    pub fps_unlocker: bool,
    pub jadeite: bool,
    pub xxmi: bool
}

#[derive(Debug, Clone)]
pub struct GameExtras {
    pub preload: Option<GamePreload>,
    pub switches: GameTweakSwitches,
    pub fps_unlock_options: Vec<String>,
}*/