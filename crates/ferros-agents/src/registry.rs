use std::collections::BTreeMap;
use std::fmt;

use crate::manifest::{AgentManifest, AgentName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSummary {
    pub name: AgentName,
    pub version: String,
}

pub trait AgentRegistry {
    type Error;

    fn register(&mut self, manifest: AgentManifest) -> Result<(), Self::Error>;
    fn deregister(&mut self, name: &AgentName) -> Result<Option<AgentManifest>, Self::Error>;
    fn list(&self) -> Vec<AgentSummary>;
    fn describe(&self, name: &AgentName) -> Result<Option<AgentManifest>, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    AlreadyRegistered(AgentName),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered(name) => {
                write!(f, "agent {} is already registered", name.as_str())
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct InMemoryAgentRegistry {
    manifests: BTreeMap<AgentName, AgentManifest>,
}

impl AgentRegistry for InMemoryAgentRegistry {
    type Error = RegistryError;

    fn register(&mut self, manifest: AgentManifest) -> Result<(), Self::Error> {
        let name = manifest.name.clone();
        if self.manifests.contains_key(&name) {
            return Err(RegistryError::AlreadyRegistered(name));
        }

        self.manifests.insert(name, manifest);
        Ok(())
    }

    fn deregister(&mut self, name: &AgentName) -> Result<Option<AgentManifest>, Self::Error> {
        Ok(self.manifests.remove(name))
    }

    fn list(&self) -> Vec<AgentSummary> {
        self.manifests
            .values()
            .map(|manifest| AgentSummary {
                name: manifest.name.clone(),
                version: manifest.version.clone(),
            })
            .collect()
    }

    fn describe(&self, name: &AgentName) -> Result<Option<AgentManifest>, Self::Error> {
        Ok(self.manifests.get(name).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentRegistry, InMemoryAgentRegistry, RegistryError};
    use crate::manifest::{AgentManifest, AgentName};

    fn manifest(name: &str, version: &str) -> AgentManifest {
        AgentManifest::new(
            AgentName::new(name).expect("valid agent name"),
            version,
            Vec::new(),
        )
    }

    #[test]
    fn registry_rejects_duplicate_names() {
        let mut registry = InMemoryAgentRegistry::default();
        let manifest = manifest("echo", "0.1.0");

        registry
            .register(manifest.clone())
            .expect("initial register should succeed");

        let error = registry.register(manifest).expect_err("duplicate should fail");
        assert_eq!(
            error,
            RegistryError::AlreadyRegistered(
                AgentName::new("echo").expect("valid agent name")
            )
        );
    }

    #[test]
    fn registry_lists_agents_in_deterministic_name_order() {
        let mut registry = InMemoryAgentRegistry::default();
        registry
            .register(manifest("timer", "0.2.0"))
            .expect("timer should register");
        registry
            .register(manifest("echo", "0.1.0"))
            .expect("echo should register");

        let names: Vec<String> = registry
            .list()
            .into_iter()
            .map(|summary| summary.name.as_str().to_owned())
            .collect();

        assert_eq!(names, vec!["echo".to_string(), "timer".to_string()]);
    }

    #[test]
    fn registry_deregister_removes_manifest_from_future_queries() {
        let mut registry = InMemoryAgentRegistry::default();
        let echo = manifest("echo", "0.1.0");
        let name = echo.name.clone();

        registry
            .register(echo.clone())
            .expect("echo should register");

        let removed = registry
            .deregister(&name)
            .expect("deregister should succeed");

        assert_eq!(removed, Some(echo));
        assert_eq!(registry.describe(&name).expect("describe should succeed"), None);
        assert!(registry.list().is_empty());
    }
}