use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Confiq {
    pub aliases: Vec<Alias>,
    pub scopes: HashMap<String, ScopeDescriptor>,
    pub environment_variables: Vec<EnvironmentVariable>,
    pub scripts: Vec<Scripts>,
}

#[derive(Deserialize, Debug)]
pub struct Scripts {
    pub path: String,
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Deserialize, Debug)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Deserialize, Debug)]
pub struct Alias {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Deserialize, Debug)]
pub struct Scope(pub String);

impl Default for Scope {
    fn default() -> Self {
        Scope(String::from(".*"))
    }
}

#[derive(Deserialize, Debug)]
pub struct ScopeDescriptor {
    pub whoami: String,
}

