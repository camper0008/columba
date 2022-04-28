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
}

fn parse_create(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg_value = iter
        .next()
        .expect("recieved invalid inbox response from server");
    Response::Create { msg: msg_value }
}

fn parse_inbox_success(iter: &mut IntoIter<String>) -> Response {
    let _inbox_field = iter.next();
    let inbox = iter
        .take_while(|l| l != "===END_INBOX_RES===")
        .map(|l| {
            let mut split = l.splitn(2, " ");
            let id = split
                .next()
                .expect("recieved invalid inbox response from server");
            let date = split
                .next()
                .expect("recieved invalid inbox response from server");
            InboxItem {
                id: id.to_string(),
                date: date.to_string(),
            }
        })
        .collect();

    Response::Inbox {
        msg: String::from("success"),
        inbox: Some(inbox),
    }
}

fn parse_inbox(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg_value = iter
        .next()
        .expect("recieved invalid inbox response from server");

    if msg_value != "success" {
        return Response::Inbox {
            msg: msg_value,
            inbox: None,
        };
    }

    parse_inbox_success(iter)
}
fn parse_read(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg_value = iter
        .next()
        .expect("recieved invalid inbox response from server");

    if msg_value != "success" {
        return Response::Read {
            msg: msg_value,
            read: None,
        };
    }

    let _read_field = iter.next();
    let read = iter
        .take_while(|l| l != "===END_READ_RES===")
        .reduce(|acc, n| acc + "\n" + &n);

    Response::Read {
        msg: msg_value,
        read,
    }
}
fn parse_send(iter: &mut IntoIter<String>) -> Response {
    let _msg_field = iter.next();
    let msg_value = iter
        .next()
        .expect("recieved invalid inbox response from server");
    Response::Send { msg: msg_value }
}

pub fn parse_response(lines: Vec<String>) -> Vec<Response> {
    const CREATE_HEADER: &str = "===BEGIN_CREATE_RES===";
    const INBOX_HEADER: &str = "===BEGIN_INBOX_RES===";
    const READ_HEADER: &str = "===BEGIN_READ_RES===";
    const SEND_HEADER: &str = "===BEGIN_SEND_RES===";

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
            _ => Response::Unknown,
        });
    }

    res.into_iter()
        .filter(|r| *r != Response::Unknown)
        .collect()
}
