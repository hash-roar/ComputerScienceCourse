use std::net::SocketAddr;

use kvs::{DbError, Result};
use log::debug;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Args {
    #[structopt(
        long,
        help = "specify the address",
        value_name = "IP:PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(subcommand)]
    command: KvsCommand,
}

#[derive(Debug, StructOpt)]
enum KvsCommand {
    #[structopt(help = "add key-value to database")]
    Set {
        #[structopt(value_name = "KEY", required = true, help = "string key")]
        key: String,
        #[structopt(value_name = "VALUE", required = true, help = "string key")]
        value: String,
        #[structopt(
            long,
            help = "kvs-server address",
            value_name = "ADDRESS",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[structopt(help = "get data")]
    Get {
        #[structopt(value_name = "KEY", required = true)]
        key: String,
        #[structopt(
            long,
            help = "kvs-server address",
            value_name = "ADDRESS",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[structopt(help = "remove data by key")]
    Remove {
        #[structopt(value_name = "KEY", required = true)]
        key: String,
        #[structopt(
            long,
            help = "kvs-server address",
            value_name = "ADDRESS",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

fn main() -> Result<()> {
    let mut arg = Args::from_args();
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();
    debug!("args load: {:?}", arg);
    

    Ok(())
}
