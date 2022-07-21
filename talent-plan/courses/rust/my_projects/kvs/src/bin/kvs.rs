


use clap::{arg, command, Command};

fn main() {
    let macthes = command!()
        .propagate_version(true)
        .subcommand(
            Command::new("set")
                .about("Set Value in the kvs")
                .arg(arg!([key]).required(true))
                .arg(arg!([value]).required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("get value by key")
                .arg(arg!([key]).required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("remove key from kvs")
                .arg(arg!([key]).required(true)),
        )
        .get_matches();

    let mut db = kvs::KvStore::new();

    match macthes.subcommand().expect("expect subcommand") {
        ("set", matches) => {
            let key: &String = matches.get_one("key").unwrap();
            let value: &String = matches.get_one("value").unwrap();

            db.set(key.to_owned(), value.to_owned());
        }
        ("get", matches) => {
            let key: &String = matches.get_one("key").unwrap();

            match db.get(key.to_owned()) {
                Some(value) => {
                    println!("{}", value)
                }
                None => {
                    println!("no value found")
                }
            }
        }
        _ => unreachable!(),
    }
}
