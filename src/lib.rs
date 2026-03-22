use serde::{Deserialize, Serialize};

/// Configuration for creating an emulator instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmulatorConfig {
    pub name: String,
    pub api_level: u32,
    pub abi: String,
    pub ram_mb: u32,
    pub storage_mb: u32,
}

/// Runtime state of an emulator instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmulatorState {
    pub name: String,
    pub running: bool,
    pub port: Option<u16>,
    pub pid: Option<u32>,
}

/// Generate a deterministic AVD name from an emulator config.
///
/// Format: `{name}_api{api_level}_{abi}`
#[must_use]
pub fn avd_name(config: &EmulatorConfig) -> String {
    format!("{}_api{}_{}", config.name, config.api_level, config.abi)
}

/// Create a default emulator config for a given API level.
#[must_use]
pub fn default_config(api_level: u32) -> EmulatorConfig {
    EmulatorConfig {
        name: format!("emyu_api{api_level}"),
        api_level,
        abi: "x86_64".to_string(),
        ram_mb: 2048,
        storage_mb: 4096,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avd_name_generation() {
        let config = EmulatorConfig {
            name: "test".to_string(),
            api_level: 34,
            abi: "arm64-v8a".to_string(),
            ram_mb: 2048,
            storage_mb: 4096,
        };
        assert_eq!(avd_name(&config), "test_api34_arm64-v8a");
    }

    #[test]
    fn default_config_api34() {
        let config = default_config(34);
        assert_eq!(config.name, "emyu_api34");
        assert_eq!(config.api_level, 34);
        assert_eq!(config.abi, "x86_64");
        assert_eq!(config.ram_mb, 2048);
        assert_eq!(config.storage_mb, 4096);
    }

    #[test]
    fn default_config_api35() {
        let config = default_config(35);
        assert_eq!(config.name, "emyu_api35");
        assert_eq!(config.api_level, 35);
    }

    #[test]
    fn state_serialization() {
        let state = EmulatorState {
            name: "test_api34_x86_64".to_string(),
            running: true,
            port: Some(5554),
            pid: Some(12345),
        };
        let json = serde_json::to_string(&state).unwrap();
        let deser: EmulatorState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deser);
    }

    #[test]
    fn config_serialization() {
        let config = default_config(34);
        let json = serde_json::to_string(&config).unwrap();
        let deser: EmulatorConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deser);
    }
}
