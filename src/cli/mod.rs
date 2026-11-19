// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

mod gki;
mod glucose;
mod ketone;

use std::fmt::Write;

use clap::Parser;

/// Calculate and interpret the Glucose Ketone Index (GKI) and related metrics.
///
/// This tool is for informational purposes only. It does not provide medical advice.
#[allow(private_interfaces)] // There is no need to look behind the curtain.
#[derive(Debug, Clone, Parser)]
#[command(version, about, propagate_version = true)]
pub(super) enum Cli {
    Glucose(glucose::Glucose),
    Ketone(ketone::Ketone),
    Gki(gki::Gki),
}

impl Cli {
    pub(super) fn show<W: Write + ?Sized>(self, f: &mut W) -> std::fmt::Result {
        match self {
            Cli::Glucose(args) => args.show(f),
            Cli::Ketone(args) => args.show(f),
            Cli::Gki(args) => args.show(f),
        }
    }
}
