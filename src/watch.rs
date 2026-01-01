use crate::{config, karabiner, runtime};
use anyhow::{Context, Result};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub fn watch(config_path: &Path, profile: &str) -> Result<()> {
    let config_path = config_path
        .canonicalize()
        .with_context(|| format!("Config file not found: {}", config_path.display()))?;

    let karabiner_path = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".config/karabiner/karabiner.json");

    println!("Watching {} ...", config_path.display());

    // Initial build
    if let Err(e) = build_once(&config_path, &karabiner_path, profile) {
        eprintln!("Error: {}", e);
    }

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Watch the config file's parent directory
    let watch_dir = config_path.parent().unwrap_or(Path::new("."));
    watcher.watch(watch_dir, RecursiveMode::NonRecursive)?;

    let mut last_build = Instant::now();
    let debounce = Duration::from_millis(100);

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                // Check if the event is for our config file
                let is_config_change = event.paths.iter().any(|p| p == &config_path);

                if is_config_change && last_build.elapsed() > debounce {
                    last_build = Instant::now();
                    println!("\nRebuilding...");

                    match build_once(&config_path, &karabiner_path, profile) {
                        Ok(_) => println!("Updated profile '{}'", profile),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
            Err(e) => {
                eprintln!("Channel error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn build_once(config_path: &Path, karabiner_path: &Path, profile: &str) -> Result<()> {
    let json = runtime::execute_config(config_path)?;
    let user_config: config::UserConfig = serde_json::from_str(&json)?;
    let rules = config::to_karabiner_rules(&user_config)?;
    let simple_mods = config::to_simple_modifications(&user_config);
    karabiner::update_profile(karabiner_path, profile, rules, simple_mods)?;
    Ok(())
}
