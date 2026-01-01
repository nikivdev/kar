mod config;
mod karabiner;
mod runtime;
mod watch;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/kar/config.ts")
}

#[derive(Parser)]
#[command(name = "kar")]
#[command(about = "Fast Karabiner config in TypeScript")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to config file (default: ~/.config/kar/config.ts)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// Profile name to update (default: kar)
    #[arg(short, long, global = true, default_value = "kar")]
    profile: String,

    /// Print JSON to stdout instead of writing
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build karabiner.json from config (default command)
    Build,

    /// Watch config and rebuild on changes
    Watch,

    /// Create ~/.config/kar/config.ts from example
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    match cli.command.unwrap_or(Commands::Build) {
        Commands::Build => build(&config_path, cli.dry_run, &cli.profile),
        Commands::Watch => watch::watch(&config_path, &cli.profile),
        Commands::Init => init(),
    }
}

fn build(config_path: &PathBuf, dry_run: bool, profile: &str) -> Result<()> {
    // Execute TS config and get JSON
    let json = runtime::execute_config(config_path)
        .with_context(|| format!("Failed to execute {}", config_path.display()))?;

    // Parse the simplified config
    let user_config: config::UserConfig =
        serde_json::from_str(&json).context("Failed to parse config JSON")?;

    // Convert to Karabiner format
    let rules = config::to_karabiner_rules(&user_config)?;
    let simple_mods = config::to_simple_modifications(&user_config);

    if dry_run {
        let output = serde_json::to_string_pretty(&rules)?;
        println!("{}", output);
        return Ok(());
    }

    // Write to karabiner.json
    let karabiner_path = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".config/karabiner/karabiner.json");

    karabiner::update_profile(&karabiner_path, profile, rules, simple_mods)?;

    println!("Updated profile '{}'", profile);
    Ok(())
}

fn init() -> Result<()> {
    let config_dir = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".config/kar");

    let config_path = config_dir.join("config.ts");

    if config_path.exists() {
        anyhow::bail!("{} already exists", config_path.display());
    }

    std::fs::create_dir_all(&config_dir)?;

    let example = include_str!("../examples/config.ts");
    std::fs::write(&config_path, example)?;

    println!("Created {}", config_path.display());
    Ok(())
}
