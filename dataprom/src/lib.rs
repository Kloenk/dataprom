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

#[derive(Debug)]
struct DataInner {
    /// data type
    data_type: DataType,

    /// help message for prometheus
    help: String,

    /// name as path in prometheus
    name: String,

    /// optional tags for prometheus
    tags: Option<HashMap<String, String>>,

    /// Data field
    data: String,
}

const SOURCE_TAG_NAME: &str = &"source";

impl DataInner {
    pub fn print_tags(&self) -> String {
        let mut ret = String::new();
        if self.tags.is_none() {   // end if tags are not there
            return ret;
        }
        let map = self.tags.as_ref().unwrap();
        for (k, v) in map.iter() {
            if ret.is_empty() {
                ret = format!("{}=\"{}\"", k, v);
            } else {
                ret = format!("{},{}=\"{}\"", ret, k, v);
            }
        }
        ret
    }
    pub fn print(&self, source: &str) -> String {
        let tags = match &self.tags {
            None => String::new(),
            Some(_) => {
                format!(",{}", self.print_tags())
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
#[allow(dead_code)]
enum DataType {
    Gauge,
}

impl fmt::Display for DataType {
    #[allow(unused_assignments)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        match self {
            DataType::Gauge => {ret = "gauge".to_string()},
        }
        write!(f, "{}", ret)
    }
}

/// manage Source String for rocket threading
struct Source(String);

/// delete data if prometheus scrapes
struct Delete(bool);

/// manage datatype for rocket thread
struct Data(Arc<Mutex<HashMap<String, DataInner>>>);

impl Data {
    fn print(&self, clear: bool, source: &str) -> String {
        let mut ret = String::new();

        let mut data = self.0.lock().unwrap();

        if clear {
            debug!("clear data");
            for (k, v) in data.drain() {
                trace!("print data: {}", k);
                ret.push_str(&v.print(source));
            }
        } else {
            debug!("keep data alive");
            for (k, v) in data.iter() {
                trace!("print data: {}", k);
                ret.push_str(&v.print(source));
            }
        }

        ret
    }
}

pub struct Config {
    /// listen interface for rocket
    pub address: String,

    /// http port for rocket to listen on
    pub port: u16,

    /// source tag for prometheus data
    pub source: String,

    /// if true data will be deleted at prometheus scrape
    pub delete: bool,
}

impl Config {
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
        let source = Source ( self.source.clone() );
        let data = Data (Arc::new(Mutex::new(HashMap::new())));
        let delete = Delete (self.delete);

        // launch rocket
        rocket::custom(config)
            .mount("/", routes![metrics])
            .manage(data)
            .manage(source)
            .manage(delete)
            .launch();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 9091,
            address: "127.0.0.1".to_string(),
            source: "dataprom".to_string(),
            delete: false,
        }
    }
}

use rocket::response::Response;

#[get("/metrics")]
fn metrics<'a>(data: State<'a, Data>, source: State<Source>, delete: State<Delete>) -> Response<'a> {
    debug!("got request");
    let mut response = data.print(delete.0, &source.0);
    response.push_str(&format!("\n# dataprom/export_prometheus {}", env!("CARGO_PKG_VERSION")));
    Response::build()
        .status(rocket::http::Status::Ok)
        .raw_header("Content-Type", "text/plain; version=0.0.4")
        .raw_header("Server", "dataprom/export_prometheus")
        .raw_header("Cache_Control", "no-cache")
        .sized_body(std::io::Cursor::new(response))
        .finalize()
}
