use crate::karabiner::{
    Condition, DeviceIdentifier, FromEvent, FromKeyCode, FromModifiers, FromSimultaneous,
    InputSource, KarabinerConfig, Manipulator, ManipulatorParameters, Parameters, Rule,
    SetVariable, SimpleModificationEntry, SimpleModificationKey, SimultaneousKey,
    SimultaneousOptions, SocketCommand, ToDelayedAction, ToEvent, ToKeyCode, ToMouseKey,
    ToPointingButton, ToSendUserCommand, ToSetVariable, ToShellCommand, ToSocketCommand,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// User-facing config schema (simplified, declarative)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(default)]
    pub profile: ProfileSettings,
    #[serde(default)]
    pub simlayers: HashMap<String, Simlayer>,
    #[serde(default)]
    pub simple: Vec<SimpleModification>,
    #[serde(default)]
    pub imports: Vec<ImportSource>,
    #[serde(default)]
    pub rules: Vec<UserRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportSource {
    ImportJson {
        import_json: String,
    },
    ImportProfile {
        import_profile: String,
        #[serde(default)]
        karabiner_json: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleModification {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileSettings {
    #[serde(default = "default_alone")]
    pub alone: u32,
    #[serde(default = "default_sim")]
    pub sim: u32,
}

fn default_alone() -> u32 {
    80
}
fn default_sim() -> u32 {
    200
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simlayer {
    pub key: String,
    #[serde(default)]
    pub modifiers: Option<Modifiers>,
    #[serde(default)]
    pub optional: Option<Vec<String>>,
    #[serde(default)]
    pub threshold: Option<u32>,
    #[serde(default)]
    pub alone: Option<u32>,
    #[serde(default)]
    pub condition: Option<UserCondition>,
    #[serde(default)]
    pub delay_ms: Option<u32>,
    #[serde(default)]
    pub leader: Option<LeaderMode>,
    #[serde(default = "default_simlayer_mode")]
    pub mode: SimlayerMode,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderMode {
    Enabled(bool),
    Config(LeaderConfig),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaderConfig {
    #[serde(default)]
    pub sticky: bool,
    #[serde(default)]
    pub escape: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SimlayerMode {
    Hold,
    Simultaneous,
}

fn default_simlayer_mode() -> SimlayerMode {
    SimlayerMode::Simultaneous
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRule {
    #[serde(default)]
    pub id: Option<String>,
    pub description: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub condition: Option<UserCondition>,
    #[serde(default)]
    pub note: Option<String>,
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserCondition {
    App {
        app: String,
    },
    Apps {
        apps: Vec<String>,
    },
    AppUnless {
        app_unless: String,
    },
    AppsUnless {
        apps_unless: Vec<String>,
    },
    Device {
        device: UserDeviceIdentifier,
    },
    Devices {
        devices: Vec<UserDeviceIdentifier>,
    },
    DeviceUnless {
        device_unless: UserDeviceIdentifier,
    },
    DevicesUnless {
        devices_unless: Vec<UserDeviceIdentifier>,
    },
    DeviceExists {
        device_exists: UserDeviceIdentifier,
    },
    DevicesExists {
        devices_exists: Vec<UserDeviceIdentifier>,
    },
    DeviceExistsUnless {
        device_exists_unless: UserDeviceIdentifier,
    },
    DevicesExistsUnless {
        devices_exists_unless: Vec<UserDeviceIdentifier>,
    },
    InputSource {
        input_source: UserInputSource,
    },
    InputSources {
        input_sources: Vec<UserInputSource>,
    },
    InputSourceUnless {
        input_source_unless: UserInputSource,
    },
    InputSourcesUnless {
        input_sources_unless: Vec<UserInputSource>,
    },
    KeyboardType {
        keyboard_type: String,
    },
    KeyboardTypes {
        keyboard_types: Vec<String>,
    },
    KeyboardTypeUnless {
        keyboard_type_unless: String,
    },
    KeyboardTypesUnless {
        keyboard_types_unless: Vec<String>,
    },
    Variable {
        variable: String,
        value: serde_json::Value,
    },
    VariableUnless {
        variable_unless: String,
        value: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeviceIdentifier {
    #[serde(default)]
    pub vendor_id: Option<u32>,
    #[serde(default)]
    pub product_id: Option<u32>,
    #[serde(default)]
    pub location_id: Option<u32>,
    #[serde(default)]
    pub is_keyboard: Option<bool>,
    #[serde(default)]
    pub is_pointing_device: Option<bool>,
    #[serde(default)]
    pub is_game_pad: Option<bool>,
    #[serde(default)]
    pub is_consumer: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInputSource {
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub input_source_id: Option<String>,
    #[serde(default)]
    pub input_mode_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    #[serde(default)]
    pub id: Option<String>,
    pub from: FromKey,
    pub to: ToKey,
    #[serde(default)]
    pub to_if_alone: Option<ToKey>,
    #[serde(default)]
    pub to_if_held: Option<ToKey>,
    #[serde(default)]
    pub to_after_key_up: Option<ToKey>,
    #[serde(default)]
    pub to_delayed: Option<MappingDelayedAction>,
    #[serde(default)]
    pub parameters: Option<MappingParameters>,
    #[serde(default)]
    pub condition: Option<UserCondition>,
    #[serde(default)]
    pub to_if_single_tap: Option<ToKey>,
    #[serde(default)]
    pub double_tap_delay_ms: Option<u32>,
    #[serde(default)]
    pub signal: Option<MappingSignal>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingDelayedAction {
    pub invoked: ToKey,
    pub canceled: ToKey,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MappingParameters {
    #[serde(default)]
    pub simultaneous_threshold_ms: Option<u32>,
    #[serde(default)]
    pub to_if_alone_timeout_ms: Option<u32>,
    #[serde(default)]
    pub to_if_held_down_threshold_ms: Option<u32>,
    #[serde(default)]
    pub to_delayed_action_delay_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingSignal {
    #[serde(default)]
    pub intent: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub criticality: Option<SignalCriticality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignalCriticality {
    Low,
    Med,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FromKey {
    Simple(String),
    WithModifiers {
        key: String,
        #[serde(default)]
        modifiers: Option<Modifiers>,
        #[serde(default)]
        optional: Option<Vec<String>>,
    },
    DoubleTap {
        double_tap: String,
        #[serde(default)]
        modifiers: Option<Modifiers>,
        #[serde(default)]
        optional: Option<Vec<String>>,
    },
    Simultaneous(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Modifiers {
    Single(String),
    Multiple(Vec<String>),
}

impl Modifiers {
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Modifiers::Single(s) => vec![s.clone()],
            Modifiers::Multiple(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMouseKey {
    #[serde(default)]
    pub x: Option<i32>,
    #[serde(default)]
    pub y: Option<i32>,
    #[serde(default)]
    pub vertical_wheel: Option<i32>,
    #[serde(default)]
    pub horizontal_wheel: Option<i32>,
    #[serde(default)]
    pub speed_multiplier: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendUserCommand {
    pub payload: serde_json::Value,
    #[serde(default)]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToKey {
    Simple(String),
    WithModifiers {
        key: String,
        #[serde(default)]
        modifiers: Option<Modifiers>,
    },
    Shell {
        shell: String,
    },
    SocketCommand {
        socket_command: SocketCommand,
    },
    SetVariable {
        set_variable: SetVariableSpec,
    },
    SendUserCommand {
        send_user_command: SendUserCommand,
    },
    MouseKey {
        mouse_key: UserMouseKey,
    },
    PointingButton {
        pointing_button: String,
    },
    Multiple(Vec<ToKey>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVariableSpec {
    pub name: String,
    pub value: serde_json::Value,
}

/// Convert user config to Karabiner rules
pub fn to_karabiner_rules(config: &UserConfig) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for (rule_idx, user_rule) in config.rules.iter().enumerate() {
        let rule = convert_rule(user_rule, config, rule_idx)?;
        // Karabiner rejects rules with empty manipulators.
        if !rule.manipulators.is_empty() {
            rules.push(rule);
        }
    }

    Ok(rules)
}

pub fn load_imported_rules(
    config: &UserConfig,
    config_path: &Path,
    default_karabiner_path: &Path,
) -> Result<Vec<Rule>> {
    let mut out = Vec::new();
    let config_dir = config_path.parent().unwrap_or_else(|| Path::new("."));

    for source in &config.imports {
        match source {
            ImportSource::ImportJson { import_json } => {
                let path = if Path::new(import_json).is_absolute() {
                    Path::new(import_json).to_path_buf()
                } else {
                    config_dir.join(import_json)
                };
                let raw = std::fs::read_to_string(&path)
                    .with_context(|| format!("failed to read import_json: {}", path.display()))?;
                let value: serde_json::Value = serde_json::from_str(&raw)
                    .with_context(|| format!("failed to parse JSON from {}", path.display()))?;
                out.extend(parse_imported_rules_value(&value).with_context(|| {
                    format!("failed to parse imported rules from {}", path.display())
                })?);
            }
            ImportSource::ImportProfile {
                import_profile,
                karabiner_json,
            } => {
                let path = karabiner_json
                    .as_ref()
                    .map(|p| {
                        if Path::new(p).is_absolute() {
                            Path::new(p).to_path_buf()
                        } else {
                            config_dir.join(p)
                        }
                    })
                    .unwrap_or_else(|| default_karabiner_path.to_path_buf());
                let raw = std::fs::read_to_string(&path).with_context(|| {
                    format!(
                        "failed to read karabiner config for import_profile: {}",
                        path.display()
                    )
                })?;
                let cfg: KarabinerConfig = serde_json::from_str(&raw).with_context(|| {
                    format!("failed to parse karabiner config: {}", path.display())
                })?;
                let profile = cfg
                    .profiles
                    .iter()
                    .find(|p| p.name == *import_profile)
                    .with_context(|| {
                        format!(
                            "import_profile '{}' not found in {}",
                            import_profile,
                            path.display()
                        )
                    })?;
                out.extend(profile.complex_modifications.rules.clone());
            }
        }
    }

    Ok(out)
}

fn parse_imported_rules_value(value: &serde_json::Value) -> Result<Vec<Rule>> {
    if let Some(rules_val) = value.get("rules") {
        return Ok(serde_json::from_value(rules_val.clone())?);
    }
    if value.is_array() {
        return Ok(serde_json::from_value(value.clone())?);
    }
    if value.get("manipulators").is_some() {
        return Ok(vec![serde_json::from_value(value.clone())?]);
    }
    anyhow::bail!("expected JSON object with 'rules', a rules array, or a single rule object");
}

fn convert_rule(user_rule: &UserRule, config: &UserConfig, rule_idx: usize) -> Result<Rule> {
    let mut manipulators = Vec::new();

    // Check if this rule uses a simlayer
    let simlayer = user_rule
        .layer
        .as_ref()
        .and_then(|name| config.simlayers.get(name).map(|s| (name, s)));

    let base_conditions = merge_conditions(
        build_base_conditions(&user_rule.condition),
        simlayer.and_then(|(_, layer)| build_base_conditions(&layer.condition)),
    );
    let leader_enabled = simlayer
        .map(|(_, layer)| layer.mode == SimlayerMode::Hold && simlayer_leader_enabled(layer))
        .unwrap_or(false);
    let leader_sticky = simlayer
        .map(|(_, layer)| simlayer_leader_sticky(layer))
        .unwrap_or(false);

    // Goku-style hold layer: press-and-hold layer key to activate variable, tap to emit key.
    if let Some((layer_name, layer)) = simlayer {
        if layer.mode == SimlayerMode::Hold {
            let from = FromEvent::KeyCode(FromKeyCode {
                key_code: layer.key.clone(),
                modifiers: Some(simlayer_from_modifiers(layer, true)),
            });
            let set_var_on = ToEvent::SetVariable(ToSetVariable {
                set_variable: SetVariable {
                    name: layer_name.clone(),
                    value: serde_json::Value::Number(1.into()),
                },
            });
            let set_var_off = ToEvent::SetVariable(ToSetVariable {
                set_variable: SetVariable {
                    name: layer_name.clone(),
                    value: serde_json::Value::Number(0.into()),
                },
            });
            let to_after_key_up = if leader_enabled {
                None
            } else {
                Some(vec![set_var_off.clone()])
            };
            let to_if_alone = vec![ToEvent::KeyCode(ToKeyCode {
                key_code: layer.key.clone(),
                modifiers: None,
                lazy: None,
                repeat: None,
            })];
            let delay_ms = layer.delay_ms;
            let mut params = ManipulatorParameters {
                simultaneous_threshold: None,
                to_if_alone_timeout: Some(layer.alone.unwrap_or(config.profile.alone)),
                to_if_held_down_threshold: delay_ms,
                to_delayed_action_delay: None,
            };
            if delay_ms.is_some() && layer.alone.is_none() {
                params.to_if_alone_timeout = delay_ms;
            }
            manipulators.push(Manipulator {
                manipulator_type: "basic".to_string(),
                from,
                to: if delay_ms.is_some() {
                    None
                } else {
                    Some(vec![set_var_on.clone()])
                },
                to_if_alone: Some(to_if_alone),
                to_if_held_down: if delay_ms.is_some() {
                    Some(vec![set_var_on.clone()])
                } else {
                    None
                },
                to_after_key_up,
                to_delayed_action: None,
                conditions: base_conditions.clone(),
                parameters: Some(params),
            });

            if leader_enabled {
                for escape_key in simlayer_leader_escape_keys(layer) {
                    let mut escape_conditions = base_conditions.clone().unwrap_or_default();
                    escape_conditions.push(Condition::VariableIf {
                        name: layer_name.clone(),
                        value: serde_json::Value::Number(1.into()),
                    });
                    manipulators.push(Manipulator {
                        manipulator_type: "basic".to_string(),
                        from: FromEvent::KeyCode(FromKeyCode {
                            key_code: escape_key,
                            modifiers: Some(FromModifiers {
                                mandatory: None,
                                optional: Some(vec!["any".to_string()]),
                            }),
                        }),
                        to: Some(vec![set_var_off.clone()]),
                        to_if_alone: None,
                        to_if_held_down: None,
                        to_after_key_up: None,
                        to_delayed_action: None,
                        conditions: Some(escape_conditions),
                        parameters: None,
                    });
                }
            }
        }
    }

    for (mapping_idx, mapping) in user_rule.mappings.iter().enumerate() {
        let signal_context = build_signal_context(user_rule, mapping, rule_idx, mapping_idx);
        let manips = convert_mapping(
            mapping,
            simlayer,
            &config.profile,
            base_conditions.clone(),
            leader_enabled,
            leader_sticky,
            &signal_context,
        )?;
        manipulators.extend(manips);
    }

    Ok(Rule {
        description: user_rule.description.clone(),
        manipulators,
    })
}

pub fn to_karabiner_parameters(profile: &ProfileSettings) -> Parameters {
    Parameters {
        simultaneous_threshold: Some(profile.sim),
        to_if_alone_timeout: Some(profile.alone),
        ..Default::default()
    }
}

fn build_base_conditions(condition: &Option<UserCondition>) -> Option<Vec<Condition>> {
    condition.as_ref().map(|c| match c {
        UserCondition::App { app } => {
            vec![Condition::FrontmostAppIf {
                bundle_identifiers: Some(vec![app.clone()]),
                file_paths: None,
            }]
        }
        UserCondition::Apps { apps } => {
            vec![Condition::FrontmostAppIf {
                bundle_identifiers: Some(apps.clone()),
                file_paths: None,
            }]
        }
        UserCondition::AppUnless { app_unless } => {
            vec![Condition::FrontmostAppUnless {
                bundle_identifiers: Some(vec![app_unless.clone()]),
                file_paths: None,
            }]
        }
        UserCondition::AppsUnless { apps_unless } => {
            vec![Condition::FrontmostAppUnless {
                bundle_identifiers: Some(apps_unless.clone()),
                file_paths: None,
            }]
        }
        UserCondition::Device { device } => vec![Condition::DeviceIf {
            identifiers: vec![to_device_identifier(device)],
        }],
        UserCondition::Devices { devices } => vec![Condition::DeviceIf {
            identifiers: devices.iter().map(to_device_identifier).collect(),
        }],
        UserCondition::DeviceUnless { device_unless } => vec![Condition::DeviceUnless {
            identifiers: vec![to_device_identifier(device_unless)],
        }],
        UserCondition::DevicesUnless { devices_unless } => vec![Condition::DeviceUnless {
            identifiers: devices_unless.iter().map(to_device_identifier).collect(),
        }],
        UserCondition::DeviceExists { device_exists } => vec![Condition::DeviceExistsIf {
            identifiers: vec![to_device_identifier(device_exists)],
        }],
        UserCondition::DevicesExists { devices_exists } => vec![Condition::DeviceExistsIf {
            identifiers: devices_exists.iter().map(to_device_identifier).collect(),
        }],
        UserCondition::DeviceExistsUnless {
            device_exists_unless,
        } => vec![Condition::DeviceExistsUnless {
            identifiers: vec![to_device_identifier(device_exists_unless)],
        }],
        UserCondition::DevicesExistsUnless {
            devices_exists_unless,
        } => vec![Condition::DeviceExistsUnless {
            identifiers: devices_exists_unless
                .iter()
                .map(to_device_identifier)
                .collect(),
        }],
        UserCondition::InputSource { input_source } => vec![Condition::InputSourceIf {
            input_sources: vec![to_input_source(input_source)],
        }],
        UserCondition::InputSources { input_sources } => vec![Condition::InputSourceIf {
            input_sources: input_sources.iter().map(to_input_source).collect(),
        }],
        UserCondition::InputSourceUnless {
            input_source_unless,
        } => vec![Condition::InputSourceUnless {
            input_sources: vec![to_input_source(input_source_unless)],
        }],
        UserCondition::InputSourcesUnless {
            input_sources_unless,
        } => vec![Condition::InputSourceUnless {
            input_sources: input_sources_unless.iter().map(to_input_source).collect(),
        }],
        UserCondition::KeyboardType { keyboard_type } => vec![Condition::KeyboardTypeIf {
            keyboard_types: vec![keyboard_type.clone()],
        }],
        UserCondition::KeyboardTypes { keyboard_types } => vec![Condition::KeyboardTypeIf {
            keyboard_types: keyboard_types.clone(),
        }],
        UserCondition::KeyboardTypeUnless {
            keyboard_type_unless,
        } => vec![Condition::KeyboardTypeUnless {
            keyboard_types: vec![keyboard_type_unless.clone()],
        }],
        UserCondition::KeyboardTypesUnless {
            keyboard_types_unless,
        } => vec![Condition::KeyboardTypeUnless {
            keyboard_types: keyboard_types_unless.clone(),
        }],
        UserCondition::Variable { variable, value } => vec![Condition::VariableIf {
            name: variable.clone(),
            value: value.clone(),
        }],
        UserCondition::VariableUnless {
            variable_unless,
            value,
        } => vec![Condition::VariableUnless {
            name: variable_unless.clone(),
            value: value.clone(),
        }],
    })
}

fn to_device_identifier(src: &UserDeviceIdentifier) -> DeviceIdentifier {
    DeviceIdentifier {
        vendor_id: src.vendor_id,
        product_id: src.product_id,
        location_id: src.location_id,
        is_keyboard: src.is_keyboard,
        is_pointing_device: src.is_pointing_device,
        is_game_pad: src.is_game_pad,
        is_consumer: src.is_consumer,
    }
}

fn to_input_source(src: &UserInputSource) -> InputSource {
    InputSource {
        language: src.language.clone(),
        input_source_id: src.input_source_id.clone(),
        input_mode_id: src.input_mode_id.clone(),
    }
}

fn merge_conditions(
    a: Option<Vec<Condition>>,
    b: Option<Vec<Condition>>,
) -> Option<Vec<Condition>> {
    match (a, b) {
        (None, None) => None,
        (Some(conds), None) | (None, Some(conds)) => Some(conds),
        (Some(mut a_conds), Some(mut b_conds)) => {
            a_conds.append(&mut b_conds);
            Some(a_conds)
        }
    }
}

fn simlayer_from_modifiers(layer: &Simlayer, allow_any_fallback: bool) -> FromModifiers {
    let mandatory = layer.modifiers.as_ref().map(|m| m.to_vec());
    let optional = layer.optional.clone().or_else(|| {
        if mandatory.is_none() && allow_any_fallback {
            Some(vec!["any".to_string()])
        } else {
            None
        }
    });
    FromModifiers {
        mandatory,
        optional,
    }
}

fn simlayer_leader_enabled(layer: &Simlayer) -> bool {
    match layer.leader.as_ref() {
        Some(LeaderMode::Enabled(v)) => *v,
        Some(LeaderMode::Config(_)) => true,
        None => false,
    }
}

fn simlayer_leader_sticky(layer: &Simlayer) -> bool {
    match layer.leader.as_ref() {
        Some(LeaderMode::Config(cfg)) => cfg.sticky,
        _ => false,
    }
}

fn simlayer_leader_escape_keys(layer: &Simlayer) -> Vec<String> {
    match layer.leader.as_ref() {
        Some(LeaderMode::Config(cfg)) => cfg
            .escape
            .clone()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| vec!["escape".to_string(), "caps_lock".to_string()]),
        _ => vec!["escape".to_string(), "caps_lock".to_string()],
    }
}

fn mapping_to_delayed_action(
    mapping: &Mapping,
    signal_context: &SignalContext,
) -> Option<ToDelayedAction> {
    mapping.to_delayed.as_ref().map(|d| ToDelayedAction {
        to_if_invoked: convert_to_events(&d.invoked, Some(signal_context)),
        to_if_canceled: convert_to_events(&d.canceled, Some(signal_context)),
    })
}

fn merge_mapping_parameters(
    mapping: &Mapping,
    mut base: Option<ManipulatorParameters>,
) -> Option<ManipulatorParameters> {
    let Some(custom) = mapping.parameters.as_ref() else {
        return base;
    };
    let mut params = base.take().unwrap_or_default();
    if let Some(v) = custom.simultaneous_threshold_ms {
        params.simultaneous_threshold = Some(v);
    }
    if let Some(v) = custom.to_if_alone_timeout_ms {
        params.to_if_alone_timeout = Some(v);
    }
    if let Some(v) = custom.to_if_held_down_threshold_ms {
        params.to_if_held_down_threshold = Some(v);
    }
    if let Some(v) = custom.to_delayed_action_delay_ms {
        params.to_delayed_action_delay = Some(v);
    }
    Some(params)
}

fn convert_mapping(
    mapping: &Mapping,
    simlayer: Option<(&String, &Simlayer)>,
    profile: &ProfileSettings,
    base_conditions: Option<Vec<Condition>>,
    leader_enabled: bool,
    leader_sticky: bool,
    signal_context: &SignalContext,
) -> Result<Vec<Manipulator>> {
    let mut manipulators = Vec::new();

    // Build condition chain: rule -> layer -> mapping
    let mut conditions =
        merge_conditions(base_conditions, build_base_conditions(&mapping.condition));

    let mapping_to_if_alone = mapping
        .to_if_alone
        .as_ref()
        .map(|t| convert_to_events(t, Some(signal_context)));
    let mapping_to_if_held = mapping
        .to_if_held
        .as_ref()
        .map(|t| convert_to_events(t, Some(signal_context)));
    let mapping_to_after_key_up = mapping
        .to_after_key_up
        .as_ref()
        .map(|t| convert_to_events(t, Some(signal_context)));
    let mapping_to_delayed = mapping_to_delayed_action(mapping, signal_context);

    match &mapping.from {
        FromKey::Simultaneous(keys) => {
            // Simultaneous key press (e.g., j+k together)
            let from = FromEvent::Simultaneous(FromSimultaneous {
                simultaneous: keys
                    .iter()
                    .map(|k| SimultaneousKey {
                        key_code: k.clone(),
                    })
                    .collect(),
                simultaneous_options: Some(SimultaneousOptions {
                    // For ad-hoc simultaneous chords (j+k etc), allow small timing noise.
                    // Simlayers have their own stricter settings.
                    detect_key_down_uninterruptedly: Some(false),
                    key_down_order: Some("insensitive".to_string()),
                    key_up_order: Some("insensitive".to_string()),
                    key_up_when: Some("any".to_string()),
                    to_after_key_up: None,
                }),
                modifiers: Some(FromModifiers {
                    optional: Some(vec!["any".to_string()]),
                    mandatory: None,
                }),
            });

            manipulators.push(Manipulator {
                manipulator_type: "basic".to_string(),
                from,
                to: Some(convert_to_events(&mapping.to, Some(signal_context))),
                to_if_alone: mapping_to_if_alone.clone(),
                to_if_held_down: mapping_to_if_held.clone(),
                to_after_key_up: mapping_to_after_key_up.clone(),
                to_delayed_action: mapping_to_delayed.clone(),
                conditions: conditions.clone(),
                parameters: merge_mapping_parameters(
                    mapping,
                    Some(ManipulatorParameters {
                        simultaneous_threshold: Some(profile.sim),
                        to_if_alone_timeout: None,
                        to_if_held_down_threshold: None,
                        to_delayed_action_delay: None,
                    }),
                ),
            });
        }
        FromKey::DoubleTap {
            double_tap,
            modifiers,
            optional,
        } => {
            let from_mods = Some(FromModifiers {
                mandatory: modifiers.as_ref().map(|m| m.to_vec()),
                optional: optional.clone(),
            });
            let from = FromEvent::KeyCode(FromKeyCode {
                key_code: double_tap.clone(),
                modifiers: from_mods.clone(),
            });
            let var_name = format!("double-tap-{}", slug_for_id(&signal_context.mapping_id));

            let mut armed_conditions = conditions.clone().unwrap_or_default();
            armed_conditions.push(Condition::VariableIf {
                name: var_name.clone(),
                value: serde_json::Value::Number(1.into()),
            });
            manipulators.push(Manipulator {
                manipulator_type: "basic".to_string(),
                from: from.clone(),
                to: Some(convert_to_events(&mapping.to, Some(signal_context))),
                to_if_alone: mapping_to_if_alone.clone(),
                to_if_held_down: mapping_to_if_held.clone(),
                to_after_key_up: mapping_to_after_key_up.clone(),
                to_delayed_action: mapping_to_delayed.clone(),
                conditions: Some(armed_conditions),
                parameters: merge_mapping_parameters(mapping, None),
            });

            let single_tap_events = if let Some(to) = &mapping.to_if_single_tap {
                convert_to_events(to, Some(signal_context))
            } else {
                let mandatory = from_mods
                    .as_ref()
                    .and_then(|m| m.mandatory.as_ref())
                    .map(|mods| {
                        mods.iter()
                            .filter(|m| m.as_str() != "any")
                            .cloned()
                            .collect::<Vec<String>>()
                    })
                    .filter(|mods| !mods.is_empty());
                vec![ToEvent::KeyCode(ToKeyCode {
                    key_code: double_tap.clone(),
                    modifiers: mandatory,
                    lazy: None,
                    repeat: None,
                })]
            };
            let delay_ms = mapping
                .double_tap_delay_ms
                .or_else(|| {
                    mapping
                        .parameters
                        .as_ref()
                        .and_then(|p| p.to_delayed_action_delay_ms)
                })
                .unwrap_or(200);
            let mut disarmed_conditions = conditions.clone().unwrap_or_default();
            disarmed_conditions.push(Condition::VariableUnless {
                name: var_name.clone(),
                value: serde_json::Value::Number(1.into()),
            });
            let mut invoked = single_tap_events;
            invoked.push(ToEvent::SetVariable(ToSetVariable {
                set_variable: SetVariable {
                    name: var_name.clone(),
                    value: serde_json::Value::Number(0.into()),
                },
            }));
            let canceled = vec![ToEvent::SetVariable(ToSetVariable {
                set_variable: SetVariable {
                    name: var_name.clone(),
                    value: serde_json::Value::Number(0.into()),
                },
            })];
            manipulators.push(Manipulator {
                manipulator_type: "basic".to_string(),
                from,
                to: Some(vec![ToEvent::SetVariable(ToSetVariable {
                    set_variable: SetVariable {
                        name: var_name,
                        value: serde_json::Value::Number(1.into()),
                    },
                })]),
                to_if_alone: None,
                to_if_held_down: None,
                to_after_key_up: None,
                to_delayed_action: Some(ToDelayedAction {
                    to_if_invoked: invoked,
                    to_if_canceled: canceled,
                }),
                conditions: Some(disarmed_conditions),
                parameters: merge_mapping_parameters(
                    mapping,
                    Some(ManipulatorParameters {
                        simultaneous_threshold: None,
                        to_if_alone_timeout: None,
                        to_if_held_down_threshold: None,
                        to_delayed_action_delay: Some(delay_ms),
                    }),
                ),
            });
        }
        _ => {
            // Single key or key with modifiers
            let (key_code, from_mods) = match &mapping.from {
                FromKey::Simple(key) => (key.clone(), None),
                FromKey::WithModifiers {
                    key,
                    modifiers,
                    optional,
                } => {
                    let mods = FromModifiers {
                        mandatory: modifiers.as_ref().map(|m| m.to_vec()),
                        optional: optional.clone(),
                    };
                    (key.clone(), Some(mods))
                }
                FromKey::DoubleTap { .. } => unreachable!(),
                FromKey::Simultaneous(_) => unreachable!(),
            };

            if let Some((layer_name, layer)) = simlayer {
                // This is a layer-backed mapping.
                let var_name = layer_name.clone();

                // Add layer variable condition
                let layer_condition = Condition::VariableIf {
                    name: var_name.clone(),
                    value: serde_json::Value::Number(1.into()),
                };
                match &mut conditions {
                    Some(conds) => conds.push(layer_condition),
                    None => conditions = Some(vec![layer_condition]),
                }

                // Regular mapping with layer condition (activated when layer is on)
                let from = FromEvent::KeyCode(FromKeyCode {
                    key_code: key_code.clone(),
                    modifiers: from_mods.clone().or(Some(FromModifiers {
                        optional: Some(vec!["any".to_string()]),
                        mandatory: None,
                    })),
                });

                manipulators.push(Manipulator {
                    manipulator_type: "basic".to_string(),
                    from,
                    to: {
                        let mut events = convert_to_events(&mapping.to, Some(signal_context));
                        if leader_enabled && !leader_sticky && layer.mode == SimlayerMode::Hold {
                            events.push(ToEvent::SetVariable(ToSetVariable {
                                set_variable: SetVariable {
                                    name: var_name.clone(),
                                    value: serde_json::Value::Number(0.into()),
                                },
                            }));
                        }
                        Some(events)
                    },
                    to_if_alone: mapping_to_if_alone.clone(),
                    to_if_held_down: mapping_to_if_held.clone(),
                    to_after_key_up: mapping_to_after_key_up.clone(),
                    to_delayed_action: mapping_to_delayed.clone(),
                    conditions: conditions.clone(),
                    parameters: merge_mapping_parameters(mapping, None),
                });

                if layer.mode == SimlayerMode::Simultaneous {
                    // Simultaneous trigger (layer key + this key activates layer)
                    let sim_from = FromEvent::Simultaneous(FromSimultaneous {
                        simultaneous: vec![
                            SimultaneousKey {
                                key_code: layer.key.clone(),
                            },
                            SimultaneousKey {
                                key_code: key_code.clone(),
                            },
                        ],
                        simultaneous_options: Some(SimultaneousOptions {
                            detect_key_down_uninterruptedly: Some(true),
                            key_down_order: Some("strict".to_string()),
                            key_up_order: Some("strict_inverse".to_string()),
                            key_up_when: Some("any".to_string()),
                            to_after_key_up: Some(vec![ToEvent::SetVariable(ToSetVariable {
                                set_variable: SetVariable {
                                    name: var_name.clone(),
                                    value: serde_json::Value::Number(0.into()),
                                },
                            })]),
                        }),
                        modifiers: Some(simlayer_from_modifiers(layer, true)),
                    });

                    let mut to_events = vec![ToEvent::SetVariable(ToSetVariable {
                        set_variable: SetVariable {
                            name: var_name,
                            value: serde_json::Value::Number(1.into()),
                        },
                    })];
                    to_events.extend(convert_to_events(&mapping.to, Some(signal_context)));

                    manipulators.push(Manipulator {
                        manipulator_type: "basic".to_string(),
                        from: sim_from,
                        to: Some(to_events),
                        to_if_alone: None,
                        to_if_held_down: None,
                        to_after_key_up: None,
                        to_delayed_action: None,
                        conditions: None,
                        parameters: merge_mapping_parameters(
                            mapping,
                            Some(ManipulatorParameters {
                                simultaneous_threshold: Some(
                                    layer.threshold.unwrap_or(profile.sim),
                                ),
                                to_if_alone_timeout: None,
                                to_if_held_down_threshold: None,
                                to_delayed_action_delay: None,
                            }),
                        ),
                    });
                }
            } else {
                // Simple mapping without layer
                let from = FromEvent::KeyCode(FromKeyCode {
                    key_code,
                    modifiers: from_mods,
                });

                manipulators.push(Manipulator {
                    manipulator_type: "basic".to_string(),
                    from,
                    to: Some(convert_to_events(&mapping.to, Some(signal_context))),
                    to_if_alone: mapping_to_if_alone,
                    to_if_held_down: mapping_to_if_held,
                    to_after_key_up: mapping_to_after_key_up,
                    to_delayed_action: mapping_to_delayed,
                    conditions,
                    parameters: merge_mapping_parameters(mapping, None),
                });
            }
        }
    }

    Ok(manipulators)
}

#[derive(Debug, Clone)]
struct SignalContext {
    rule_id: String,
    mapping_id: String,
    signal: Option<MappingSignal>,
}

fn slug_for_id(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_dash = false;
    for c in input.chars() {
        let lower = c.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            out.push(lower);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_signal_context(
    user_rule: &UserRule,
    mapping: &Mapping,
    rule_idx: usize,
    mapping_idx: usize,
) -> SignalContext {
    let fallback_rule_id = format!(
        "rule.{}.{}",
        rule_idx + 1,
        slug_for_id(&user_rule.description)
    );
    let rule_id = user_rule.id.clone().unwrap_or(fallback_rule_id);
    let mapping_id = mapping
        .id
        .clone()
        .unwrap_or_else(|| format!("{}.map.{}", rule_id, mapping_idx + 1));
    SignalContext {
        rule_id,
        mapping_id,
        signal: mapping.signal.clone(),
    }
}

fn inject_signal_payload(
    payload: &serde_json::Value,
    signal_context: &SignalContext,
) -> serde_json::Value {
    let serde_json::Value::Object(mut obj) = payload.clone() else {
        return payload.clone();
    };

    let mut meta = serde_json::Map::new();
    meta.insert(
        "rule_id".to_string(),
        serde_json::Value::String(signal_context.rule_id.clone()),
    );
    meta.insert(
        "mapping_id".to_string(),
        serde_json::Value::String(signal_context.mapping_id.clone()),
    );

    if let Some(signal) = signal_context.signal.clone() {
        let signal_json = serde_json::to_value(signal).unwrap_or(serde_json::Value::Null);
        meta.insert("signal".to_string(), signal_json);
    }

    obj.insert("_kar_signal".to_string(), serde_json::Value::Object(meta));
    serde_json::Value::Object(obj)
}

fn key_event(key_code: &str, modifiers: Option<Vec<&str>>) -> ToEvent {
    ToEvent::KeyCode(ToKeyCode {
        key_code: key_code.to_string(),
        modifiers: modifiers.map(|mods| mods.into_iter().map(|m| m.to_string()).collect()),
        lazy: None,
        repeat: None,
    })
}

fn native_key_for_char(ch: char) -> Option<ToEvent> {
    // US-layout fast path for low-latency text entry.
    if ch.is_ascii_lowercase() {
        return Some(key_event(&ch.to_string(), None));
    }
    if ch.is_ascii_uppercase() {
        return Some(key_event(
            &ch.to_ascii_lowercase().to_string(),
            Some(vec!["left_shift"]),
        ));
    }
    if ch.is_ascii_digit() {
        return Some(key_event(&ch.to_string(), None));
    }

    match ch {
        ' ' => Some(key_event("spacebar", None)),
        '\t' => Some(key_event("tab", None)),
        '\n' => Some(key_event("return_or_enter", None)),
        '-' => Some(key_event("hyphen", None)),
        '_' => Some(key_event("hyphen", Some(vec!["left_shift"]))),
        '=' => Some(key_event("equal_sign", None)),
        '+' => Some(key_event("equal_sign", Some(vec!["left_shift"]))),
        '[' => Some(key_event("open_bracket", None)),
        '{' => Some(key_event("open_bracket", Some(vec!["left_shift"]))),
        ']' => Some(key_event("close_bracket", None)),
        '}' => Some(key_event("close_bracket", Some(vec!["left_shift"]))),
        '\\' => Some(key_event("backslash", None)),
        '|' => Some(key_event("backslash", Some(vec!["left_shift"]))),
        ';' => Some(key_event("semicolon", None)),
        ':' => Some(key_event("semicolon", Some(vec!["left_shift"]))),
        '\'' => Some(key_event("quote", None)),
        '"' => Some(key_event("quote", Some(vec!["left_shift"]))),
        '`' => Some(key_event("grave_accent_and_tilde", None)),
        '~' => Some(key_event(
            "grave_accent_and_tilde",
            Some(vec!["left_shift"]),
        )),
        ',' => Some(key_event("comma", None)),
        '<' => Some(key_event("comma", Some(vec!["left_shift"]))),
        '.' => Some(key_event("period", None)),
        '>' => Some(key_event("period", Some(vec!["left_shift"]))),
        '/' => Some(key_event("slash", None)),
        '?' => Some(key_event("slash", Some(vec!["left_shift"]))),
        '!' => Some(key_event("1", Some(vec!["left_shift"]))),
        '@' => Some(key_event("2", Some(vec!["left_shift"]))),
        '#' => Some(key_event("3", Some(vec!["left_shift"]))),
        '$' => Some(key_event("4", Some(vec!["left_shift"]))),
        '%' => Some(key_event("5", Some(vec!["left_shift"]))),
        '^' => Some(key_event("6", Some(vec!["left_shift"]))),
        '&' => Some(key_event("7", Some(vec!["left_shift"]))),
        '*' => Some(key_event("8", Some(vec!["left_shift"]))),
        '(' => Some(key_event("9", Some(vec!["left_shift"]))),
        ')' => Some(key_event("0", Some(vec!["left_shift"]))),
        _ => None,
    }
}

fn native_text_events_from_payload(payload: &serde_json::Value) -> Option<Vec<ToEvent>> {
    const MAX_NATIVE_TEXT_CHARS: usize = 96;
    let obj = payload.as_object()?;
    let ty = obj.get("type")?.as_str()?;
    if ty != "paste_text" && ty != "enter_text" {
        return None;
    }

    let text = obj
        .get("text")
        .and_then(|v| v.as_str())
        .or_else(|| obj.get("arg").and_then(|v| v.as_str()))
        .or_else(|| obj.get("value").and_then(|v| v.as_str()))
        .unwrap_or("");

    // Keep native expansion for short snippets only.
    // Long text is more reliable via clipboard-backed paste/enter fallback.
    if text.chars().count() > MAX_NATIVE_TEXT_CHARS {
        return None;
    }

    if text.is_empty() {
        if ty == "enter_text" {
            return Some(vec![key_event("return_or_enter", None)]);
        }
        return None;
    }

    let mut out = Vec::with_capacity(text.len() + if ty == "enter_text" { 1 } else { 0 });
    for ch in text.chars() {
        out.push(native_key_for_char(ch)?);
    }
    if ty == "enter_text" {
        out.push(key_event("return_or_enter", None));
    }
    Some(out)
}

fn convert_to_events(to: &ToKey, signal_context: Option<&SignalContext>) -> Vec<ToEvent> {
    match to {
        ToKey::Simple(key) => {
            vec![ToEvent::KeyCode(ToKeyCode {
                key_code: key.clone(),
                modifiers: None,
                lazy: None,
                repeat: None,
            })]
        }
        ToKey::WithModifiers { key, modifiers } => {
            vec![ToEvent::KeyCode(ToKeyCode {
                key_code: key.clone(),
                modifiers: modifiers.as_ref().map(|m| m.to_vec()),
                lazy: None,
                repeat: None,
            })]
        }
        ToKey::Shell { shell } => {
            vec![ToEvent::ShellCommand(ToShellCommand {
                shell_command: shell.clone(),
            })]
        }
        ToKey::SocketCommand { socket_command } => {
            vec![ToEvent::SocketCommand(ToSocketCommand {
                socket_command: socket_command.clone(),
            })]
        }
        ToKey::SetVariable { set_variable } => {
            vec![ToEvent::SetVariable(ToSetVariable {
                set_variable: SetVariable {
                    name: set_variable.name.clone(),
                    value: set_variable.value.clone(),
                },
            })]
        }
        ToKey::SendUserCommand { send_user_command } => {
            let payload = if let Some(ctx) = signal_context {
                inject_signal_payload(&send_user_command.payload, ctx)
            } else {
                send_user_command.payload.clone()
            };
            if let Some(native_events) = native_text_events_from_payload(&payload) {
                return native_events;
            }
            vec![ToEvent::SendUserCommand(ToSendUserCommand {
                send_user_command: crate::karabiner::SendUserCommand {
                    payload,
                    endpoint: send_user_command.endpoint.clone(),
                },
            })]
        }
        ToKey::MouseKey { mouse_key } => {
            vec![ToEvent::MouseKey(ToMouseKey {
                mouse_key: crate::karabiner::MouseKey {
                    x: mouse_key.x,
                    y: mouse_key.y,
                    vertical_wheel: mouse_key.vertical_wheel,
                    horizontal_wheel: mouse_key.horizontal_wheel,
                    speed_multiplier: mouse_key.speed_multiplier,
                },
            })]
        }
        ToKey::PointingButton { pointing_button } => {
            vec![ToEvent::PointingButton(ToPointingButton {
                pointing_button: pointing_button.clone(),
                modifiers: None,
            })]
        }
        ToKey::Multiple(keys) => keys
            .iter()
            .flat_map(|k| convert_to_events(k, signal_context))
            .collect(),
    }
}

/// Convert simple modifications from user config to Karabiner format
pub fn to_simple_modifications(config: &UserConfig) -> Vec<SimpleModificationEntry> {
    config
        .simple
        .iter()
        .map(|s| SimpleModificationEntry {
            from: SimpleModificationKey {
                key_code: s.from.clone(),
            },
            to: vec![SimpleModificationKey {
                key_code: s.to.clone(),
            }],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_fields_deserialize_without_changing_behavior() {
        let json = r#"{
          "simple": [
            { "from": "caps_lock", "to": "escape", "note": "tap for escape" }
          ],
          "simlayers": {
            "o-mode": { "key": "o", "threshold": 250, "note": "openers" }
          },
          "rules": [
            {
              "id": "rule.test",
              "description": "test",
              "note": "rule doc",
              "mappings": [
                {
                  "id": "map.open.test",
                  "from": "o",
                  "to": "escape",
                  "note": "opens x front page",
                  "signal": {
                    "intent": "open_test",
                    "tags": ["test", "editor"],
                    "criticality": "med"
                  }
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        assert_eq!(config.rules[0].id.as_deref(), Some("rule.test"));
        assert_eq!(
            config.rules[0].mappings[0].id.as_deref(),
            Some("map.open.test")
        );
        assert_eq!(
            config.rules[0].mappings[0]
                .signal
                .as_ref()
                .and_then(|s| s.intent.as_deref()),
            Some("open_test")
        );
        assert_eq!(
            config.rules[0].mappings[0].note.as_deref(),
            Some("opens x front page")
        );

        let rules = to_karabiner_rules(&config).expect("rules should build");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].manipulators.len(), 1);
    }

    #[test]
    fn send_user_command_payload_includes_signal_context() {
        let json = r#"{
          "rules": [
            {
              "id": "rule.nav",
              "description": "Navigation",
              "mappings": [
                {
                  "id": "map.nav.prompt",
                  "from": "f",
                  "to": {
                    "send_user_command": {
                      "payload": { "action": "predict_next" },
                      "endpoint": "http://127.0.0.1:8780/v1"
                    }
                  },
                  "signal": {
                    "intent": "next_type_prediction",
                    "tags": ["keyboard", "suggestion"],
                    "criticality": "high"
                  }
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let events = rules[0].manipulators[0]
            .to
            .as_ref()
            .expect("to events should exist");
        let ToEvent::SendUserCommand(send_user_command) = &events[0] else {
            panic!("expected send_user_command event");
        };
        let payload = &send_user_command.send_user_command.payload;
        let signal = payload
            .get("_kar_signal")
            .expect("signal context should be added");
        assert_eq!(
            signal.get("rule_id").and_then(|v| v.as_str()),
            Some("rule.nav")
        );
        assert_eq!(
            signal.get("mapping_id").and_then(|v| v.as_str()),
            Some("map.nav.prompt")
        );
        assert_eq!(
            signal
                .get("signal")
                .and_then(|v| v.get("intent"))
                .and_then(|v| v.as_str()),
            Some("next_type_prediction")
        );
    }

    #[test]
    fn send_user_command_non_object_payload_is_left_unchanged() {
        let json = r#"{
          "rules": [
            {
              "description": "Command passthrough",
              "mappings": [
                {
                  "from": "g",
                  "to": {
                    "send_user_command": {
                      "payload": "plain-string",
                      "endpoint": "http://127.0.0.1:8780/v1"
                    }
                  }
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let events = rules[0].manipulators[0]
            .to
            .as_ref()
            .expect("to events should exist");
        let ToEvent::SendUserCommand(send_user_command) = &events[0] else {
            panic!("expected send_user_command event");
        };
        assert_eq!(
            send_user_command.send_user_command.payload.as_str(),
            Some("plain-string")
        );
    }

    #[test]
    fn paste_text_payload_compiles_to_native_key_events() {
        let json = r#"{
          "rules": [
            {
              "description": "Native paste text",
              "mappings": [
                {
                  "from": "l",
                  "to": {
                    "send_user_command": {
                      "payload": { "v": 1, "type": "paste_text", "text": "/prompts:review-push" }
                    }
                  }
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let events = rules[0].manipulators[0]
            .to
            .as_ref()
            .expect("to events should exist");
        assert!(!events.is_empty());
        // "/" then "p"
        let ToEvent::KeyCode(first) = &events[0] else {
            panic!("expected first native key event");
        };
        assert_eq!(first.key_code, "slash");
        let ToEvent::KeyCode(second) = &events[1] else {
            panic!("expected second native key event");
        };
        assert_eq!(second.key_code, "p");
    }

    #[test]
    fn non_ascii_paste_text_payload_falls_back_to_send_user_command() {
        let json = r#"{
          "rules": [
            {
              "description": "Fallback paste text",
              "mappings": [
                {
                  "from": "h",
                  "to": {
                    "send_user_command": {
                      "payload": { "v": 1, "type": "paste_text", "text": "€" }
                    }
                  }
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let events = rules[0].manipulators[0]
            .to
            .as_ref()
            .expect("to events should exist");
        let ToEvent::SendUserCommand(_) = &events[0] else {
            panic!("expected send_user_command fallback");
        };
    }

    #[test]
    fn long_ascii_paste_text_payload_falls_back_to_send_user_command() {
        let long_text = "a".repeat(200);
        let payload = serde_json::json!({
            "v": 1,
            "type": "paste_text",
            "text": long_text,
        });
        let event = convert_to_events(
            &ToKey::SendUserCommand {
                send_user_command: SendUserCommand {
                    payload,
                    endpoint: None,
                },
            },
            None,
        );
        let ToEvent::SendUserCommand(_) = &event[0] else {
            panic!("expected send_user_command fallback for long text");
        };
    }

    #[test]
    fn simlayer_hold_mode_emits_layer_gate_and_conditioned_mapping() {
        let json = r#"{
          "profile": { "alone": 90, "sim": 80 },
          "simlayers": {
            "r-mode": { "key": "r", "mode": "hold", "alone": 120 }
          },
          "rules": [
            {
              "description": "r hold",
              "layer": "r-mode",
              "mappings": [
                { "from": "l", "to": "escape" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].manipulators.len(), 2);

        // Hold-gate manipulator
        let gate = &rules[0].manipulators[0];
        let FromEvent::KeyCode(from_gate) = &gate.from else {
            panic!("expected key gate");
        };
        assert_eq!(from_gate.key_code, "r");
        let gate_to_if_alone = gate.to_if_alone.as_ref().expect("gate to_if_alone");
        let ToEvent::KeyCode(key) = &gate_to_if_alone[0] else {
            panic!("expected keycode to_if_alone");
        };
        assert_eq!(key.key_code, "r");
        assert_eq!(
            gate.parameters.as_ref().and_then(|p| p.to_if_alone_timeout),
            Some(120)
        );

        // Layer-conditioned mapping (no simultaneous trigger in hold mode)
        let map = &rules[0].manipulators[1];
        let FromEvent::KeyCode(from_map) = &map.from else {
            panic!("expected key mapping");
        };
        assert_eq!(from_map.key_code, "l");
        let has_layer_cond = map.conditions.as_ref().is_some_and(|conds| {
            conds.iter().any(|c| {
                matches!(
                    c,
                    Condition::VariableIf { name, value }
                    if name == "r-mode" && value.as_i64() == Some(1)
                )
            })
        });
        assert!(has_layer_cond, "mapping must be gated by r-mode variable");
        assert!(rules[0]
            .manipulators
            .iter()
            .all(|m| !matches!(m.from, FromEvent::Simultaneous(_))));
    }

    #[test]
    fn simlayer_default_mode_remains_simultaneous() {
        let json = r#"{
          "profile": { "alone": 90, "sim": 80 },
          "simlayers": {
            "r-mode": { "key": "r", "threshold": 250 }
          },
          "rules": [
            {
              "description": "r sim",
              "layer": "r-mode",
              "mappings": [
                { "from": "l", "to": "escape" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].manipulators.len(), 2);
        assert!(rules[0]
            .manipulators
            .iter()
            .any(|m| matches!(m.from, FromEvent::Simultaneous(_))));
    }

    #[test]
    fn simlayer_with_modifiers_and_layer_condition_propagates_to_manipulators() {
        let json = r#"{
          "profile": { "alone": 90, "sim": 80 },
          "simlayers": {
            "w-mode": {
              "key": "w",
              "modifiers": "left_control",
              "condition": { "app": "^dev\\.zed\\.Zed$" }
            }
          },
          "rules": [
            {
              "description": "w sim + zed only",
              "layer": "w-mode",
              "condition": { "variable": "ctx", "value": 1 },
              "mappings": [
                { "from": "e", "to": "tab" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].manipulators.len(), 2);

        let layer_mapping = &rules[0].manipulators[0];
        let conds = layer_mapping.conditions.as_ref().expect("layer conditions");
        assert!(conds.iter().any(|c| {
            matches!(
                c,
                Condition::VariableIf { name, value } if name == "ctx" && value.as_i64() == Some(1)
            )
        }));
        assert!(conds.iter().any(|c| {
            matches!(
                c,
                Condition::FrontmostAppIf { bundle_identifiers, .. }
                if bundle_identifiers
                    .as_ref()
                    .is_some_and(|v| v.iter().any(|s| s == "^dev\\.zed\\.Zed$"))
            )
        }));
        assert!(conds.iter().any(|c| {
            matches!(
                c,
                Condition::VariableIf { name, value } if name == "w-mode" && value.as_i64() == Some(1)
            )
        }));

        let sim = rules[0]
            .manipulators
            .iter()
            .find(|m| matches!(m.from, FromEvent::Simultaneous(_)))
            .expect("simultaneous trigger");
        let FromEvent::Simultaneous(sim_from) = &sim.from else {
            panic!("expected simultaneous from");
        };
        let mandatory = sim_from
            .modifiers
            .as_ref()
            .and_then(|m| m.mandatory.as_ref())
            .expect("mandatory modifiers");
        assert_eq!(mandatory, &vec!["left_control".to_string()]);
    }

    #[test]
    fn apps_condition_maps_to_frontmost_application_if() {
        let json = r#"{
          "rules": [
            {
              "description": "Scoped by multiple apps",
              "condition": {
                "apps": ["^com\\.apple\\.Terminal$", "^dev\\.zed\\.Zed$"]
              },
              "mappings": [
                { "from": "j", "to": "down_arrow" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::FrontmostAppIf {
            bundle_identifiers, ..
        } = &conds[0]
        else {
            panic!("expected frontmost_application_if");
        };
        assert_eq!(bundle_identifiers.as_ref().expect("bundle ids").len(), 2);
    }

    #[test]
    fn app_unless_condition_maps_to_frontmost_application_unless() {
        let json = r#"{
          "rules": [
            {
              "description": "Scoped by app unless",
              "condition": {
                "app_unless": "^com\\.apple\\.Xcode$"
              },
              "mappings": [
                { "from": "k", "to": "up_arrow" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::FrontmostAppUnless {
            bundle_identifiers, ..
        } = &conds[0]
        else {
            panic!("expected frontmost_application_unless");
        };
        assert_eq!(
            bundle_identifiers
                .as_ref()
                .and_then(|v| v.first())
                .map(String::as_str),
            Some("^com\\.apple\\.Xcode$")
        );
    }

    #[test]
    fn device_condition_maps_to_device_if() {
        let json = r#"{
          "rules": [
            {
              "description": "device scoped",
              "condition": {
                "device": { "vendor_id": 1452, "product_id": 832 }
              },
              "mappings": [
                { "from": "j", "to": "down_arrow" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::DeviceIf { identifiers } = &conds[0] else {
            panic!("expected device_if");
        };
        assert_eq!(identifiers.len(), 1);
        assert_eq!(identifiers[0].vendor_id, Some(1452));
    }

    #[test]
    fn input_source_unless_maps_to_input_source_unless() {
        let json = r#"{
          "rules": [
            {
              "description": "input source scoped",
              "condition": {
                "input_source_unless": { "language": "en" }
              },
              "mappings": [
                { "from": "k", "to": "up_arrow" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::InputSourceUnless { input_sources } = &conds[0] else {
            panic!("expected input_source_unless");
        };
        assert_eq!(
            input_sources.first().and_then(|s| s.language.as_deref()),
            Some("en")
        );
    }

    #[test]
    fn keyboard_type_condition_maps_to_keyboard_type_if() {
        let json = r#"{
          "rules": [
            {
              "description": "keyboard type scoped",
              "condition": { "keyboard_type": "ansi" },
              "mappings": [
                { "from": "l", "to": "right_arrow" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::KeyboardTypeIf { keyboard_types } = &conds[0] else {
            panic!("expected keyboard_type_if");
        };
        assert_eq!(keyboard_types, &vec!["ansi".to_string()]);
    }

    #[test]
    fn variable_unless_condition_maps_to_variable_unless() {
        let json = r#"{
          "rules": [
            {
              "description": "variable unless scoped",
              "condition": { "variable_unless": "blocked", "value": 1 },
              "mappings": [
                { "from": "spacebar", "to": "tab" }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let conds = rules[0].manipulators[0]
            .conditions
            .as_ref()
            .expect("conditions");
        let Condition::VariableUnless { name, value } = &conds[0] else {
            panic!("expected variable_unless");
        };
        assert_eq!(name, "blocked");
        assert_eq!(value.as_i64(), Some(1));
    }

    #[test]
    fn double_tap_from_builds_dual_manipulators_with_delay() {
        let json = r#"{
          "rules": [
            {
              "description": "double tap",
              "mappings": [
                {
                  "id": "map.double.tap",
                  "from": { "double_tap": "q", "modifiers": "left_command" },
                  "to": "escape",
                  "double_tap_delay_ms": 150
                }
              ]
            }
          ]
        }"#;

        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        assert_eq!(rules[0].manipulators.len(), 2);
        let has_action = rules[0].manipulators.iter().any(|m| {
            m.conditions.as_ref().is_some_and(|conds| {
                conds
                    .iter()
                    .any(|c| matches!(c, Condition::VariableIf { .. }))
            }) && m.to.as_ref().is_some_and(|to| {
                matches!(
                    to.first(),
                    Some(ToEvent::KeyCode(ToKeyCode { key_code, .. })) if key_code == "escape"
                )
            })
        });
        assert!(has_action, "double tap action manipulator missing");
        let has_toggle = rules[0].manipulators.iter().any(|m| {
            m.to_delayed_action
                .as_ref()
                .is_some_and(|d| !d.to_if_invoked.is_empty() && !d.to_if_canceled.is_empty())
                && m.parameters
                    .as_ref()
                    .and_then(|p| p.to_delayed_action_delay)
                    == Some(150)
        });
        assert!(has_toggle, "double tap toggle manipulator missing");
    }

    #[test]
    fn hold_layer_leader_mode_adds_escape_and_keeps_layer_on_key_up() {
        let json = r#"{
          "simlayers": {
            "r-mode": { "key": "r", "mode": "hold", "leader": true }
          },
          "rules": [
            {
              "description": "leader",
              "layer": "r-mode",
              "mappings": [
                { "from": "l", "to": "right_arrow" }
              ]
            }
          ]
        }"#;
        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        // gate + 2 escapes + 1 mapping
        assert_eq!(rules[0].manipulators.len(), 4);
        let gate = &rules[0].manipulators[0];
        assert!(
            gate.to_after_key_up.is_none(),
            "leader gate should not auto-disable on key up"
        );
        let has_escape = rules[0].manipulators.iter().any(|m| {
            matches!(
                m.from,
                FromEvent::KeyCode(FromKeyCode { ref key_code, .. }) if key_code == "escape"
            )
        });
        assert!(has_escape, "leader escape manipulator missing");
    }

    #[test]
    fn hold_layer_delay_activates_on_hold_threshold() {
        let json = r#"{
          "simlayers": {
            "r-mode": { "key": "r", "mode": "hold", "delay_ms": 140 }
          },
          "rules": [
            {
              "description": "delayed hold",
              "layer": "r-mode",
              "mappings": [
                { "from": "h", "to": "left_arrow" }
              ]
            }
          ]
        }"#;
        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let gate = &rules[0].manipulators[0];
        assert!(
            gate.to.is_none(),
            "delayed hold gate should not activate on key down"
        );
        assert!(
            gate.to_if_held_down.is_some(),
            "delayed hold gate should activate on hold"
        );
        assert_eq!(
            gate.parameters
                .as_ref()
                .and_then(|p| p.to_if_held_down_threshold),
            Some(140)
        );
    }

    #[test]
    fn import_json_rules_are_loaded() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let import_path = std::env::temp_dir().join(format!("kar_import_rules_{ts}.json"));
        let content = r#"{
          "rules": [
            {
              "description": "imported",
              "manipulators": [
                { "type": "basic", "from": { "key_code": "a" }, "to": [{ "key_code": "b" }] }
              ]
            }
          ]
        }"#;
        std::fs::write(&import_path, content).expect("write import");

        let cfg_json = format!(
            r#"{{
              "imports": [{{ "import_json": "{}" }}],
              "rules": []
            }}"#,
            import_path.display()
        );
        let config: UserConfig = serde_json::from_str(&cfg_json).expect("valid config");
        let config_path = std::env::temp_dir().join(format!("kar_cfg_{ts}.ts"));
        let loaded =
            load_imported_rules(&config, &config_path, &import_path).expect("load imports");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].description, "imported");

        let _ = std::fs::remove_file(import_path);
    }

    #[test]
    fn import_profile_rules_are_loaded() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let karabiner_path = std::env::temp_dir().join(format!("kar_profile_import_{ts}.json"));
        let content = r#"{
          "profiles": [
            {
              "name": "src",
              "selected": false,
              "complex_modifications": {
                "rules": [
                  {
                    "description": "from profile",
                    "manipulators": [
                      { "type": "basic", "from": { "key_code": "x" }, "to": [{ "key_code": "y" }] }
                    ]
                  }
                ]
              }
            }
          ]
        }"#;
        std::fs::write(&karabiner_path, content).expect("write karabiner");

        let cfg_json = format!(
            r#"{{
              "imports": [{{ "import_profile": "src", "karabiner_json": "{}" }}],
              "rules": []
            }}"#,
            karabiner_path.display()
        );
        let config: UserConfig = serde_json::from_str(&cfg_json).expect("valid config");
        let config_path = std::env::temp_dir().join(format!("kar_cfg_profile_{ts}.ts"));
        let loaded =
            load_imported_rules(&config, &config_path, &karabiner_path).expect("load profile");
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].description, "from profile");

        let _ = std::fs::remove_file(karabiner_path);
    }

    #[test]
    fn to_key_set_variable_compiles_to_set_variable_event() {
        let json = r#"{
          "rules": [
            {
              "description": "set var",
              "mappings": [
                { "from": "a", "to": { "set_variable": { "name": "x", "value": 1 } } }
              ]
            }
          ]
        }"#;
        let config: UserConfig = serde_json::from_str(json).expect("valid config");
        let rules = to_karabiner_rules(&config).expect("rules should build");
        let events = rules[0].manipulators[0].to.as_ref().expect("to events");
        let ToEvent::SetVariable(set_var) = &events[0] else {
            panic!("expected set_variable event");
        };
        assert_eq!(set_var.set_variable.name, "x");
    }
}
