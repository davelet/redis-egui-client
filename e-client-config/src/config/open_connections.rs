use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OpenConnections {
    #[serde(default)]
    pub connection_names: Vec<String>,
}

impl OpenConnections {
    pub fn is_empty(&self) -> bool {
        self.connection_names.is_empty()
    }

    pub fn add_connection(&mut self, name: &str) {
        if !self.connection_names.contains(&name.to_string()) {
            self.connection_names.push(name.to_string());
        }
    }

    pub fn remove_connection(&mut self, name: &str) {
        self.connection_names.retain(|n| n != name);
    }

    pub fn clear(&mut self) {
        self.connection_names.clear();
    }

    pub fn set_connections(&mut self, names: Vec<String>) {
        self.connection_names = names;
    }
}
