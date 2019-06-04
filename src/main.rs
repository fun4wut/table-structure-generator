extern crate clap;

use clap::{Arg, App};

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
            .multiple(true))
        .get_matches();

    let q = Query::new(Config::new(&matches)).unwrap_or_else(|err| {
        eprintln!("数据库连接失败!:{}", err);
        process::exit(1);
    });
    println!("{}", q.make_query());
}