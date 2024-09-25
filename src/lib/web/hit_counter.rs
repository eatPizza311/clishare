use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use parking_lot::Mutex;
use tokio::runtime::Handle;

use crate::data::DatabasePool;
use crate::service::{self, ServiceError};
use crate::ShortCode;

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        todo!()
    }
}
