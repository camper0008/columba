use std::vec::IntoIter;

#[derive(PartialEq, Debug)]
pub struct InboxItem {
    pub date: String,
    pub id: String,
}

#[derive(PartialEq, Debug)]
pub enum Response {
    Unknown,
    Create {
        msg: String,
    },
    Inbox {
        msg: String,
        inbox: Option<Vec<InboxItem>>,
    },
    Read {
        msg: String,
        read: Option<String>,
    },
    Send {
        msg: String,
    },
    Whois {
        msg: String,
        public: Option<String>,
    },
}

fn assert_end_header_valid(iter: &mut IntoIter<String>, header: &str) {
    let res_end_header = iter.next();
    assert_eq!(res_end_header.expect("end header does not exist"), header);
}

fn parse_create(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg = iter
        .next()
        .expect("recieved invalid create response from server");

    assert_end_header_valid(iter, "===END_CREATE_RES===");

    Response::Create { msg }
}

fn split_inbox_message(line: String) -> (String, String) {
    let mut split = line.splitn(2, " ");

    let id = split
        .next()
        .expect("recieved invalid inbox response from server")
        .to_string();

    let date = split
        .next()
        .expect("recieved invalid inbox response from server")
        .to_string();

    (id, date)
}

fn parse_inbox_success(iter: &mut IntoIter<String>) -> Response {
    let _inbox_field = iter.next();
    let inbox = iter
        .take_while(|line| line != "===END_INBOX_RES===")
        .map(|line| {
            let (id, date) = split_inbox_message(line);
            InboxItem { id, date }
        })
        .collect();

    Response::Inbox {
        msg: String::from("success"),
        inbox: Some(inbox),
    }
}

fn parse_inbox(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg = iter
        .next()
        .expect("recieved invalid inbox response from server");

    if msg != "success" {
        return Response::Inbox { msg, inbox: None };
    }

    parse_inbox_success(iter)
}
fn parse_read(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg = iter
        .next()
        .expect("recieved invalid read response from server");

    if msg != "success" {
        return Response::Read { msg, read: None };
    }

    let _read_field = iter.next();
    let read = iter
        .take_while(|l| l != "===END_READ_RES===")
        .reduce(|acc, n| acc + "\n" + &n);

    Response::Read { msg, read }
}
fn parse_send(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg = iter
        .next()
        .expect("recieved invalid send response from server");

    assert_end_header_valid(iter, "===END_SEND_RES===");

    Response::Send { msg }
}

fn parse_whois(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg = iter
        .next()
        .expect("recieved invalid whois response from server");

    if msg != "success" {
        return Response::Whois { msg, public: None };
    }

    let _public_field = iter.next();
    let public = iter
        .take_while(|l| l != "===END_WHOIS_RES===")
        .reduce(|acc, n| acc + "\n" + &n);

    Response::Whois { msg, public }
}

pub fn parse_response(lines: Vec<String>) -> Vec<Response> {
    const CREATE_HEADER: &str = "===BEGIN_CREATE_RES===";
    const INBOX_HEADER: &str = "===BEGIN_INBOX_RES===";
    const READ_HEADER: &str = "===BEGIN_READ_RES===";
    const SEND_HEADER: &str = "===BEGIN_SEND_RES===";
    const WHOIS_HEADER: &str = "===BEGIN_WHOIS_RES===";

    let mut res = Vec::new();

    let mut lines_iter = lines.into_iter();
    loop {
        let line_option = lines_iter.next();
        if line_option.is_none() {
            break;
        }
        let line = line_option.unwrap();
        res.push(match line.as_str() {
            CREATE_HEADER => parse_create(&mut lines_iter),
            INBOX_HEADER => parse_inbox(&mut lines_iter),
            READ_HEADER => parse_read(&mut lines_iter),
            SEND_HEADER => parse_send(&mut lines_iter),
            WHOIS_HEADER => parse_whois(&mut lines_iter),
            _ => Response::Unknown,
        });
    }

    res.into_iter()
        .filter(|r| *r != Response::Unknown)
        .collect()
}
