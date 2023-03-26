use std::collections::HashMap;

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
}

pub trait Role {
    fn new() -> Self;
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String>;
}

impl Proposer {
    pub fn set_f(&mut self, f: u8) {
        self.f = f;
    }

    pub fn reset(&mut self) {
        self.suggested_proposal_number = 0;
        self.suggested_value = (None, None);
        self.wait_for_promise = false;
        self.wait_for_accepted = false;
        self.wait_for_response = false;
        self.promise_vote_count = 0;
        self.accepted_vote_count = 0;
        self.proposal_number = 1;
        self.proposal_value = (None, None);
        self.nack_count = 0;
        self.unaccepted_count = 0;
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
            proposal_number: 1,
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
                    println!("{}",self.f);

                    if self.promise_vote_count >= self.f + 1 {
                        println!("Got enough promises");
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
            MsgType::RESPONSE => {}
            MsgType::ACCEPTED => {}
            MsgType::UNACCEPTED => {}
            MsgType::NACK => {}
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
                        "{} {}",
                        char::from(MsgType::NACK as u8),
                        self.promised_proposal_number,
                    );
                    return Some(msg);
                }
            }
            MsgType::ACCEPT => {}
            _ => {}
        }
        None
    }
}

impl Role for Learner {
    fn new() -> Self {
        Learner {
            kv_store: HashMap::new(),
        }
    }
    fn handle_msg(&mut self, msg_type: &MsgType, msg: &str) -> Option<String> {
        let split_msg: Vec<&str> = msg.split(" ").filter(|&msg| msg != "").collect();
        match msg_type {
            MsgType::ACCEPTED => {
                let key = split_msg[0].to_string();
                let value = split_msg[1].to_string();
                self.kv_store.insert(key, value);
            }
            _ => {}
        }
        None
    }
}
