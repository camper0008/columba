pub mod create;
pub mod inbox;
pub mod read;
pub mod send;

pub mod handlers {
    pub use crate::api::create::create;
    pub use crate::api::inbox::inbox;
    pub use crate::api::read::read;
    pub use crate::api::send::send;
}
