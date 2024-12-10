use std::io;
use std::io::prelude::*;

/// Pause execution for debugging
/// https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/4
pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}