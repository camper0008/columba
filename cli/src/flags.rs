use std::path::PathBuf;

pub enum Flag {
    Help,
    Keyring { location: PathBuf },
    Url { location: String },
}
