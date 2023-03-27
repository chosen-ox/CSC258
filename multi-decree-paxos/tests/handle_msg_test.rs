use multi_decree_paxos::*;

#[test]
fn test_proposer() {
    let mut proposer = Proposer::new();
    // set f
    proposer.set_f(1);

    // test msg from client
    let msg = "put\nhello\nworld";
    if let Some(receive_msg) = proposer.send_prepare(msg) {
        let msg = format!("{} {}", char::from(MsgType::PREPARE as u8), 1);
        assert_eq!(msg, receive_msg);
    } else {
        assert!(false);
    }

    let msg = format!("{} {}", 1, 0);

    // test promise
    // 1st promise, not achieve quorum
    assert!(proposer.handle_msg(&MsgType::PROMISE, &msg).is_none());

    // 2nd promise, achieve quorum
    if let Some(receive_msg) = proposer.handle_msg(&MsgType::PROMISE, &msg) {
        let msg = format!(
            "{} {} {} {}",
            char::from(MsgType::ACCEPT as u8),
            1,
            "hello",
            "world"
        );
        assert_eq!(msg, receive_msg);
    } else {
        assert!(false);
    }
    // test response
    let msg = format!("{} {}", 1, 0);
    if let Some(receive_msg) = proposer.handle_msg(&MsgType::RESPONSE, &msg) {
        assert_eq!(receive_msg, "put successful!".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn test_acceptor() {
    let mut acceptor = Acceptor::new();

    // test prepare
    let msg = format!("{}", 1);
    if let Some(receive_msg) = acceptor.handle_msg(&MsgType::PREPARE, &msg) {
        let msg = format!("{} {} {}", char::from(MsgType::PROMISE as u8), 1, 0);
        assert_eq!(msg, receive_msg);
    } else {
        assert!(false);
    }

    // test accept
    let msg = format!("{} {} {}", 1, "hello", "world");
    if let Some(receive_msg) = acceptor.handle_msg(&MsgType::ACCEPT, &msg) {
        let msg = format!(
            "{} {} {} {}",
            char::from(MsgType::ACCEPTED as u8),
            1,
            "hello",
            "world"
        );
        assert_eq!(msg, receive_msg);
    } else {
        assert!(false);
    }
}

#[test]
fn test_learner() {
    let mut learner = Learner::new();

    // test accepted
    let msg = format!("{} {} {}", 1, "hello", "world");
    if let Some(receive_msg) = learner.handle_msg(&MsgType::ACCEPTED, &msg) {
        let msg = format!("{} {} {}", char::from(MsgType::RESPONSE as u8), 1, 0);
        assert_eq!(msg, receive_msg);
    } else {
        assert!(false);
    }
}

#[test]
fn test_paxos() {
    let mut proposer = Proposer::new();
    // set f
    proposer.set_f(1);
    let mut acceptor = Acceptor::new();
    let mut learner = Learner::new();
    // test msg from client
    let msg = "get\nhello";
    let prepare_msg = proposer.send_prepare(msg).unwrap();

    let promise_msg = acceptor
        .handle_msg(&MsgType::PREPARE, &prepare_msg[1..])
        .unwrap();

    proposer.handle_msg(&MsgType::PROMISE, &promise_msg[1..]);

    let accept_msg = proposer
        .handle_msg(&MsgType::PROMISE, &promise_msg[1..])
        .unwrap();

    let accepted_msg = acceptor
        .handle_msg(&MsgType::ACCEPT, &accept_msg[1..])
        .unwrap();

    let response_msg = learner
        .handle_msg(&MsgType::ACCEPTED, &accepted_msg[1..])
        .unwrap();

    let send_msg = proposer
        .handle_msg(&MsgType::RESPONSE, &response_msg[1..])
        .unwrap();

    assert_eq!(send_msg, "get failed!".to_string());
}
