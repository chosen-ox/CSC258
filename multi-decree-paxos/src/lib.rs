use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

/* Message Format:
 * PREPARE <proposal_number>
 * PROMISE <proposal_number> <accepted_proposal_number> <key> *<value>
 * ACCEPT <proposal_number> <key> *<value> // If no accepted value, proposal_number is 0
 * ACCEPTED <proposal_number> <key> *<value>
 * UNACCEPTED <proposal_number>
 * RESPONSE <proposal_number> <key> *<value>
 * NACK <proposal_number>
 */
#[derive(PartialEq)]
#[repr(u8)]
pub enum MsgType {
    PREPARE = 0,
    PROMISE,
    ACCEPT,
    ACCEPTED,
    UNACCEPTED,
    RESPONSE,
    NACK,
}

impl From<u8> for MsgType {
    fn from(item: u8) -> Self {
        match item {
            0 => MsgType::PREPARE,
            1 => MsgType::PROMISE,
            2 => MsgType::ACCEPT,
            3 => MsgType::ACCEPTED,
            4 => MsgType::UNACCEPTED,
            5 => MsgType::RESPONSE,
            6 => MsgType::NACK,
            _ => panic!("Unknown message type"),
        }
    }
}
pub struct Proposer {
    suggested_proposal_number: u8,
    suggested_value: (Option<String>, Option<String>),
    wait_for_promise: bool,
    wait_for_accepted: bool,
    wait_for_response: bool,
    promise_vote_count: u8,
    accepted_vote_count: u8,
    proposal_number: u8,
    proposal_value: (Option<String>, Option<String>),
    nack_count: u8,
    unaccepted_count: u8,
    f: u8,
}

pub struct Acceptor {
    promised_proposal_number: u8,
    accepted_value: (Option<String>, Option<String>),
    accepted_proposal_number: u8,
}

pub struct Learner {
    kv_store: HashMap<String, String>,
    proposal_number: u8,
}

pub trait Role {
    fn new() -> Self;
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String>;
}

impl Proposer {
    pub fn set_f(&mut self, f: u8) {
        self.f = f;
    }

    pub fn send_prepare(&mut self, msg: &str) -> Option<String> {
        let msg: Vec<&str> = msg.split("\n").collect();
        // println!("from client: {:?}", msg);
        let method = msg[0];
        let key = msg[1];
        self.wait_for_promise = true;
        self.promise_vote_count = 0;
        self.accepted_vote_count = 0;
        self.nack_count = 0;
        self.unaccepted_count = 0;
        self.proposal_number += 1;
        if method == "get" {
            self.proposal_value = (Some(key.to_string()), None);
        } else {
            let value = msg[2];
            self.proposal_value = (Some(key.to_string()), Some(value.to_string()));
        };
        let msg = format!(
            "{} {}",
            char::from(MsgType::PREPARE as u8),
            self.proposal_number,
        );
        Some(msg)
    }

    pub fn reset(&mut self) {
        self.suggested_proposal_number = 0;
        self.suggested_value = (None, None);
        self.wait_for_promise = false;
        self.wait_for_accepted = false;
        self.wait_for_response = false;
        self.promise_vote_count = 0;
        self.accepted_vote_count = 0;
        // self.proposal_number = 0;
        self.proposal_value = (None, None);
        self.nack_count = 0;
        self.unaccepted_count = 0;
    }
}

impl Acceptor {
    pub fn flush_accepted_value(&mut self) {
        self.accepted_value = (None, None);
    }
}

impl Learner {
    pub fn get_kv_store(&self) -> &HashMap<String, String> {
        &self.kv_store
    }
    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.kv_store.get(key)
    }

    pub fn print_kv_store(&self) {
        for (key, value) in &self.kv_store {
            println!("{}: {}", key, value);
        }
    }
}

impl Role for Proposer {
    fn new() -> Self {
        Proposer {
            suggested_proposal_number: 0,
            suggested_value: (None, None),
            wait_for_promise: false,
            wait_for_accepted: false,
            wait_for_response: false,
            proposal_number: 0,
            promise_vote_count: 0,
            accepted_vote_count: 0,
            nack_count: 0,
            unaccepted_count: 0,
            f: 0,
            proposal_value: (None, None),
        }
    }
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String> {
        let split_msg: Vec<&str> = msg.split(" ").filter(|&msg| msg != "").collect();
        match msg_type {
            MsgType::PROMISE => {
                if self.wait_for_promise {
                    let proposal_number = split_msg[0].parse::<u8>().unwrap();
                    if proposal_number != self.proposal_number {
                        return None;
                    }
                    self.promise_vote_count += 1;
                    let accepted_proposal_number = split_msg[1].parse::<u8>().unwrap();
                    if accepted_proposal_number > self.suggested_proposal_number {
                        self.suggested_proposal_number = accepted_proposal_number;
                        let key = split_msg[2].to_string();
                        if split_msg.len() == 4 {
                            let value = split_msg[3].to_string();
                            self.suggested_value = (Some(key), Some(value));
                        } else {
                            self.suggested_value = (Some(key), None);
                        }
                    }
                    // println!("{}", self.f);

                    if self.promise_vote_count >= self.f + 1 {
                        // println!("Got enough promises");
                        self.wait_for_promise = false;
                        self.wait_for_accepted = true;
                        return if self.suggested_proposal_number == 0 {
                            if self.proposal_value.1.is_some() {
                                let msg = format!(
                                    "{} {} {} {}",
                                    char::from(MsgType::ACCEPT as u8),
                                    self.proposal_number,
                                    self.proposal_value.0.as_ref().unwrap(),
                                    self.proposal_value.1.as_ref().unwrap().clone(),
                                );
                                Some(msg)
                            } else {
                                let msg = format!(
                                    "{} {} {}",
                                    char::from(MsgType::ACCEPT as u8),
                                    self.proposal_number,
                                    self.proposal_value.0.as_ref().unwrap(),
                                );
                                Some(msg)
                            }
                        } else {
                            if self.suggested_value.1.is_some() {
                                let msg = format!(
                                    "{} {} {} {}",
                                    char::from(MsgType::ACCEPT as u8),
                                    self.proposal_number,
                                    self.suggested_value.0.as_ref().unwrap(),
                                    self.suggested_value.1.as_ref().unwrap().clone(),
                                );
                                Some(msg)
                            } else {
                                let msg = format!(
                                    "{} {} {}",
                                    char::from(MsgType::ACCEPT as u8),
                                    self.proposal_number,
                                    self.suggested_value.0.as_ref().unwrap(),
                                );
                                Some(msg)
                            }
                        };
                    }
                }
            }
            MsgType::RESPONSE => {
                // println!("response:{}", self.proposal_number);
                // if self.wait_for_response {
                    let proposal_number = split_msg[0].parse::<u8>().unwrap();
                    if proposal_number != self.proposal_number {
                        return None;
                    }
                    self.reset();
                    let response_type = split_msg[1].parse::<u8>().unwrap();
                    if response_type == 0 {
                        // println!("put successful!");
                        return Some("put successful!".to_string());
                    } else if response_type == 1 {
                        let value = split_msg[2].to_string();
                        // println!("get successful!");
                        return Some("get successful! value:".to_string() + &value);
                    } else {
                        // println!("get failed!");
                        return Some("get failed!".to_string());
                    }
                // }
            }
            MsgType::ACCEPTED => {
                if self.wait_for_accepted {
                    self.wait_for_response = true;
                    let proposal_number = split_msg[0].parse::<u8>().unwrap();
                    if proposal_number != self.proposal_number {
                        return None;
                    }
                    self.accepted_vote_count += 1;
                    if self.accepted_vote_count >= self.f + 1 {
                        self.wait_for_accepted = false;
                        // self.proposal_number += 1;
                        self.proposal_value = (None, None);
                        self.suggested_proposal_number = 0;
                        self.suggested_value = (None, None);
                        self.promise_vote_count = 0;
                        self.accepted_vote_count = 0;
                        self.nack_count = 0;
                        self.unaccepted_count = 0;
                        return None;
                    }
                }
            }
            MsgType::UNACCEPTED => {
                let proposal_number = split_msg[0].parse::<u8>().unwrap();
                if proposal_number != self.proposal_number {
                    return None;
                }
                // self.proposal_number = split_msg[1].parse::<u8>().unwrap();
                // self.proposal_number += 1;
                // println!("unaccepted, new proposal number: {}", self.proposal_number);
            }
            MsgType::NACK => {
                let proposal_number = split_msg[0].parse::<u8>().unwrap();
                if proposal_number != self.proposal_number {
                    return None;
                }
                self.proposal_number = split_msg[1].parse::<u8>().unwrap();
                self.proposal_number += 1;
                self.wait_for_promise = true;
                self.promise_vote_count = 0;
                self.accepted_vote_count = 0;
                self.nack_count = 0;
                self.unaccepted_count = 0;
                // println!("nack, new proposal number:{}", self.proposal_number);
                let msg = format!(
                    "{} {}",
                    char::from(MsgType::PREPARE as u8),
                    self.proposal_number
                );
                sleep(Duration::from_millis(1000));
                return Some(msg);
            }
            _ => {}
        }
        None
    }
}

impl Role for Acceptor {
    fn new() -> Self {
        Acceptor {
            accepted_value: (None, None),
            promised_proposal_number: 0,
            accepted_proposal_number: 0,
        }
    }
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String> {
        let split_msg: Vec<&str> = msg.split(" ").filter(|&msg| msg != "").collect();

        match msg_type {
            MsgType::PREPARE => {
                let proposal_number = split_msg[0].parse().unwrap();
                // println!("acceptor receive prepare: {}, current {}", proposal_number, self.promised_proposal_number);
                if self.promised_proposal_number < proposal_number {
                    self.promised_proposal_number = proposal_number;
                    if self.accepted_value.0.is_some() {
                        return if self.accepted_value.1.is_some() {
                            let msg = format!(
                                "{} {} {} {} {}",
                                char::from(MsgType::PROMISE as u8),
                                proposal_number,
                                self.accepted_proposal_number,
                                self.accepted_value.0.as_ref().unwrap(),
                                self.accepted_value.1.as_ref().unwrap(),
                            );
                            Some(msg)
                        } else {
                            let msg = format!(
                                "{} {} {} {}",
                                char::from(MsgType::PROMISE as u8),
                                proposal_number,
                                self.accepted_proposal_number,
                                self.accepted_value.0.as_ref().unwrap(),
                            );
                            Some(msg)
                        };
                    } else {
                        let msg = format!(
                            "{} {} {}",
                            char::from(MsgType::PROMISE as u8),
                            proposal_number,
                            0,
                        );
                        return Some(msg);
                    }
                } else {
                    let msg = format!(
                        "{} {} {}",
                        char::from(MsgType::NACK as u8),
                        proposal_number,
                        self.promised_proposal_number,
                    );
                    return Some(msg);
                }
            }
            MsgType::ACCEPT => {
                let proposal_number = split_msg[0].parse().unwrap();
                if self.promised_proposal_number <= proposal_number {
                    self.accepted_proposal_number = proposal_number;
                    let key = split_msg[1].to_string();
                    return if split_msg.len() == 3 {
                        let value = split_msg[2].to_string();
                        self.accepted_value = (Some(key), Some(value));
                        let msg = format!(
                            "{} {} {} {}",
                            char::from(MsgType::ACCEPTED as u8),
                            proposal_number,
                            self.accepted_value.0.as_ref().unwrap(),
                            self.accepted_value.1.as_ref().unwrap(),
                        );
                        Some(msg)
                    } else {
                        self.accepted_value = (Some(key), None);
                        let msg = format!(
                            "{} {} {}",
                            char::from(MsgType::ACCEPTED as u8),
                            proposal_number,
                            self.accepted_value.0.as_ref().unwrap(),
                        );
                        Some(msg)
                    };
                } else {
                    let msg = format!(
                        "{} {} {}",
                        char::from(MsgType::UNACCEPTED as u8),
                        proposal_number,
                        self.promised_proposal_number,
                    );
                    return Some(msg);
                }
            }
            _ => {}
        }
        None
    }
}

impl Role for Learner {
    fn new() -> Self {
        Learner {
            proposal_number: 0,
            kv_store: HashMap::new(),
        }
    }
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String> {
        let split_msg: Vec<&str> = msg.split(" ").filter(|&msg| msg != "").collect();
        match msg_type {
            MsgType::ACCEPTED => {
                let proposal_number = split_msg[0].parse::<u8>().unwrap();
                if proposal_number > self.proposal_number {
                    self.proposal_number = proposal_number;
                    // println!("client fuck{:?}", split_msg);
                    let key = split_msg[1].to_string();
                    return if split_msg.len() == 3 {
                        let value = split_msg[2].to_string();
                        self.kv_store.insert(key, value);
                        let msg = format!(
                            "{} {} {}",
                            char::from(MsgType::RESPONSE as u8),
                            proposal_number,
                            0,
                        );
                        Some(msg)
                    } else {
                        let value = self.kv_store.get(&key);
                        return if value.is_some() {
                            let msg = format!(
                                "{} {} {} {}",
                                char::from(MsgType::RESPONSE as u8),
                                proposal_number,
                                1,
                                value.unwrap(),
                            );
                            Some(msg)
                        } else {
                            let msg = format!(
                                "{} {} {}",
                                char::from(MsgType::RESPONSE as u8),
                                proposal_number,
                                2,
                            );
                            Some(msg)
                        };
                    };
                }
            }
            _ => {}
        }
        None
    }
}
