extern crate askama;

use super::query::SingleTable;
use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct MyTemplate {
    tables: Vec<SingleTable>
}

impl MyTemplate {
    pub fn new(tables: Vec<SingleTable>) -> Self {
        Self {
            tables
        }
    }

    pub fn show(&self) {
        println!("{}", self.render().unwrap())
    }
}