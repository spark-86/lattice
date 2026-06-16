use std::collections::HashMap;

use anyhow::Result;

use crate::{context::TransformContext, loaded::LoadedTransform};

pub type Registry = HashMap<[u8; 32], LoadedTransform>;

pub type RtMapping = HashMap<String, Vec<[u8; 32]>>;

pub struct TransformRegistry {
    pub registry: Registry,
    pub rt_mapping: RtMapping,
}

impl TransformRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            rt_mapping: HashMap::new(),
        }
    }

    pub fn add_transform(&mut self, transform: LoadedTransform) {
        let hash = transform.descriptor.hash.clone();
        let hooks = transform.descriptor.rt_hooks.clone();
        self.registry.insert(transform.descriptor.hash, transform);
        for rt_hook in hooks {
            let mapping = self.rt_mapping.entry(rt_hook.clone()).or_insert(Vec::new());
            mapping.push(hash.clone());
        }
    }

    pub fn remove_transform(&mut self, hash: [u8; 32]) {
        self.registry.remove(&hash);
        self.rt_mapping.retain(|_, v| !v.contains(&hash));
    }

    /// # run
    /// Executes a single transform with the input already CBORed
    ///
    pub fn run(&self, hash: [u8; 32], input: &[u8]) -> Result<Vec<u8>> {
        let transform = self.registry.get(&hash);
        if transform.is_none() {
            anyhow::bail!("Transform not found");
        }
        let transform = transform.unwrap();
        let entry = transform.entry;
        let mut input = TransformContext {
            input: input,
            output: &mut None,
            diag: &mut None,
        };
        println!("Firing transform: {}", transform.descriptor.name);
        let result = (entry.entry)(&mut input);
        println!("Result: {:?}", result);

        let output = input.output.clone();
        let _diag = input.diag.clone();
        // TODO: Figure out how/what we are doing with diag.

        Ok(output.unwrap())
    }
}
