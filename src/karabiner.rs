use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

// Karabiner JSON types matching the official spec

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarabinerConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<Global>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_for_updates_on_startup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_menu_bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_profile_name_in_menu_bar: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub selected: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub simple_modifications: Vec<SimpleModificationEntry>,
    #[serde(default)]
    pub complex_modifications: ComplexModifications,
    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModificationEntry {
    pub from: SimpleModificationKey,
    pub to: Vec<SimpleModificationKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModificationKey {
    pub key_code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplexModifications {
    #[serde(default)]
    pub parameters: Parameters,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(
        rename = "basic.simultaneous_threshold_milliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub simultaneous_threshold: Option<u32>,
    #[serde(
        rename = "basic.to_if_alone_timeout_milliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_if_alone_timeout: Option<u32>,
    #[serde(
        rename = "basic.to_if_held_down_threshold_milliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_if_held_down_threshold: Option<u32>,
    #[serde(
        rename = "basic.to_delayed_action_delay_milliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_delayed_action_delay: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manipulator {
    #[serde(rename = "type")]
    pub manipulator_type: String,
    pub from: FromEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<ToEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<ToEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_held_down: Option<Vec<ToEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ManipulatorParameters>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManipulatorParameters {
    #[serde(
        rename = "basic.simultaneous_threshold_milliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub simultaneous_threshold: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FromEvent {
    KeyCode(FromKeyCode),
    Simultaneous(FromSimultaneous),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromKeyCode {
    pub key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<FromModifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromSimultaneous {
    pub simultaneous: Vec<SimultaneousKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simultaneous_options: Option<SimultaneousOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<FromModifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimultaneousKey {
    pub key_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimultaneousOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_key_down_uninterruptedly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_down_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_up_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_up_when: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToEvent>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FromModifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToEvent {
    KeyCode(ToKeyCode),
    ConsumerKeyCode(ToConsumerKeyCode),
    PointingButton(ToPointingButton),
    ShellCommand(ToShellCommand),
    SetVariable(ToSetVariable),
    MouseKey(ToMouseKey),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToPointingButton {
    pub pointing_button: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToConsumerKeyCode {
    pub consumer_key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToKeyCode {
    pub key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToShellCommand {
    pub shell_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToSetVariable {
    pub set_variable: SetVariable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVariable {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMouseKey {
    pub mouse_key: MouseKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_wheel: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_wheel: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_multiplier: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    #[serde(rename = "variable_if")]
    VariableIf { name: String, value: serde_json::Value },
    #[serde(rename = "variable_unless")]
    VariableUnless { name: String, value: serde_json::Value },
    #[serde(rename = "frontmost_application_if")]
    FrontmostAppIf {
        #[serde(skip_serializing_if = "Option::is_none")]
        bundle_identifiers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_paths: Option<Vec<String>>,
    },
    #[serde(rename = "frontmost_application_unless")]
    FrontmostAppUnless {
        #[serde(skip_serializing_if = "Option::is_none")]
        bundle_identifiers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_paths: Option<Vec<String>>,
    },
}

/// Update a profile in karabiner.json with new rules
pub fn update_profile(
    path: &Path,
    profile_name: &str,
    rules: Vec<Rule>,
    simple_modifications: Vec<SimpleModificationEntry>,
) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let mut config: KarabinerConfig =
        serde_json::from_str(&content).context("Failed to parse karabiner.json")?;

    let profile = config
        .profiles
        .iter_mut()
        .find(|p| p.name == profile_name)
        .with_context(|| format!("Profile '{}' not found", profile_name))?;

    profile.complex_modifications.rules = rules;
    if !simple_modifications.is_empty() {
        profile.simple_modifications = simple_modifications;
    }

    let output = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, output).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}
