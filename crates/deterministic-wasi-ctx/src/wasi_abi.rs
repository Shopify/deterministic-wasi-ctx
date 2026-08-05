//! The WASI Preview 1 structs used by `poll_oneoff`.

pub(crate) const ERRNO_SUCCESS: u16 = 0;
pub(crate) const EVENTTYPE_CLOCK: u8 = 0;
pub(crate) const EVENTTYPE_FD_READ: u8 = 1;
pub(crate) const EVENTTYPE_FD_WRITE: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct EventFdReadwrite {
    pub(crate) nbytes: u64,
    pub(crate) flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Event {
    pub(crate) userdata: u64,
    pub(crate) error: u16,
    pub(crate) type_: u8,
    pub(crate) fd_readwrite: EventFdReadwrite,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SubscriptionData {
    clock: SubscriptionClock,
    fd_read: SubscriptionFdReadwrite,
    fd_write: SubscriptionFdReadwrite,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SubscriptionClock {
    id: u32,
    timeout: u64,
    precision: u64,
    flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SubscriptionFdReadwrite {
    file_descriptor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SubscriptionU {
    pub(crate) tag: u8,
    u: SubscriptionData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Subscription {
    pub(crate) userdata: u64,
    pub(crate) u: SubscriptionU,
}

#[cfg(test)]
mod tests {
    use super::{Event, Subscription};

    #[test]
    fn record_sizes_match_wasi_preview_1_abi() {
        assert_eq!(std::mem::size_of::<Event>(), 32);
        assert_eq!(std::mem::size_of::<Subscription>(), 48);
    }
}
