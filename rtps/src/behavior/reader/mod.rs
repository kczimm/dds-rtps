use crate::structure::{
    historycache::HistoryCache,
    participant::{self, Endpoint},
    Guid,
};

use super::Duration;

pub mod stateful;
pub mod stateless;

pub struct Reader {
    guid: Guid,

    endpoint: Endpoint<participant::Reader>,

    expects_inline_qos: bool,
    heartbeat_response_delay: Duration,

    reader_cache: Box<HistoryCache>,
}
