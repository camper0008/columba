use columba_client::parse::{parse_response, InboxItem, Response};

fn create_res(msg: &str) -> Vec<String> {
    vec!["===BEGIN_CREATE_RES===", "msg", msg, "===END_CREATE_RES==="]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
}

fn inbox_res(success: bool) -> Vec<String> {
    (if success {
        vec![
            "===BEGIN_INBOX_RES===",
            "msg",
            "success",
            "inbox",
            "id0 2022-04-27T12:15:17.352Z",
            "0id 2022-04-27T14:15:17.352Z",
            "===END_INBOX_RES===",
        ]
    } else {
        vec![
            "===BEGIN_INBOX_RES===",
            "msg",
            "error",
            "===END_INBOX_RES===",
        ]
    })
    .into_iter()
    .map(|s| String::from(s))
    .collect()
}

fn read_res(success: bool) -> Vec<String> {
    (if success {
        vec![
            "===BEGIN_READ_RES===",
            "msg",
            "success",
            "read",
            "abc",
            "def",
            "===END_READ_RES===",
        ]
    } else {
        vec!["===BEGIN_READ_RES===", "msg", "error", "===END_READ_RES==="]
    })
    .into_iter()
    .map(|s| String::from(s))
    .collect()
}

fn send_res(msg: &str) -> Vec<String> {
    vec!["===BEGIN_SEND_RES===", "msg", msg, "===END_SEND_RES==="]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_res() {
        let abc_res = parse_response(create_res("abc"))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            abc_res,
            Response::Create {
                msg: String::from("abc")
            }
        );
        let def_res = parse_response(create_res("def"))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            def_res,
            Response::Create {
                msg: String::from("def")
            }
        );
    }

    #[test]
    fn test_inbox_res_success() {
        let success_res = parse_response(inbox_res(true))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            success_res,
            Response::Inbox {
                msg: String::from("success"),
                inbox: Some(vec![
                    InboxItem {
                        id: String::from("id0"),
                        date: String::from("2022-04-27T12:15:17.352Z")
                    },
                    InboxItem {
                        id: String::from("0id"),
                        date: String::from("2022-04-27T14:15:17.352Z")
                    }
                ]),
            }
        );
    }

    #[test]
    fn test_inbox_res_error() {
        let error_res = parse_response(inbox_res(false))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            error_res,
            Response::Inbox {
                msg: String::from("error"),
                inbox: None,
            }
        );
        let error_res = parse_response(inbox_res(false))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            error_res,
            Response::Inbox {
                msg: String::from("error"),
                inbox: None,
            }
        );
    }

    #[test]
    fn test_read_res_success() {
        let success_res = parse_response(read_res(true))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            success_res,
            Response::Read {
                msg: String::from("success"),
                read: Some(String::from("abc\ndef")),
            }
        );
    }
    #[test]
    fn test_read_res_error() {
        let error_res = parse_response(read_res(false))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            error_res,
            Response::Read {
                msg: String::from("error"),
                read: None,
            }
        );
    }

    #[test]
    fn test_send_res() {
        let abc_res = parse_response(send_res("abc"))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            abc_res,
            Response::Send {
                msg: String::from("abc")
            }
        );
        let def_res = parse_response(send_res("def"))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            def_res,
            Response::Send {
                msg: String::from("def")
            }
        );
    }

    #[test]
    fn test_combined_parse() {
        let mut res_iter = parse_response(
            vec![
                create_res("abc"),
                inbox_res(true),
                read_res(true),
                send_res("def"),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .into_iter();

        let create = res_iter.next().unwrap();
        assert_eq!(
            create,
            Response::Create {
                msg: String::from("abc")
            }
        );

        let inbox = res_iter.next().unwrap();
        assert_eq!(
            inbox,
            Response::Inbox {
                msg: String::from("success"),
                inbox: Some(vec![
                    InboxItem {
                        id: String::from("id0"),
                        date: String::from("2022-04-27T12:15:17.352Z")
                    },
                    InboxItem {
                        id: String::from("0id"),
                        date: String::from("2022-04-27T14:15:17.352Z")
                    }
                ]),
            }
        );

        let read = res_iter.next().unwrap();
        assert_eq!(
            read,
            Response::Read {
                msg: String::from("success"),
                read: Some(String::from("abc\ndef")),
            }
        );

        let send = res_iter.next().unwrap();
        assert_eq!(
            send,
            Response::Send {
                msg: String::from("def")
            }
        );
    }
}
