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

fn parse_create(_iter: &mut IntoIter<String>) -> Response {
    unimplemented!()
}
fn parse_inbox(_iter: &mut IntoIter<String>) -> Response {
    unimplemented!()
}
fn parse_read(_iter: &mut IntoIter<String>) -> Response {
    unimplemented!()
}
fn parse_send(_iter: &mut IntoIter<String>) -> Response {
    unimplemented!()
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
