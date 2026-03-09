### Do's & Don'ts — Realtime audio thread (Rust-specific, golden rules)

---

### DON'T (Realtime thread)
- **Do not allocate or deallocate on the realtime thread** (no `Box::new`, `Vec::push`, `String::push`, `Arc::clone` that triggers alloc, `Box::drop`, etc.).
- **Do not call `Vec::resize`, `Vec::push`, `Vec::extend`, `String::push_str`, or any API that may grow capacity.**
- **Do not call blocking synchronization primitives** (`Mutex::lock`, `RwLock::read`, `Condvar::wait`, `std::thread::park`, etc.).
- **Do not use `std::alloc::System` operations or other implicit heap ops** inside the audio callback.
- **Do not perform file or network I/O** (any std I/O), logging that allocates, or printing (`println!`) from the realtime thread.
- **Do not call functions that may lock internally** (e.g., standard library formatted I/O, global caches).
- **Do not cause unwraps/panics on the realtime thread**; avoid `panic!`, `.unwrap()`, `.expect()` in callback.
- **Do not use `Rc`/`Arc` clones that may allocate or trigger atomic contention** in the callback.
- **Do not perform dynamic linking or syscall-heavy operations** from the callback (DNS, file system metadata, etc.).
- **Do not touch cold, large memory regions for the first time** (avoid page faults) inside the callback.

---

### DO (Realtime thread safety)
- **Pre-allocate all buffers and objects** before starting realtime processing (use `Vec::with_capacity`, pre-filled `Box<[T]>`, fixed arrays).
- **Use lock-free SPSC ring buffers for audio frames** (one producer, one consumer).
- **Use fixed-size object pools / arena allocators** created on non-realtime thread; only return pointers/handles to the realtime thread.
- **Use non-allocating parameter queues** (small ring buffer or atomic single-slot for control values).
- **Return quickly from the callback**; do only sample math and copy to the provided buffer.
- **Use `core::sync::atomic` for shared counters/flags** (avoid heavier sync).
- **Mark callback functions as `extern "C"` or match host API signature exactly** when required by the audio backend.
- **Clear/zero output buffers on underrun** using deterministic memory operations (pre-allocated).
- **Detect and report overruns to a non-realtime thread** via a non-allocating signal (atomic flag or non-alloc SPSC queue).
- **Test under stress**: exercise heap and other threads while measuring callback duration and overruns.

---

### Rust-specific primitives & patterns (golden rules)
- **Use crossbeam::channel::ArrayQueue or a purpose-built SPSC ring** that guarantees no allocations on push/pop in the realtime path.
- **Prefer heap-allocated fixed slices (`Box<[T]>`) or stack arrays for sample buffers**; avoid resizing.
- **If using Arc, clone only on non-realtime thread; pass raw pointer/reference into callback** (ensuring safety and lifetime).
- **Avoid `std::sync::Mutex` in the realtime path; use atomic state or lock-free structures**.
- **Avoid `format!` and other allocation-producing macros** in callback; use preformatted/static messages or send codes to logger thread.
- **Use `MaybeUninit<T>` and explicit initialization when constructing objects ahead of time** to prevent hidden allocations or drops.
- **Drop-heavy objects only on non-realtime thread**; don't rely on implicit drops in the callback.
- **Use `#![no_std]` patterns or `alloc`-free coding style in critical modules if possible** to ensure zero-alloc behavior.

---

### Minimal non-alloc example patterns
- Pre-allocated buffer:
```
let mut audio_buf: Box<[f32]> = vec![0.0f32; BUFFER_SIZE].into_boxed_slice();
```

Atomic overun flag:
```
use std::sync::atomic::{AtomicBool, Ordering};
static OVERRUN: AtomicBool = AtomicBool::new(false);
// In callback:
OVERRUN.store(true, Ordering::Relaxed);
```
Simple SPSC using crossbeam (create on non-realtime thread; use try_pop/try_push without alloc in callback):
```
use crossbeam_queue::ArrayQueue;
//create: let q = Arc::new(ArrayQueue::<Frame>::new(QUEUE_CAP));
```
Quick checklist (one-line golden rules)
Pre-allocate everything.
No heap ops in callback.
No blocking locks in callback.
Use lock-free SPSC for audio & parameters.
Report to logger thread via non-alloc signal.
Avoid panics/unwraps in callback.
Measure callback worst-case latency.
