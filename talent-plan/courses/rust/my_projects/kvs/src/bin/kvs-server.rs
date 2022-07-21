use std::{env::current_dir, fs, net::SocketAddr, process::exit};

use kvs::Result;
use log::{error, LevelFilter};
use structopt::StructOpt;

const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:4000";

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Args {
    #[structopt(
        long,
        help = "specify the address",
        value_name = "IP:PORT",
        default_value = "DEFAULT_SERVER_ADDR",
        parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(
        long,
        help= "specify the strorage engine",
        value_name = "ENGIN-NAME",
        possible_values = &["sled","kvs"]
           )]
    engine: Option<String>,
}

//arg_enum! {
//#[allow(non_camel_case_types)]
//#[derive(Debug,Clone, Copy,PartialEq,Eq)]
//enum Engine {
//kvs,
//sled
//}
//}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();
    let mut args = Args::from_args();
    let res = get_engine().and_then(move |engine| {
        if args.engine.is_none() {
            args.engine = Some(engine.clone());
        }
        if !engine.is_empty() && args.engine != Some(engine) {
            error!("engine error");
            exit(1);
        }
        server_start()
    });

    if let Err(err) = res {
        error!("error happen{}", err);
    }

    Ok(())
}

fn server_start() -> Result<()> {

    Ok(())
}

fn get_engine() -> Result<String> {
    let engine_path = current_dir()?.join("engine");

    if !engine_path.exists() {
        return Ok("".to_string());
    }
    fs::read_to_string(engine_path).map_err(|err| err.into())
}
