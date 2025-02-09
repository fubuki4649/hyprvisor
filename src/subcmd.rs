use crate::{repo_functions, versions};
use crate::versions::{get_hyprland_versions, Version, HYPRVISOR_TEST_DIR};
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

pub fn install(version: String, dry: bool) -> Result<(), Error> {

    let versions = get_hyprland_versions()?;

    // Check if version exists
    if !versions.contains_key(&version) {
        println!("Hyprland version {} doesnt exist!", version);
        return Ok(())
    }

    // Do nothing if specified version is already installed
    if versions[version.as_str()].installed {
        println!("Hyprland version {} already installed!", version);
        return Ok(())
    }

    // Clone and build hyprland if not already cached
    if !versions[version.as_str()].cached {
        repo_functions::clone_hyprland(&version)?;
        repo_functions::build_hyprland(&version)?;
    }
    
    repo_functions::install_hyprland(&version, dry)?;
    
    Ok(())

}

pub fn uninstall(version: String) -> Result<(), Error> {
    
    // Check if specified version is installed
    if !Command::new("grep")
        .arg("-q")
        .arg(format!("\"{}\"", version))
        .arg(format!("{}/installed", &*HYPRVISOR_TEST_DIR))
        .status()?.success() {

        // Remove installed version if installed
        repo_functions::uninstall_hyprland(&version)?
        
    } else {
        println!("Hyprland version {} not installed!", version);
    }

    Ok(())

}