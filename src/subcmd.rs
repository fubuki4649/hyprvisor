use crate::versions;
use crate::versions::{get_hyprland_versions, Version, HYPRLAND_REPO_URL, HYPRVISOR_TEST_DIR};
use git2::build::RepoBuilder;
use git2::FetchOptions;
use std::path::Path;
use std::process::Command;
use anyhow::Error;

pub fn ls() -> Result<(), Error> {

    // Sort tags by date
    let mut versions: Vec<Version> = versions::get_hyprland_versions()?.into_iter().collect();
    versions.sort_by(|a, b| a.1.release_time.cmp(&b.1.release_time));

    // Print results
    println!("{:<10} {:<15} {:<12} {}", "Version", "", "", "Time of Release");
    println!("--------------------------------------------------------------");
    for version in versions {
        println!("{:<10} {:<15} {:<15} {}",
                 version.0,
                 if version.1.installed { "Installed" } else { "" },
                 if version.1.cached { "Cached" } else { "" },
                 version.1.release_time.format("%Y-%m-%d %H:%M:%S")
        );
    }

    Ok(())

}

fn clone_hyprland(version: &String) -> Result<(), Error> {
    // Set up fetch options
    let mut fetch_opts = FetchOptions::new();
    fetch_opts.download_tags(git2::AutotagOption::All);

    // Configure and perform the clone
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_opts);
    let repo = builder.clone(HYPRLAND_REPO_URL, Path::new(&*format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version)))?;

    // Initialize and update all submodules
    let submodules = repo.submodules()?;
    for mut submodule in submodules {
        submodule.update(true, None)?;
    }

    // Checkout the specified tag
    let obj = repo.revparse_single(&*version)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head_detached(obj.id())?;

    Ok(())
}

fn build_hyprland(version: &String) -> Result<(), Error> {

    Command::new("make")
        .arg("-C") // Change directory before running make
        .arg(&*format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version))
        .arg("all")
        .output()?;

    Ok(())

}

fn install_hyprland(version: &String) -> Result<(), Error> {

    // Install hyprland
    Command::new("sudo")
        .arg("make")
        .arg("-C") // Change directory before running make
        .arg(&*format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version))
        .arg("install")
        .output()?;

    // Mark hyprland as installed
    // TODO
    
    
    Ok(())

}



pub fn install(version: String, dry: bool) -> Result<(), Error> {

    let versions = get_hyprland_versions()?;

    // Do nothing if specified version is already installed
    if versions[version.as_str()].installed {
        return Ok(())
    }

    // Clone and build hyprland if not already cached
    if !versions[version.as_str()].cached {
        clone_hyprland(&version)?;
        build_hyprland(&version)?;
    }
    
    if !dry {
        install_hyprland(&version)?;
    }
    
    Ok(())

}