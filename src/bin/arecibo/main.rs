//! Main entry point for Arecibo

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use arecibo::application::APP;

/// Boot Arecibo
fn main() {
    abscissa_core::boot(&APP);
}
