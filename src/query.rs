extern crate postgres;

use postgres::Connection;
use super::config::{Config, TableConfig};
use self::postgres::TlsMode;
use self::postgres::rows::Rows;
use std::error::Error;

pub struct Query<'a> {
    conn: Connection,
    config: Config<'a>,
}

impl<'a> Query<'a> {
    pub fn new(config: Config<'a>) -> Result<Self, Box<dyn Error>> {
        let q = Self {
            conn: Connection::connect(config.gen_url(), TlsMode::None)?,
            config,
        };
        Ok(q)
    }

    //   根据tableconfig字段给出vec表
    fn list_all_tables(&self) -> Vec<String> {
        self.conn.query("select
        table_name
        from information_schema.columns where table_schema = 'public'
        group by table_name;", &[]).unwrap_or_else(|err| {
            eprintln!("获取表名数据失败:{}", err);
            std::process::exit(1);
        }).iter().map(|row| {
            let table_name: String = row.get(0);
            table_name
        }).collect::<Vec<_>>()
    }


    fn make_single_query(&self, table_name: &str) -> Result<Rows, Box<dyn Error>> {
        let res = self.conn.query("select
        column_name ,
        data_type ,
        is_nullable ,
        column_default
        from information_schema.columns where table_schema = 'public' and table_name = $1;",
                                  &[&table_name])?;
        Ok(res)
    }
    fn to_html(&self, rows: Rows) -> Result<String, Box<dyn Error>> {
        let tbody = rows.iter().try_fold(String::new(), |acc: String, item|
                                                         -> Result<String, Box<dyn Error>>{
            let type_name: String = item.get(0);
            let data_type: String = item.get(1);
            let nullable: String = item.get(2);
            let default: Option<String> = item.get(3);
            Ok(acc + format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                             type_name, data_type, nullable, default.unwrap_or(String::from("Null"))).as_str())
        })?;
        Ok(format!("<table border=\"1\"><thead><tr><th>字段名称</th><th>数据类型</th><th>是否可空</th><th>默认值</th></tr></thead><tbody>{}</tbody></table>"
                   , tbody))
    }
    pub fn make_query(&self) -> String {
        let tmp = self.list_all_tables();
        let tables = match &self.config.tables {
            TableConfig::All => &tmp,
            TableConfig::List(v) => v
        };
        let html_text = tables.iter()
            .fold(String::new(), |acc: String, table| {
                acc + format!("<h3>{}</h3>", table).as_str()
                    + self.to_html(self.make_single_query(table)
                    .unwrap_or_else(|err| {
                        eprintln!("数据库读取错误:{}", err);
                        std::process::exit(1);
                    })).unwrap_or_else(|err| {
                    eprintln!("渲染错误:{}", err);
                    std::process::exit(1);
                }).as_str()
            });
        format!("<!doctype html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <title>Document</title>
            </head>
            <body>
              {}
            </body>
            </html>", html_text)
    }
}