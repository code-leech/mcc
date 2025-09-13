include!("src/cli.rs"); // include Cli struct

fn main() {
    use std::{env, fs};
    use clap::CommandFactory;
    use clap_complete::{generate_to, Shell};
    let mut cmd = Cli::command();
    let home = env::var("HOME").expect("HOME environment variable not set");
    let shells = [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ];

    for &shell in &shells {
        let out_dir = match shell {
            Shell::Bash => format!("{}/.config/bash/completions", home),
            Shell::Zsh => format!("{}/.config/zsh/completions", home),
            Shell::Fish => format!("{}/.config/fish/completions", home),
            Shell::PowerShell => format!("{}/.config/powershell/completions", home),
            Shell::Elvish => format!("{}/.config/elvish/completions", home),
            _ => continue,
        };

        fs::create_dir_all(&out_dir).unwrap();
        generate_to(shell, &mut cmd, "mcc", &out_dir).unwrap();
        println!("Generated {} completions in {}", shell, out_dir);
    }
}
