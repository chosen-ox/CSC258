/*
    Key-Value TCP Client
    Created by: Owen Wacha
          Date: March 20, 2023
    CSC258/458 - Parallel & Distributed Systems


    Compile with "rustc kvclient.rs"
    Run "./kvclient port action key value"
*/
#![allow(unused)]
use std::{
    io::{prelude::*},
    net::{Shutdown, TcpStream},
    env
};
use std::time::{Instant};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args[1].to_lowercase().eq("help") {
        help();
    }
    else if args.len() > 2 {
        // construct tcp target from args
        let mut target = "127.0.0.1".to_string();
        target.push_str(":");
        target.push_str(&args[1]);
        // println!("{:?}",target);

        // connect to target
        let start = Instant::now();
        if let Ok(mut stream) = TcpStream::connect(target.clone()) {
            println!("Connected to the server!");        
            let action = args[2].clone().to_lowercase();
            // send args after port to remote, separated by newlines
            let msg: String = args[2..].join("\n");
            stream.write_all(msg.as_bytes()).unwrap();
            stream.shutdown(Shutdown::Write);

            //wait for response
            let mut buffer = String::new();
            stream.read_to_string(&mut buffer);   
            let duration = start.elapsed();
            // handle response
            println!("Received response in {:?}",duration);
            if action.eq("get") {
                println!("{}",buffer);
            }
        } else {
            println!("Couldn't connect to server: {}",target);
        }
    }
    else{
        println!("Incorrect usage. Try \"kvclient help\" for valid usage");
    }
}

fn help() {
    println!("\n** KVCLIENT HELP **:");
    println!("Correct Usage:\n kvclient port action key value");
    println!("Example Usage:\n kvclient 7878 get 12 twelve");
    println!("port: numerical port. Example: 7878");
    println!("action: can be \"get\" or \"put\" depending on desired action.");
    println!("key: the desired key");
    println!("value: the string to insert if the action is put");
}