use clap::{arg, command, Command};

fn main() {
    let mathes = command!()
        .propagate_version(true)
        .subcommand(
            Command::new("set")
                .about("Set Value in the kvs")
                .arg(arg!([key]))
                .arg(arg!([value])),
        )
        .subcommand(
            Command::new("get")
                .about("get value by key")
                .arg(arg!([key])),
        )
        .subcommand(
            Command::new("rm")
                .about("remove key from kvs")
                .arg(arg!([key])),
        )
        .get_matches();

    if let Some(maches) = mathes.subcommand_matches("set") {
        let key: &String = maches.get_one("key").expect("expect key");
        let value:&String = maches.get_one("value").expect("expect value");

        println!("{}  {}",key,value)
    }
}
