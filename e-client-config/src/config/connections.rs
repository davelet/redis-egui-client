use crate::connection::RedisConnectionConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ConfigOnConnections {
    pub connections: Vec<RedisConnectionConfig>,
}

impl ConfigOnConnections {
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }

    pub(crate) fn push(&mut self, config: RedisConnectionConfig) {
        self.connections.push(config)
    }

    pub(crate) fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&RedisConnectionConfig) -> bool,
    {
        self.connections.retain(f)
    }

    pub fn get(&self, index: usize) -> Option<&RedisConnectionConfig> {
        self.connections.get(index)
    }
}

impl<'a> IntoIterator for &'a ConfigOnConnections {
    type Item = &'a RedisConnectionConfig;
    type IntoIter = std::slice::Iter<'a, RedisConnectionConfig>;

    fn into_iter(self) -> Self::IntoIter {
        self.connections.iter()
        // self.connections.iter_mut()
        // self.connections.into_iter()
    }
}
