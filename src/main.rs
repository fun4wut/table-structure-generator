extern crate clap;

use clap::{Arg, App};

mod config;
mod query;
mod template;
use query::Query;
use crate::config::Config;
use std::process;
use template::MyTemplate;
fn main() {
    let matches = App::new("table-structure-generator")
        .version("1.0")
        .author("Fun4wut")
        .about("Generate PGSQL table structure")
        .arg(Arg::with_name("username")
            .short("U")
            .long("username")
            .help("Sets PG username")
            .value_name("USERNAME")
            .default_value("postgres"))
        .arg(Arg::with_name("database")
            .short("d")
            .long("database")
            .value_name("DATABASE")
            .help("Sets database")
            .default_value("postgres"))
        .arg(Arg::with_name("host")
            .long("host")
            .short("H")
            .value_name("HOST")
            .help("Sets the host")
            .default_value("127.0.0.1"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Sets the port")
            .default_value("5432"))
        .arg(Arg::with_name("password")
            .long("password")
            .short("P")
            .required(true)
            .value_name("PASSWORD")
            .help("Sets the password"))
        .arg(Arg::with_name("TABLES")
            .multiple(true)
            .help("the tables you want to generate the structure, default is all tables"))
        .get_matches();

    let q = Query::new(Config::new(&matches)).unwrap_or_else(|err| {
        eprintln!("数据库连接失败!:{}", err);
        process::exit(1);
    });
    MyTemplate::new(q.make_query()).show();
}