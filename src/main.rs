extern crate clap;

use clap::{Arg, App, SubCommand};

mod config;
mod query;

use query::Query;
use crate::config::Config;
use std::process;

fn main() {
    let matches = App::new("table-structure-generator")
        .version("0.1")
        .author("Fun4wut")
        .about("Generate PGSQL table structure")
        .args_from_usage("-U, --username=[USERNAME] 'Sets PG username'
                              -d, --database=[DATABASE]   'Sets database'
                              --host=[HOST] 'Sets the host'
                              -p, --port=[PORT] 'Sets the port'
                              --password=<PASSWORD> 'Sets the password'")
//        多参数只能使用该方法进行解析
        .arg(Arg::with_name("TABLES")
            .multiple(true)
            .required(true)
            .min_values(1))
        .get_matches();
    let cfg = Config {
        db_name: matches.value_of("database").unwrap_or("postgres"),
        port: matches.value_of("port").unwrap_or("5432").parse::<i32>().unwrap(),
        host: matches.value_of("host").unwrap_or("0.0.0.0"),
        username: matches.value_of("username").unwrap_or("postgres"),
        password: matches.value_of("password").unwrap(),
        tables: matches.values_of("TABLES").unwrap().collect::<Vec<&str>>(),
    };
//    println!("{:#?}",cfg);
    let q = Query::new(cfg).unwrap_or_else(|err| {
        eprintln!("数据库连接失败!");
        process::exit(1);
    });
    println!("{}", q.make_query());
}