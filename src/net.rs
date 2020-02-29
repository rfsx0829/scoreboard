extern crate iron;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate rustc_serialize;

use super::db::{insert, select, Record};
use super::game::MinesweeperMode;
use iron::{headers::AccessControlAllowOrigin, prelude::*, status, Chain};
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(Debug)]
pub struct NetConfig {
    port: i32,
}

impl NetConfig {
    pub fn new(port: i32) -> NetConfig {
        NetConfig { port }
    }

    pub fn to_host(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

struct PasspoolBody;

impl iron::typemap::Key for PasspoolBody {
    type Value = Pool<PostgresConnectionManager<NoTls>>;
}

pub fn start_server(pool: Pool<PostgresConnectionManager<NoTls>>, conf: &NetConfig) {
    let mut router = Router::new();

    router.get("/api/v1/minesweeper/:mode", get_handler, "get_handler");
    router.post("/api/v1/minesweeper/:mode", post_handler, "post_handler");

    let mut chain = Chain::new(router);
    chain.link_before(move |req: &mut Request| {
        req.extensions.insert::<PasspoolBody>(pool.clone());
        Ok(())
    });
    chain.link_before(|req: &mut Request| {
        println!("[info]: {} -- {}", req.method, req.url.path().join("/"));
        Ok(())
    });
    chain.link_after(|_: &mut Request, mut res: Response| {
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    });

    println!("[info]: Listening on http://{}", conf.to_host());
    Iron::new(chain).http(conf.to_host().as_str()).unwrap();
}

fn get_handler(req: &mut Request) -> IronResult<Response> {
    let pool = req.extensions.get::<PasspoolBody>().unwrap();
    let mut db = pool.clone().get().unwrap();

    let ref mode = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("mode")
        .unwrap_or("/");
    let mode = MinesweeperMode::from(*mode);

    match mode {
        MinesweeperMode::Error => Ok(Response::with((
            status::BadRequest,
            String::from("Bad Mode"),
        ))),
        _ => {
            let mut results = select(&mut db, mode, 3);
            for i in 0..results.len() {
                results[i].set_id(i as i32);
            }
            let data = json::encode(&results).unwrap();
            Ok(Response::with((status::Ok, data)))
        }
    }
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let ref mode = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("mode")
        .unwrap_or("/");
    let mode = MinesweeperMode::from(*mode);

    let pool = req.extensions.get::<PasspoolBody>().unwrap();
    let mut db = pool.clone().get().unwrap();

    let mut buffer = String::new();
    req.body.read_to_string(&mut buffer).unwrap();
    let record: Record = json::decode(buffer.as_str()).unwrap();

    match mode {
        MinesweeperMode::Error => Ok(Response::with((
            status::BadRequest,
            String::from("Bad Mode"),
        ))),
        _ => {
            insert(&mut db, record).unwrap();
            Ok(Response::with((status::Ok, String::from("SUCCESS"))))
        }
    }
}
