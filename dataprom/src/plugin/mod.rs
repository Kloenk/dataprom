use std::sync::{Arc, Mutex};


pub(crate) mod test;   // TODO: REMOVE?

pub trait Plugin {
    fn init(&mut self);
    fn parse(&self, data: DataIn) -> Vec<DataOut>;
    /// print Plugin name
    /// This name must also be included in http header
    fn name(&self) -> String;
}

/// struct for data to go in a plugin
pub struct DataIn {
    /// name of the collector encoded in http header
    /// FIXME: add header name
    pub collector_name: Option<String>,

    /// Data got from the collector
    pub data: String,

    /// tags defined in the header
    pub tags: Option<Vec<String>>,
}

impl DataIn {
    pub fn new(name: &str, data: &str) -> Self {
        Self {
            collector_name: Some(name.to_string()),
            data: data.to_string(),
            tags: None,
        }
    }
}

/// struct for data to go out of a plugin
pub struct DataOut {
    /// name to find in hashmap
    /// should be plugin name and if multiple with a simple suffix
    /// so no doubles are existing
    /// 
    /// This name is not the one existing in prometheus, just used for internal reference
    pub name: String,

    /// help message to be shown in prometheus
    pub help: String,

    /// name to be found in the prometheus data
    pub prometheus_name: String,

    /// data to be presented to prometheus
    pub data: String,

    /// tags for the prometheus data
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl DataOut {


    /// add to Data
    fn add_to_data(&self, data: Arc<Mutex<HashMap<String, super::DataInner>>>) {
        let mut data = data.lock().unwrap();

        let data_type = super::DataType::Gauge;

        let data_inner = super::DataInner {
            data_type,
            help: self.help.clone(),
            name: self.prometheus_name.clone(),
            tags: self.tags.clone(),
            data: self.data.clone(),
        };

        data.insert(self.name.clone(), data_inner);
        
    }
}

use std::collections::HashMap;

pub(crate) struct Plugins {
    pub(crate) counter: u32,
    pub(crate) plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Plugins {
    pub(crate) fn new() -> Self {
        Self {
            counter: 0,
            plugins: HashMap::new(),
        }
    }
    #[allow(clippy::borrowed_box)]
    pub(crate) fn execute(&self, name: String, data: String, data_pool: Arc<Mutex<HashMap<String, super::DataInner>>>, tags: Option<Vec<String>>) -> bool {
        let mut data_in = DataIn::new(&name, &data);
        data_in.tags = tags;
        let plugin = self.plugins.get(&name);
        if plugin.is_none() {
            return false;
        }
        let plugin: &Box<dyn Plugin> = plugin.unwrap();
        let data_out = plugin.parse(data_in);
        for v in data_out {
            v.add_to_data(Arc::clone(&data_pool));
        }
        

        true
    }
    pub(crate) fn add(&mut self, plugin: Box<dyn Plugin>) {
        self.counter += 1;
        self.plugins.insert(plugin.name(), plugin);
    }
}