extern crate scoreboard;

use scoreboard::db::*;
use scoreboard::net::*;

fn main() {
    let dbconf = PostgresConfig::new("db.config.json");
    let pool = connect(dbconf).unwrap();

    let conf = NetConfig::new(6666);
    start_server(pool.clone(), &conf);
}
