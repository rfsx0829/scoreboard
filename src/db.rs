extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rustc_serialize;

use super::game::MinesweeperMode;
use postgres::{Client, Error, NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use rustc_serialize::json;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::string::String;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct PostgresConfig {
    user: String,
    pass: String,
    host: String,
    port: i32,
    db: String,
    tls: bool,
}

impl PostgresConfig {
    pub fn new(filename: &str) -> PostgresConfig {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&Path::new(&String::from(filename)))
            .unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        json::decode(&buffer).unwrap()
    }

    pub fn to_string(&self) -> String {
        String::from(format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.db
        ))
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Record {
    id: i32,
    name: String,
    score: f32,
    mode: MinesweeperMode,
}

impl Record {
    pub fn new(mode: MinesweeperMode, name: String, score: f32) -> Record {
        Record {
            id: 0,
            name: name,
            score: score,
            mode: mode,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}

pub fn connect(
    conf: PostgresConfig,
) -> Result<Pool<PostgresConnectionManager<NoTls>>, r2d2::Error> {
    let manager = PostgresConnectionManager::new(conf.to_string().parse().unwrap(), NoTls);
    Pool::new(manager)
}

pub fn insert(conn: &mut Client, r: Record) -> Result<u64, Error> {
    conn.execute(
        format!(
            "insert into {} (name, score, date) values ($1, $2, now())",
            r.mode
        )
        .as_str(),
        &[&r.name, &r.score],
    )
}

pub fn select(conn: &mut Client, table: MinesweeperMode, limit: i32) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];
    for row in &conn
        .query(
            format!(
                "select name, score from {} order by score offset 0 limit {}",
                table, limit
            )
            .as_str(),
            &[],
        )
        .unwrap()
    {
        let each = Record::new(table, row.get(0), row.get(1));
        records.push(each);
    }
    records
}
