extern crate postgres;

use postgres::Connection;
use super::config::{Config, TableConfig};
use self::postgres::TlsMode;
use self::postgres::rows::Rows;
use std::error::Error;

#[derive(Debug)]
pub struct TableRow {
    pub type_name: String,
    pub data_type: String,
    pub nullable: String,
    pub default: String,
}

#[derive(Debug)]
pub struct SingleTable {
    pub title: String,
    pub rows: Vec<TableRow>,
}

pub struct Query<'a> {
    conn: Connection,
    config: Config<'a>
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
        from information_schema.Columns where table_schema = 'public'
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
        from information_schema.Columns where table_schema = 'public' and table_name = $1;",
                                  &[&table_name])?;
        Ok(res)
    }
    fn to_table_rows(&self, rows: Rows) -> Vec<TableRow> {
        rows.iter().map(|item| {
            let type_name: String = item.get(0);
            let data_type: String = item.get(1);
            let nullable: String = item.get(2);
            let default: Option<String> = item.get(3);
            TableRow {
                type_name,
                data_type,
                nullable,
                default: default.unwrap_or(String::from("Null")),
            }
        }).collect::<Vec<_>>()
    }
    pub fn make_query(&self) -> Vec<SingleTable> {
        let tmp = self.list_all_tables();
        let tables = match &self.config.tables {
            TableConfig::All => &tmp,
            TableConfig::List(v) => v
        };
        tables.iter()
            .map(|table| {
                SingleTable {
                    title: table.clone(),
                    rows: self.to_table_rows(self.make_single_query(table)
                        .unwrap_or_else(|err| {
                            eprintln!("数据库读取错误:{}", err);
                            std::process::exit(1);
                        })),
                }
            }).collect::<Vec<_>>()
    }
}