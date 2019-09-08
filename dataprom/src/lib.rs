// rocket foo
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
use rocket::{State};

#[macro_use] extern crate log;
extern crate env_logger;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt;

#[cfg(test)]
mod test;

/// Enclousor around HashMap for Prometheus tag fields
#[derive(Debug)]
struct PrometheusTags (HashMap<String, String>);

impl PrometheusTags {
    pub fn new() -> Self {
        Self ( HashMap::new() )
    }
}

impl fmt::Display for PrometheusTags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for (k, v) in &self.0 {
            if ret.is_empty() {
                ret = format!("{}=\"{}\"", k, v);
            } else {
                ret = format!("{},{}=\"{}\"", ret, k, v);
            }
        }
        write!(f, "{}", ret)
    }
}

#[derive(Debug)]
struct DataInner {
    /// data type
    data_type: DataType,

    /// help message for prometheus
    help: String,

    /// name as path in prometheus
    name: String,

    /// optional tags for prometheus
    tags: Option<PrometheusTags>,

    /// Data field
    data: String,
}

const SOURCE_TAG_NAME: &str = &"source";

impl DataInner {
    pub fn new() -> Self {
        Self {
            data_type: DataType::Gauge,
            help: "test".to_string(),
            name: "test".to_string(),
            tags: None,
            data: "2".to_string(),
        }
    }
    pub fn print(&self, source: &str) -> String {
        let tags = match &self.tags {
            None => String::new(),
            Some(tags) => {
                format!(",{}", tags)
            },
        };
        format!(r#"# HELP {} {}
# TYPE {} {}
{}{{{}="{}"{}}} {}"#, self.name, self.help,
            self.name, self.data_type,
            self.name, SOURCE_TAG_NAME, source, tags, self.data)
    }
}

/// Type enum for possible prometheus types
#[derive(Debug)]
enum DataType {
    Gauge,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        match self {
            DataType::Gauge => {ret = "gauge".to_string()},
        }
        write!(f, "{}", ret)
    }
}

/// manage datatype for rocket thread
struct Data(Arc<Mutex<HashMap<String, DataInner>>>);

impl Data {
    fn print(&self, clear: bool) -> String {
        let mut ret = String::new();

        let mut data = self.0.lock().unwrap();
        for (k, v) in data.iter() {
            println!("key: {}", k);
            println!("value: {}", v.print("test"));
        }

        ret
    }
}

pub struct Config {
    /// listen interface for rocket
    pub address: String,

    /// http port for rocket to listen on
    pub port: u16,
}

impl Config {
    /// playground FIXME: remove
    fn test(&self, data: Arc<Mutex<HashMap<String, DataInner>>>) {
        warn!("remove");
        let mut data = data.lock().unwrap();
        let mut d = DataInner::new();
        let mut tags = PrometheusTags::new();
        tags.0.insert("host".to_string(), "kloenkX".to_string());
        tags.0.insert("hello".to_string(), "world".to_string());
        d.tags = Some(tags);
        data.insert("test".to_string(), d);
    }

    /// run server
    pub fn run(self) {
        println!("dataprom: {}", env!("CARGO_PKG_VERSION"));
        println!("listening on: {}:{}", self.address, self.port);

        // initialise rocket config
        use rocket::config::{Config, Environment};

        let config = Config::build(Environment::Staging)
            .address(&self.address)
            .port(self.port)
            .finalize()
            .unwrap();
        trace!("build rocket config");


        // create Data
        let data = Data (Arc::new(Mutex::new(HashMap::new())));
        self.test(Arc::clone(&data.0));

        // launch rocket
        rocket::custom(config)
            .mount("/", routes![metrics])
            .manage(data)
            .launch();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 9091,
            address: "127.0.0.1".to_string(),
        }
    }
}

use rocket::response::Response;

#[get("/metrics")]
fn metrics(data: State<Data>) -> Response {
    Response::build()
        .status(rocket::http::Status::Ok)
        .raw_header("Content-Type", "text/plain; version=0.0.4")
        .sized_body(std::io::Cursor::new(data.print(false)))
        .finalize()
}
