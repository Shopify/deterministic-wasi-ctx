use std::{mem, ptr, slice};

use anyhow::{anyhow, Result};
use wasmtime::{Caller, Linker};

use wasi_structs::{Event, EventFdReadwrite, Subscription};

mod wasi_structs;

/// Adds implementations for WASI preview 1 `poll_oneoff` and `sched_yield` to
/// the linker which will return immediately.
/// Note: This function will enable shadowing on the linker.
pub fn replace_scheduling_functions<T>(linker: &mut Linker<T>) -> Result<()>
where
    T: Send,
{
    override_scheduling_functions(linker, "wasi_snapshot_preview1")
}

/// Adds implementations for WASI preview 0 `poll_oneoff` and `sched_yield` to
/// the linker which will return immediately.
/// Note: This function will enable shadowing on the linker.
pub fn replace_scheduling_functions_for_wasi_preview_0<T>(linker: &mut Linker<T>) -> Result<()>
where
    T: Send,
{
    override_scheduling_functions(linker, "wasi_unstable")
}

fn override_scheduling_functions<T>(linker: &mut Linker<T>, module: &str) -> Result<()> {
    linker.allow_shadowing(true);
    linker.func_wrap(
        module,
        "poll_oneoff",
        |mut caller: Caller<'_, T>,
         in_ptr: i32,
         out_ptr: i32,
         nsubscriptions: i32,
         nevents_ptr: i32|
         -> anyhow::Result<i32> {
            let in_ptr = in_ptr as usize;
            let out_ptr = out_ptr as usize;
            let nsubscriptions = nsubscriptions as usize;
            let nevents_ptr = nevents_ptr as usize;
            // See https://github.com/WebAssembly/WASI/blob/3d5e0553cd01dd4d6e2c06ad2a702ee9dda17b7f/legacy/tools/witx-docs.md#pointers
            let memory = caller
                .get_export("memory")
                .map_or_else(|| Err(anyhow!("missing required memory export")), Ok)?
                .into_memory()
                .map_or_else(|| Err(anyhow!("missing required memory export")), Ok)?;

            for i in 0..nsubscriptions {
                // Read the `Subscription` from memory from `in_ptr`.
                let offset = in_ptr + (i * mem::size_of::<Subscription>());
                let mut subscription_buffer = [0u8; mem::size_of::<Subscription>()];
                memory.read(&caller, offset, &mut subscription_buffer)?;
                let subscription =
                    unsafe { ptr::read(subscription_buffer.as_ptr() as *const Subscription) };

                // Create a successful `Event` for each subscription.
                let event = Event {
                    userdata: subscription.userdata,
                    error: 0,
                    // See https://github.com/WebAssembly/wasi-libc/blob/e9524a0980b9bb6bb92e87a41ed1055bdda5bb86/libc-bottom-half/headers/public/wasi/api.h#L1100-L1121
                    // for the mapping between the integers and the event type.
                    type_: subscription.u.tag,
                    fd_readwrite: EventFdReadwrite {
                        nbytes: 0,
                        flags: 0,
                    },
                };

                // Write the event into memory at `out_ptr`.
                let offset = out_ptr + (i * mem::size_of::<Event>());
                let event_buffer = unsafe {
                    slice::from_raw_parts(
                        &event as *const Event as *const u8,
                        mem::size_of::<Event>(),
                    )
                };
                memory.write(&mut caller, offset, event_buffer)?
            }

            // Copy number of subscriptions into number of events pointer.
            let buffer = nsubscriptions.to_le_bytes();
            memory.write(&mut caller, nevents_ptr, &buffer)?;

            Ok(0)
        },
    )?;

    linker.func_wrap(module, "sched_yield", || 0)?;

    Ok(())
}
