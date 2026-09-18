// Tests that an unknown `#[spirv(..)]` argument is named in the diagnostic, and
// that a close match is suggested. When a proc macro stamps its own invocation
// span onto generated tokens the span points at the macro call rather than the
// argument, so the name is the only thing identifying what was rejected.

// build-fail

use spirv_std::spirv;

#[spirv(fragmnt)]
fn _typo_entry() {}

#[spirv(vertex)]
fn _typo_param(#[spirv(uinform)] _: ()) {}

#[spirv(vertex)]
fn _no_close_match(#[spirv(zzzzzzzz)] _: ()) {}

fn main() {}
