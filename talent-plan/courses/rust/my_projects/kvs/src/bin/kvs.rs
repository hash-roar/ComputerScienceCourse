use clap::{arg, command, Command};

fn main() {
    let mathes = command!()
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

    if let Some(maches) = mathes.subcommand_matches("set") {
        let key: String = maches
            .get_one::<String>("key")
            .expect("expect key")
            .to_owned();
        let value: String = maches
            .get_one::<String>("value")
            .expect("expect key")
            .to_owned();

        db.set(key, value);
    }
    if let Some(_) = mathes.subcommand_matches("get") {}
    if let Some(_) = mathes.subcommand_matches("rm") {}
}
