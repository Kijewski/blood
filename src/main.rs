// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

mod cli;
#[cfg(test)]
mod tests;
mod units;

use std::mem::DropGuard;

use clap::Parser;
use rustix::io::Errno;
use rustix::stdio::stdout;
use rustix::termios::{OptionalActions, tcgetattr, tcsetattr};

fn main() -> std::fmt::Result {
    struct WriteToIo<W: std::io::Write + ?Sized>(W);

    impl<W: std::io::Write + ?Sized> std::fmt::Write for WriteToIo<W> {
        #[inline]
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
        }
    }

    let _termios_guard = tcgetattr(stdout()).ok().map(|termios| {
        DropGuard::new(termios, |termios| {
            let _: Result<(), Errno> = tcsetattr(stdout(), OptionalActions::Flush, &termios);
        })
    });

    cli::Cli::parse().show(&mut WriteToIo(std::io::stdout()))
}
