use chrono::{DateTime, Utc};
use git2::{Error, Repository};
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use once_cell::sync::Lazy;

pub type VersionList = HashMap<String, VersionInfo>;
pub type Version = (String, VersionInfo);

pub struct VersionInfo {
    pub release_time: DateTime<Utc>,
    pub cached: bool,
    pub installed: bool
}



pub static HYPRVISOR_CACHE_DIR: Lazy<String> = Lazy::new(|| {
    format!("{}/.cache/hyprvisor", env!("HOME"))
});

pub static HYPRVISOR_TEST_DIR: Lazy<String> = Lazy::new(|| {
    "./test".to_string()
});

pub const HYPRLAND_REPO_URL: &str = "https://github.com/hyprwm/Hyprland.git";


pub fn is_version_cached(version_name: &String) -> bool {
    Path::new(format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version_name).as_str()).is_dir()
}

pub fn is_version_installed(version_name: &String) -> bool {
    fs::read_to_string(format!("{}/versions_installed", &*HYPRVISOR_TEST_DIR).as_str()).unwrap_or("".to_string()).contains(version_name)
}

pub fn get_hyprland_versions() -> Result<VersionList, Error> {

    // Fetch tags from GitHub
    let repo = Repository::init(&*HYPRVISOR_TEST_DIR)?;
    match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(_) => repo.remote("origin", HYPRLAND_REPO_URL)?,
    }.fetch(&["refs/tags/v*:refs/tags/v*"], None, None)?;

    // Get release tags
    let versions: VersionList = repo.references_glob("refs/tags/v*")?.filter_map(|reference| {
        match reference {
            Ok(tag) => {
                let version_name = tag.name().unwrap_or("").strip_prefix("refs/tags/")?.to_string();

                // Return None for all beta versions
                (!version_name.clone().contains("beta")).then_some(())?;

                let commit = tag.peel_to_commit().ok()?;
                let version_info = VersionInfo {
                    release_time: DateTime::from_timestamp(commit.time().seconds(), 0).unwrap_or_else(Utc::now),
                    cached: is_version_cached(&version_name),
                    installed: is_version_installed(&version_name),
                };
                Some((version_name, version_info))
            },
            Err(_) => None
        }
    }).collect::<VersionList>();

    Ok(versions)

}



