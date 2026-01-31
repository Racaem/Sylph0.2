pub trait Plugin {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> Result<(), String>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn initialize_all(&mut self) -> Result<(), String> {
        for plugin in &mut self.plugins {
            plugin.initialize()?;
        }
        Ok(())
    }
}
