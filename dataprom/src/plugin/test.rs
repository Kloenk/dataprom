
use super::Plugin;
use super::{DataIn, DataOut};

pub(crate) struct Test {
    p_name: String,
}

impl Test {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            p_name: name.to_string(),
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

        let tags = if data.tags.is_some() {
            trace!("tags: {:?}", &data.tags);
            let mut tags_vec = std::collections::HashMap::new();
            let tags_orig = data.tags.unwrap();
            let tags_orig: Vec<&str> = tags_orig[0].split("; ").collect();    // FIXME
            for v in tags_orig {
                let vec: Vec<&str> = v.split('=').collect();
                if vec.len() != 2 {
                    warn!("invalid lenght of array size: {} ({:?})", vec.len(), vec);
                    continue;
                }
                tags_vec.insert(vec[0].to_string(), vec[1].to_string());
            }
            Some(tags_vec)
        } else {
            None
        };

        let name_list = format!("{}_{:?}", self.p_name, tags);
        
        let data = DataOut {
            name: name_list,
            help: "test data".to_string(),
            prometheus_name: "test_data".to_string(),
            data: data.data,
            tags,
        };
        data_vec.push(data);
        data_vec
    }
}
