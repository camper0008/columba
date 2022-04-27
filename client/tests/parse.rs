use columba_client::parse::{parse_response, Response};

fn create_res(msg: &str) -> Vec<String> {
    vec!["===BEGIN_CREATE_RES===", "msg", msg, "===END_CREATE_RES==="]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
}

fn inbox_res(success: bool) -> Vec<String> {
    (if success {
        vec![
            "===BEGIN_CREATE_RES===",
            "msg",
            "success",
            "inbox",
            "id0 2022-04-27T12:15:17.352Z",
            "0id 2022-04-27T14:15:17.352Z",
            "===END_CREATE_RES===",
        ]
    } else {
        vec![
            "===BEGIN_CREATE_RES===",
            "msg",
            "error",
            "===END_CREATE_RES===",
        ]
    })
    .into_iter()
    .map(|s| String::from(s))
    .collect()
}

fn read_res(success: bool) -> Vec<String> {
    (if success {
        vec![
            "===BEGIN_CREATE_RES===",
            "msg",
            "success",
            "read",
            "abcdefghijklmnop",
            "qrestu",
            "===END_CREATE_RES===",
        ]
    } else {
        vec![
            "===BEGIN_CREATE_RES===",
            "msg",
            "error",
            "===END_CREATE_RES===",
        ]
    })
    .into_iter()
    .map(|s| String::from(s))
    .collect()
}

fn send_res(msg: &str) -> Vec<String> {
    vec!["===BEGIN_CREATE_RES===", "msg", msg, "===END_CREATE_RES==="]
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
    fn test_send_res() {
        let abc_res = parse_response(send_res("abc"))
            .into_iter()
            .nth(0)
            .expect("parse response returned empty vec");
        assert_eq!(
            abc_res,
            Response::Create {
                msg: String::from("abc")
            }
        );
        let def_res = parse_response(send_res("def"))
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
}
