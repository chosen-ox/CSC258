#![allow(unused)]

use multi_decree_paxos::{Acceptor, Learner, MsgType, Proposer, Role};
use portpicker::pick_unused_port;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::thread::{sleep, spawn};
use std::time::Instant;
use std::{
    env,
    io::prelude::*,
    net::{Shutdown, TcpListener},
    str,
};

fn send_msg(mut stream: &TcpStream, mut msg: String) -> std::io::Result<()> {
    msg.push('\n');
    loop {
        match stream.write_all(msg.as_bytes()) {
            Ok(_) => {
                stream.flush()?;
                break;
            }
            Err(e) => {
                sleep(std::time::Duration::from_millis(1000));
                print!("error: {:?}",e);
            }
        }
    }

    sleep(std::time::Duration::from_millis(1000));
    Ok(())
}

fn broadcast_msg(streams: &Vec<TcpStream>, mut msg: String) -> std::io::Result<()> {
    // msg.push("\n".to_string());
    // let msg = msg.join(" ");
    // let whole_msg = format!("{} {}", char::from(msg_type as u8), msg);
    // println!("broadcasting: {}", whole_msg);
    msg.push('\n');
    for mut stream in streams {
        loop {
            match stream.write_all(msg.as_bytes()) {
                Ok(_) => {
                    stream.flush()?;
                    break;
                }
                Err(e) => {
                    sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }
    sleep(std::time::Duration::from_millis(1000));
    Ok(())
}

fn state_machine(
    listener: TcpListener,
    receive_streams: Vec<TcpStream>,
    send_streams: Vec<TcpStream>,
) {


    let len = receive_streams.len();
    let f = (len / 2) as u8;

    let mut proposer = Proposer::new();
    let mut acceptor = Acceptor::new();
    let mut learner = Learner::new();
    let mut client = Vec::<TcpStream>::new();
    proposer.set_f(f);
    let mut ss = 1;
    loop {

        if listener.local_addr().unwrap()
            == SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 2030)) && ss == 1
        {
            let msg = format!(
                "{} {} {} {}",
                char::from(MsgType::PREPARE as u8),
                "1",
                "hello",
                "world"
            );
            broadcast_msg(&send_streams, msg).unwrap();
            ss = 0;
        }
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    client.push(stream);
                }
                Err(e) => {
                    break;
                }
            }
        }

        for (i, mut stream) in receive_streams.iter().enumerate() {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let msg_type = MsgType::from(buffer[0]);
                    let end = buffer
                        .iter()
                        .position(|&x| x == b'\n')
                        .expect("No null byte");
                    let msg = str::from_utf8(&buffer[1..end]).unwrap().to_owned();
                    println!("Received msg {}", msg);
                    match msg_type {
                        MsgType::PREPARE | MsgType::ACCEPT => {
                            if let Some(msg) = acceptor.handle_msg(&msg_type, &msg) {
                                send_msg(&send_streams[i], msg).unwrap();
                                // broadcast_msg(&send_streams, msg).unwrap();
                            }
                        }
                        MsgType::PROMISE
                        | MsgType::RESPONSE
                        | MsgType::NACK
                        | MsgType::UNACCEPTED => {
                            if let Some(msg) = proposer.handle_msg(&msg_type, &msg) {
                                broadcast_msg(&send_streams, msg).unwrap();
                            }
                        }
                        MsgType::ACCEPTED => {
                            if let Some(msg) = learner.handle_msg(&msg_type, &msg) {
                                broadcast_msg(&send_streams, msg).unwrap();
                            }
                            if let Some(msg) = proposer.handle_msg(&msg_type, &msg) {
                                broadcast_msg(&send_streams, msg).unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }

        for mut stream in &client {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let msg_type = MsgType::from(buffer[0]);
                    let msg = str::from_utf8(&buffer[1..]).unwrap().to_owned();
                    match msg_type {
                        _ => {}
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Incorrect usage. Try \" cargo run -- port F\" for valid usage");
    } else if args.len() > 2 {
        let process_num: usize = 2 * args[2].clone().parse::<usize>().unwrap() + 1;
        let mut ports: Vec<u16> = (0..=process_num)
            .map(|_| pick_unused_port().expect("No ports free"))
            .collect();
        ports.insert(0, args[1].clone().parse().unwrap());

        println!("Starting {} processes", process_num);
        let mut listeners: Vec<TcpListener> = ports
            .iter()
            .map(|&port| {
                println!("IP address: 127.0.0.1, Port:{}", port);
                let listener = TcpListener::bind(("127.0.0.1", port))
                    .expect(&*("Could not bind to port:".to_string() + &port.to_string()));
                listener
                    .set_nonblocking(true)
                    .expect("Cannot set non-blocking");
                listener
            })
            .collect();
        let mut streams: Vec<Vec<TcpStream>> = (0..process_num)
            .map(|_| {
                let mut streams = (0..process_num)
                    .map(|j| TcpStream::connect(("127.0.0.1", ports[j])).unwrap())
                    .collect::<Vec<_>>();
                // streams.iter().for_each(|stream| {
                //     stream
                //         .set_nonblocking(true)
                //         .expect("Cannot set non-blocking");
                // });
                streams
            })
            .collect();
        (0..process_num).for_each(|_| {
            let listener = listeners.remove(0);
            let send_streams = streams.remove(0).drain(..).collect();
            let mut receive_streams = Vec::with_capacity(process_num);
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        stream.set_nonblocking(true).unwrap();
                        receive_streams.push(stream);
                    }
                    Err(e) => {
                        break;
                    }
                };
            }
            spawn(move || {
                state_machine(listener, receive_streams, send_streams);
            });
        });

        loop {}
    }
}