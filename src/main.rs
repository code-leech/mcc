mod cli;
use std::{env, path::PathBuf, process::Command};
use clap::Parser;
use colored::Colorize;
use serde_json::Value;
use cli::{Cli, Commands};

fn find_project(filename: &str) -> Option<PathBuf> {
    env::current_dir().ok()?.ancestors()
        .find(|dir| dir.join(filename).exists())
        .map(|dir| dir.to_path_buf())
}

fn test(dir: Option<PathBuf>) {
    let path = dir.unwrap_or_else(|| {
        println!("{} No Meson project detected.", "ERROR:".red().bold());
        std::process::exit(1);
    });

    if !path.join("builddir").exists() {
        Command::new("meson")
            .args(["setup", "builddir"])
            .current_dir(&path)
            .status()
            .expect("Failed to create builddir");
    }

    Command::new("meson")
        .args(["test", "-C", "builddir"])
        .current_dir(&path)
        .status()
        .unwrap();
}

fn build(dir: Option<PathBuf>) {
    let path = dir.unwrap_or_else(|| {
        println!("{} No Meson project detected.", "ERROR:".red().bold());
        std::process::exit(1);
    });

    if !path.join("builddir").exists() {
        Command::new("meson")
            .args(["setup", "builddir"])
            .current_dir(&path)
            .status()
            .expect("Failed to create builddir");
    }

    Command::new("meson")
        .args(["compile", "-C", "builddir"])
        .current_dir(&path)
        .status()
        .unwrap();
}

fn run() {
    let bin = PathBuf::from(
        serde_json::from_slice::<Value>(
            &Command::new("meson").args(["introspect", "builddir", "--targets"])
            .output().unwrap().stdout
        ).unwrap().as_array().unwrap()
        .iter().find(|t| t["type"]=="executable").unwrap()["filename"][0].as_str().unwrap()
    );
    Command::new(bin).status().unwrap();
}

fn main() {
    let cli = Cli::parse();
    let rootfolder = find_project("meson.build");

    match &cli.command {
        Commands::Run => {
            build(rootfolder);
            run();
        }
        Commands::Build => {
            build(rootfolder);
        }
        Commands::Test => {
            test(rootfolder);
        }
    }
}