use crate::karabiner::{
    Condition, FromEvent, FromKeyCode, FromModifiers, FromSimultaneous, Manipulator,
    ManipulatorParameters, Parameters, Rule, SetVariable, SimpleModificationEntry,
    SimpleModificationKey, SimultaneousKey, SimultaneousOptions, SocketCommand, ToEvent, ToKeyCode,
    ToMouseKey, ToPointingButton, ToSendUserCommand, ToSetVariable, ToShellCommand,
    ToSocketCommand,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub rules: Vec<UserRule>,
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
    pub threshold: Option<u32>,
    #[serde(default)]
    pub note: Option<String>,
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
    Variable {
        variable: String,
        value: serde_json::Value,
    },
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
    pub signal: Option<MappingSignal>,
    #[serde(default)]
    pub note: Option<String>,
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

/// Convert user config to Karabiner rules
pub fn to_karabiner_rules(config: &UserConfig) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for (rule_idx, user_rule) in config.rules.iter().enumerate() {
        let rule = convert_rule(user_rule, config, rule_idx)?;
        rules.push(rule);
    }

    Ok(rules)
}

fn convert_rule(user_rule: &UserRule, config: &UserConfig, rule_idx: usize) -> Result<Rule> {
    let mut manipulators = Vec::new();

    // Check if this rule uses a simlayer
    let simlayer = user_rule
        .layer
        .as_ref()
        .and_then(|name| config.simlayers.get(name).map(|s| (name, s)));

    for (mapping_idx, mapping) in user_rule.mappings.iter().enumerate() {
        let signal_context = build_signal_context(user_rule, mapping, rule_idx, mapping_idx);
        let manips = convert_mapping(
            mapping,
            simlayer,
            &config.profile,
            &user_rule.condition,
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

fn convert_mapping(
    mapping: &Mapping,
    simlayer: Option<(&String, &Simlayer)>,
    profile: &ProfileSettings,
    condition: &Option<UserCondition>,
    signal_context: &SignalContext,
) -> Result<Vec<Manipulator>> {
    let mut manipulators = Vec::new();

    // Build base condition from user condition
    let mut conditions: Option<Vec<Condition>> = condition.as_ref().map(|c| match c {
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
        UserCondition::Variable { variable, value } => {
            vec![Condition::VariableIf {
                name: variable.clone(),
                value: value.clone(),
            }]
        }
    });

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
                to_if_alone: mapping
                    .to_if_alone
                    .as_ref()
                    .map(|t| convert_to_events(t, Some(signal_context))),
                to_if_held_down: mapping
                    .to_if_held
                    .as_ref()
                    .map(|t| convert_to_events(t, Some(signal_context))),
                to_after_key_up: None,
                conditions: conditions.clone(),
                parameters: Some(ManipulatorParameters {
                    simultaneous_threshold: Some(profile.sim),
                }),
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
                FromKey::Simultaneous(_) => unreachable!(),
            };

            if let Some((layer_name, layer)) = simlayer {
                // This is a simlayer mapping - create simultaneous triggers
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
                    to: Some(convert_to_events(&mapping.to, Some(signal_context))),
                    to_if_alone: mapping
                        .to_if_alone
                        .as_ref()
                        .map(|t| convert_to_events(t, Some(signal_context))),
                    to_if_held_down: mapping
                        .to_if_held
                        .as_ref()
                        .map(|t| convert_to_events(t, Some(signal_context))),
                    to_after_key_up: None,
                    conditions: conditions.clone(),
                    parameters: None,
                });

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
                    modifiers: Some(FromModifiers {
                        optional: Some(vec!["any".to_string()]),
                        mandatory: None,
                    }),
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
                    conditions: None,
                    parameters: Some(ManipulatorParameters {
                        simultaneous_threshold: Some(layer.threshold.unwrap_or(profile.sim)),
                    }),
                });
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
                    to_if_alone: mapping
                        .to_if_alone
                        .as_ref()
                        .map(|t| convert_to_events(t, Some(signal_context))),
                    to_if_held_down: mapping
                        .to_if_held
                        .as_ref()
                        .map(|t| convert_to_events(t, Some(signal_context))),
                    to_after_key_up: None,
                    conditions,
                    parameters: None,
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
        ToKey::SendUserCommand { send_user_command } => {
            let payload = if let Some(ctx) = signal_context {
                inject_signal_payload(&send_user_command.payload, ctx)
            } else {
                send_user_command.payload.clone()
            };
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
        assert_eq!(
            bundle_identifiers.as_ref().expect("bundle ids").len(),
            2
        );
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
}
