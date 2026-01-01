use crate::karabiner::{
    Condition, FromEvent, FromKeyCode, FromModifiers, FromSimultaneous, Manipulator,
    ManipulatorParameters, Rule, SetVariable, SimpleModificationEntry, SimpleModificationKey,
    SimultaneousKey, SimultaneousOptions, ToEvent, ToKeyCode, ToMouseKey, ToSetVariable,
    ToShellCommand,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRule {
    pub description: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub condition: Option<UserCondition>,
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserCondition {
    App { app: String },
    Variable { variable: String, value: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub from: FromKey,
    pub to: ToKey,
    #[serde(default)]
    pub to_if_alone: Option<ToKey>,
    #[serde(default)]
    pub to_if_held: Option<ToKey>,
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
    MouseKey {
        mouse_key: UserMouseKey,
    },
    Multiple(Vec<ToKey>),
}

/// Convert user config to Karabiner rules
pub fn to_karabiner_rules(config: &UserConfig) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();

    for user_rule in &config.rules {
        let rule = convert_rule(user_rule, config)?;
        rules.push(rule);
    }

    Ok(rules)
}

fn convert_rule(user_rule: &UserRule, config: &UserConfig) -> Result<Rule> {
    let mut manipulators = Vec::new();

    // Check if this rule uses a simlayer
    let simlayer = user_rule
        .layer
        .as_ref()
        .and_then(|name| config.simlayers.get(name).map(|s| (name, s)));

    for mapping in &user_rule.mappings {
        let manips = convert_mapping(mapping, simlayer, &config.profile, &user_rule.condition)?;
        manipulators.extend(manips);
    }

    Ok(Rule {
        description: user_rule.description.clone(),
        manipulators,
    })
}

fn convert_mapping(
    mapping: &Mapping,
    simlayer: Option<(&String, &Simlayer)>,
    profile: &ProfileSettings,
    condition: &Option<UserCondition>,
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
                simultaneous: keys.iter().map(|k| SimultaneousKey { key_code: k.clone() }).collect(),
                simultaneous_options: Some(SimultaneousOptions {
                    detect_key_down_uninterruptedly: Some(true),
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
                to: Some(convert_to_events(&mapping.to)),
                to_if_alone: mapping.to_if_alone.as_ref().map(|t| convert_to_events(t)),
                to_if_held_down: mapping.to_if_held.as_ref().map(|t| convert_to_events(t)),
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
                FromKey::WithModifiers { key, modifiers, optional } => {
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
                    to: Some(convert_to_events(&mapping.to)),
                    to_if_alone: mapping.to_if_alone.as_ref().map(|t| convert_to_events(t)),
                    to_if_held_down: mapping.to_if_held.as_ref().map(|t| convert_to_events(t)),
                    to_after_key_up: None,
                    conditions: conditions.clone(),
                    parameters: None,
                });

                // Simultaneous trigger (layer key + this key activates layer)
                let sim_from = FromEvent::Simultaneous(FromSimultaneous {
                    simultaneous: vec![
                        SimultaneousKey { key_code: layer.key.clone() },
                        SimultaneousKey { key_code: key_code.clone() },
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
                to_events.extend(convert_to_events(&mapping.to));

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
                    to: Some(convert_to_events(&mapping.to)),
                    to_if_alone: mapping.to_if_alone.as_ref().map(|t| convert_to_events(t)),
                    to_if_held_down: mapping.to_if_held.as_ref().map(|t| convert_to_events(t)),
                    to_after_key_up: None,
                    conditions,
                    parameters: None,
                });
            }
        }
    }

    Ok(manipulators)
}

fn convert_to_events(to: &ToKey) -> Vec<ToEvent> {
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
        ToKey::Multiple(keys) => keys.iter().flat_map(convert_to_events).collect(),
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
