use std::collections::HashMap;
use std::fmt::{Display, Formatter};


///
/// Config Structs
/// Config is the top level element that operators should work with and add settings to
/// For this example I implemented default for it to simulate the default values for settings,
/// but later on this would read from a config map or something similar
#[derive(Clone, Debug)]
struct Config {
    config: HashMap<String, Vec<ConfigEntry>>,
}

#[derive(Clone, Debug)]
struct ConfigEntry {
    key: String,
    value: ConfigValue,
    reason: Option<String>,
}

#[derive(Clone, Debug)]
enum ConfigValue {
    StringValue { value: String },
    NumericValue { value: u8 },
}

impl Display for ConfigValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConfigValue::StringValue { value } => {
                write!(f, "{}", value)
            }
            ConfigValue::NumericValue { value } => {
                write!(f, "{}", value)
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut conf = Config::new();

        conf.set_value(ConfigEntry {
            key: SETTING_1.to_string(),
            value: ConfigValue::StringValue {
                value: SETTING_1_DEFAULT.to_string(),
            },
            reason: Some("Default value from product config applied.".to_string()),
        });

        conf.set_value(ConfigEntry {
            key: SETTING_2.to_string(),
            value: ConfigValue::NumericValue {
                value: SETTING_2_DEFAULT,
            },
            reason: Some("Default value from product config applied.".to_string()),
        });

        conf
    }
}

/// Implementation for config to be used by operators
/// Mostly just exposes functions to set one value or multiple values.
///
/// At the end, this can then be transformed to a properties file
/// I'd expect that we offer a few default output formats in the framework, but allow
/// operators to build more esoteric formats themselves.
impl Config {
    pub fn new() -> Self {
        Config {
            config: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, entry: ConfigEntry) {
        self.config
            .entry(entry.key.clone())
            .or_insert_with(|| Vec::new())
            .push(entry);
    }

    pub fn set_multiple_values(&mut self, entries: Vec<ConfigEntry>) {
        for entry in entries {
            self.set_value(entry);
        }
    }

    pub fn to_properties_file(&self) -> String {
        let mut result = String::new();

        for (parameter_name, parameter_values) in &self.config {
            if parameter_values.len() > 1 {
                result.push_str(&format!("# NOTE: [{}] has been defined multiple times during generation of the config, please review for potential conflicts!\n", parameter_name));
                for entry in parameter_values {
                    result.push_str(&format!(
                        "# Value: [{}], Reason: {}\n",
                        entry.value,
                        entry
                            .reason
                            .clone()
                            .unwrap_or_else(|| { "no reason specified".to_string() })
                    ));
                }
            }
            result.push_str(&format!(
                "{}: {}\n",
                parameter_name,
                parameter_values
                    .get(parameter_values.len() - 1)
                    .unwrap()
                    .value
            ));
            result.push_str("\n");
        }

        result
    }
}

// Just some default settings to make the code a bit more readable
static SETTING_1: &str = "setting1";
static SETTING_1_DEFAULT: &str = "insecure";

static SETTING_2: &str = "setting2";
static SETTING_2_DEFAULT: u8 = 124;

static SETTING_3: &str = "setting3";

static SETTING_4: &str = "setting4";


// Simulated overrides, in the real world these would come from the CR objects
fn get_overrides() -> Vec<ConfigEntry> {
    let mut overrides = Vec::new();

    overrides.push(ConfigEntry {
        key: SETTING_3.to_string(),
        value: ConfigValue::NumericValue { value: 20 },
        reason: Some("Overridden by user in rolegroup settings.".to_string()),
    });

    overrides
}


/// Example code
/// Generate config element with default values set, explicitly configure some, then apply overrides
/// and generate a properties file
fn main() {
    println!("Built the following config:");

    let mut config = Config::default();

    // Set values
    config.set_value(ConfigEntry {
        key: SETTING_1.to_string(),
        value: ConfigValue::StringValue {
            value: "secure".to_string(),
        },
        reason: Some("Needed because CRD contained TLS config.".to_string()),
    });

    config.set_value(ConfigEntry {
        key: SETTING_3.to_string(),
        value: ConfigValue::NumericValue { value: 4 },
        reason: Some("Best practices says to not have this higher than 4".to_string()),
    });

    config.set_value(ConfigEntry {
        key: SETTING_4.to_string(),
        value: ConfigValue::NumericValue { value: 10 },
        reason: Some(
            "User configured more than 4 disks, so we need more processes to fully utilize those."
                .to_string(),
        ),
    });

    // Apply overrides
    config.set_multiple_values(get_overrides());

    println!("{}", config.to_properties_file());
}
