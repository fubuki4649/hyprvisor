use anyhow::Error;
use std::process::Command;
use std::fs::OpenOptions;
use git2::FetchOptions;
use git2::build::RepoBuilder;
use std::path::Path;
use std::io::Write;
use crate::versions::{HYPRLAND_REPO_URL, HYPRVISOR_TEST_DIR};


pub fn clone_hyprland(version: &String) -> Result<(), Error> {
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
    let obj = repo.revparse_single(version)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head_detached(obj.id())?;

    Ok(())
}

pub fn build_hyprland(version: &String) -> Result<(), Error> {

    Command::new("make")
        .arg("-C") // Change directory before running make
        .arg(&*format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version))
        .arg("all")
        .output()?;

    Ok(())

}

pub fn install_hyprland(version: &String, dry: bool) -> Result<(), Error> {

    // Install hyprland
    if !dry {
        Command::new("sudo")
            .arg("make")
            .arg("-C") // Change directory before running make
            .arg(format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version))
            .arg("install")
            .output()?;
    }
    
    // Mark hyprland version as installed
    let mut file = OpenOptions::new().append(true).create(true).open(format!("{}/installed", &*HYPRVISOR_TEST_DIR))?;
    writeln!(file, "{}", version)?;
    
    Ok(())

}

pub fn uninstall_hyprland(version: &String) -> Result<(), Error> {

    // Uninstall hyprland
    Command::new("sudo")
        .arg("make")
        .arg("-C") // Change directory before running make
        .arg(format!("{}/hyprland-{}", &*HYPRVISOR_TEST_DIR, version))
        .arg("uninstall")
        .output()?;

    // Mark hyprland as not installed
    Command::new("sed")
        .arg("-i")
        .arg(format!("'/{}/d'", version))
        .arg(format!("{}/installed", &*HYPRVISOR_TEST_DIR))
        .output()?;
    
    Ok(())
    
}