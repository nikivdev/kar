use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Detect available TypeScript runtime
fn detect_runtime() -> Result<&'static str> {
    // Prefer Deno for its fast startup and native TS support
    if which::which("deno").is_ok() {
        return Ok("deno");
    }
    if which::which("bun").is_ok() {
        return Ok("bun");
    }
    bail!("No TypeScript runtime found. Install Deno or Bun:\n  curl -fsSL https://deno.land/install.sh | sh\n  curl -fsSL https://bun.sh/install | bash")
}

/// Execute a TypeScript config file and return the JSON output
pub fn execute_config(config_path: &Path) -> Result<String> {
    let runtime = detect_runtime()?;
    let config_path = config_path
        .canonicalize()
        .with_context(|| format!("Config file not found: {}", config_path.display()))?;

    // Wrapper script that imports config and outputs JSON
    let wrapper = format!(
        r#"
import config from "file://{}";
console.log(JSON.stringify(config));
"#,
        config_path.display()
    );

    let output = match runtime {
        "deno" => Command::new("deno")
            .args(["eval", "--ext=ts", &wrapper])
            .output()
            .context("Failed to run deno")?,
        "bun" => Command::new("bun")
            .args(["eval", &wrapper])
            .output()
            .context("Failed to run bun")?,
        _ => unreachable!(),
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("TypeScript execution failed:\n{}", stderr);
    }

    let stdout = String::from_utf8(output.stdout).context("Invalid UTF-8 in config output")?;

    Ok(stdout.trim().to_string())
}
