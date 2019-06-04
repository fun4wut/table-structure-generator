#[derive(Debug)]
pub struct Config<'a> {
    pub db_name: &'a str,
    pub port: i32,
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub tables: Vec<&'a str>,
}

impl<'a> Config<'a> {
    pub fn gen_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}"
                , self.username, self.password, self.host, self.port, self.db_name)
    }
}