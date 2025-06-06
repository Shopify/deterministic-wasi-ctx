// Specifying these types in here because the wasmtime-wasi definitions do not
// use a C-style memory layout (i.e., `#[repr(C)]`) which causes them to be
// serialized to memory incorrectly. The definitions in the `wasi` crate are
// nice but that crate unconditionally links against WASI imports which causes
// Rust projects using this crate on Windows to fail to link.

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct SubscriptionClock {
    /// The clock against which to compare the timestamp.
    pub id: u32,
    /// The absolute or relative timestamp.
    pub timeout: u64,
    /// The amount of time that the implementation may wait additionally
    /// to coalesce with other events.
    pub precision: u64,
    /// Flags specifying whether the timeout is absolute or relative
    pub flags: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct SubscriptionFdReadwrite {
    /// The file descriptor on which to wait for it to become ready for reading or writing.
    pub file_descriptor: u32,
}

#[repr(C)]
pub(super) union SubscriptionUU {
    pub clock: SubscriptionClock,
    pub fd_read: SubscriptionFdReadwrite,
    pub fd_write: SubscriptionFdReadwrite,
}

#[repr(C)]
pub(super) struct SubscriptionU {
    pub tag: u8,
    pub u: SubscriptionUU,
}

#[repr(C)]
pub(super) struct Subscription {
    /// User-provided value that is attached to the subscription in the
    /// implementation and returned through `event::userdata`.
    pub userdata: u64,
    /// The type of the event to which to subscribe, and its contents
    pub u: SubscriptionU,
}

#[repr(C)]
pub(super) struct EventFdReadwrite {
    /// The number of bytes available for reading or writing.
    pub nbytes: u64,
    /// The state of the file descriptor.
    pub flags: u16,
}

#[repr(C)]
pub(super) struct Event {
    /// User-provided value that got attached to `subscription::userdata`.
    pub userdata: u64,
    /// If non-zero, an error that occurred while processing the subscription request.
    pub error: u16,
    /// The type of event that occured
    pub type_: u8,
    /// The contents of the event, if it is an `eventtype::fd_read` or
    /// `eventtype::fd_write`. `eventtype::clock` events ignore this field.
    pub fd_readwrite: EventFdReadwrite,
}
