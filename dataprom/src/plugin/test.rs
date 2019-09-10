
use super::Plugin;
use super::{DataIn, DataOut};

pub(crate) struct Test {
    p_name: String,
}

impl Test {
    pub(crate) fn new(name: String) -> Self {
        Self {
            p_name: name,
        }
    }
}

impl Plugin for Test {
    fn name(&self) -> String {
        self.p_name.clone()
    }
    fn init(&mut self) {

    }
    fn parse(&self, data: DataIn) -> Vec<DataOut> {
        trace!("test_plugin: parse {}", data.data);
        if data.collector_name.unwrap() != self.p_name {
            panic!("wrong data");
        }
        let mut data_vec = Vec::new();
        
        let data = DataOut {
            name: self.p_name.clone(),
            help: "test data".to_string(),
            prometheus_name: "test_data".to_string(),
            data: data.data,
            tags: None,
        };
        data_vec.push(data);
        data_vec
    }
}
