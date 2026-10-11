// Keep a crash from raising the macOS Crash Reporter dialog.
//
// A process killed by SIGABRT (an explicit abort(), a panic while panicking,
// or Rust's stack-overflow handler, which prints a message and then aborts)
// gets the "quit unexpectedly" dialog on macOS. Handling the signal and
// _exit()ing instead ends the process with a plain exit status, which is all
// the harness needs: the runner protocol only looks for "died without
// producing an outcome". Rust's own SIGSEGV handler runs on an alternate
// stack and then calls abort(), so this covers stack overflows too without
// touching SIGSEGV.
//
// This file is compiled into the harness and also embedded verbatim in the
// generated runner binary, so it must stay dependency-free.

const SIGABRT: i32 = 6;
const SIGABRT_EXIT: i32 = 128 + SIGABRT;

unsafe extern "C" {
    fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    fn _exit(code: i32) -> !;
}

extern "C" fn on_abort(_sig: i32) {
    unsafe { _exit(SIGABRT_EXIT) }
}

/// Install the SIGABRT handler. Call once, early in `main`.
pub fn install() {
    unsafe {
        signal(SIGABRT, on_abort);
    }
}
