use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve runtime binary: PATH first, then common install locations (e.g. when run from flow/cron).
fn resolve_runtime_binary(name: &str) -> Option<PathBuf> {
    if let Ok(p) = which::which(name) {
        return Some(p);
    }
    let home = dirs::home_dir()?;
    let path = match name {
        "deno" => home.join(".deno/bin/deno"),
        "bun" => home.join(".bun/bin/bun"),
        _ => return None,
    };
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}

/// Ensure child process has HOME and runtime's dir on PATH (helps when kar is run from flow/cron with minimal env).
fn env_for_runtime(runtime_path: &Path) -> Command {
    let mut cmd = Command::new(runtime_path);
    if let Some(parent) = runtime_path.parent() {
        if let Ok(path) = env::var("PATH") {
            let new_path = format!("{}:{}", parent.display(), path);
            cmd.env("PATH", new_path);
        }
    }
    if let Some(home) = dirs::home_dir() {
        cmd.env("HOME", &home);
    }
    cmd
}

/// Check if a runtime actually works by running a simple eval
fn runtime_works(runtime_path: &Path) -> bool {
    // deno: deno eval "..." ; bun: bun --print "..." (bun has no "eval" subcommand)
    let result = if runtime_path.ends_with("deno") {
        env_for_runtime(runtime_path)
            .args(["eval", "console.log('ok')"])
            .output()
    } else {
        env_for_runtime(runtime_path)
            .args(["--print", "console.log('ok')"])
            .output()
    };
    match result {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// True if path is under a known install location (we trust it even when probe fails).
fn is_known_install_path(path: &Path) -> bool {
    if let Some(home) = dirs::home_dir() {
        let b = home.join(".bun/bin/bun");
        let d = home.join(".deno/bin/deno");
        path == b || path == d
    } else {
        false
    }
}

/// Detect available TypeScript runtime; returns (name, path to binary).
fn detect_runtime() -> Result<(&'static str, PathBuf)> {
    // Prefer Deno for its fast startup and native TS support
    if let Some(deno_path) = resolve_runtime_binary("deno") {
        if runtime_works(&deno_path) || is_known_install_path(&deno_path) {
            return Ok(("deno", deno_path));
        }
    }
    if let Some(bun_path) = resolve_runtime_binary("bun") {
        if runtime_works(&bun_path) || is_known_install_path(&bun_path) {
            return Ok(("bun", bun_path));
        }
    }
    // Helpful error when binary exists but doesn't work (e.g. wrong arch, broken install)
    let deno_found = resolve_runtime_binary("deno").is_some();
    let bun_found = resolve_runtime_binary("bun").is_some();
    if deno_found || bun_found {
        let found: Vec<&str> = [("deno", deno_found), ("bun", bun_found)]
            .iter()
            .filter(|(_, exists)| *exists)
            .map(|(name, _)| *name)
            .collect();
        bail!(
            "Found {} but it doesn't work correctly.\nInstall a working TypeScript runtime:\n  curl -fsSL https://deno.land/install.sh | sh\n  curl -fsSL https://bun.sh/install | bash",
            found.join(", ")
        );
    }
    bail!("No TypeScript runtime found. Install Deno or Bun:\n  curl -fsSL https://deno.land/install.sh | sh\n  curl -fsSL https://bun.sh/install | bash")
}

/// Execute a TypeScript config file and return the JSON output
pub fn execute_config(config_path: &Path) -> Result<String> {
    let (runtime_name, runtime_path) = detect_runtime()?;
    let config_path = config_path
        .canonicalize()
        .with_context(|| format!("Config file not found: {}", config_path.display()))?;

    let output = match runtime_name {
        "deno" => {
            let wrapper = format!(
                r#"import config from "file://{}"; console.log(JSON.stringify(config));"#,
                config_path.display()
            );
            env_for_runtime(&runtime_path)
                .args(["eval", "--ext=ts", &wrapper])
                .output()
                .context("Failed to run deno")?
        }
        "bun" => {
            // Bun has no "eval"; use a temp file and "bun run" so import works
            let wrapper = format!(
                r#"import config from "file://{}"; console.log(JSON.stringify(config));"#,
                config_path.display()
            );
            let temp_ts = env::temp_dir().join(format!("kar_wrapper_{}.ts", std::process::id()));
            fs::write(&temp_ts, &wrapper).context("Failed to write temp wrapper")?;
            let out = env_for_runtime(&runtime_path)
                .arg(&temp_ts)
                .output()
                .context("Failed to run bun")?;
            let _ = fs::remove_file(&temp_ts);
            out
        }
        _ => unreachable!(),
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("TypeScript execution failed:\n{}", stderr);
    }

    let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 in config output")?;

    Ok(stdout.trim().to_string())
}
