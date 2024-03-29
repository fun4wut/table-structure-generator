use clap::ArgMatches;

#[derive(Debug)]
pub struct Config<'a> {
    pub db_name: &'a str,
    pub port: i32,
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub tables: TableConfig
}

#[derive(Debug)]
pub enum TableConfig {
    All,
    List(Vec<String>)
}

impl<'a> Config<'a> {
    pub fn gen_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}"
                , self.username, self.password, self.host, self.port, self.db_name)
    }
    pub fn new(matches: &'a ArgMatches) -> Self {
        Self {
            db_name: matches.value_of("database").unwrap(),
            port: matches.value_of("port").unwrap().parse::<i32>().unwrap(),
            host: matches.value_of("host").unwrap(),
            username: matches.value_of("username").unwrap(),
            password: matches.value_of("password").unwrap(),
            tables: match matches.values_of("TABLES") {
                None => TableConfig::All,
                Some(values) => TableConfig::List(values.map(String::from).collect::<Vec<_>>())
            },
        }
    }
}