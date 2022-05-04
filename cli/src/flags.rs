use std::env;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum Flag {
    Temp,
    Help,
    Keyring { location: PathBuf },
    Url { location: String },
    Unknown { flag: String },
}

pub fn parse() -> Vec<Flag> {
    env::args()
        .skip(1)
        .scan(None, |state, c| {
            if state.is_none() {
                if c == "-h" || c == "--help" {
                    return Some(Flag::Help);
                }
                *state = Some(c);

                Some(Flag::Temp)
            } else {
                let flag_clone = state.clone().unwrap();
                let flag: &str = flag_clone.as_ref();

                *state = None;

                match flag {
                    "-u" | "--url" => Some(Flag::Url { location: c }),
                    "-k" | "--keyring" => Some(Flag::Keyring {
                        location: PathBuf::from(c),
                    }),
                    other => Some(Flag::Unknown {
                        flag: other.to_string(),
                    }),
                }
            }
        })
        .filter(|x| *x != Flag::Temp)
        .collect()
}
