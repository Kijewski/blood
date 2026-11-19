// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;

use clap::Parser;

use crate::units::Unit;

/// Interpret a blood ketone value.
#[derive(Debug, Clone, Parser)]
#[command(version, about, propagate_version = true)]
pub(super) struct Ketone {
    /// Blood ketone value.
    #[arg()]
    ketone: f32,
    /// Unit of the ketone value.
    #[arg(default_value = "mmol/L")]
    unit: Unit,
}

impl Ketone {
    pub(super) fn show<W: Write + ?Sized>(self, f: &mut W) -> std::fmt::Result {
        let Self { ketone, unit } = self;

        writeln!(
            f,
            "Blood ketone level: {ketone:.1} {unit} = {:.1} {}.",
            unit.to_other(ketone),
            !unit,
        )?;
        writeln!(f)?;

        let value_mmol = unit.to(Unit::MmolPerLiter, ketone);
        let print_category = |f: &mut W, ranges: &[(f32, f32, &str)]| -> std::fmt::Result {
            for &(low, high, label) in ranges {
                let marker = if value_mmol >= low && value_mmol < high {
                    "*"
                } else {
                    " "
                };

                let low = unit.from(Unit::MmolPerLiter, low);
                let high = unit.from(Unit::MmolPerLiter, high);
                let value = match unit {
                    Unit::MgPerDeciliter => match (low.is_finite(), high.is_finite()) {
                        (true, true) => format_args!("{low:>2.0} - {high:>2.0}"),
                        (true, false) => format_args!("   > {low:>2.0}"),
                        (false, true) => format_args!("   < {high:>2.0}"),
                        (false, false) => unreachable!(),
                    },
                    Unit::MmolPerLiter => match (low.is_finite(), high.is_finite()) {
                        (true, true) => format_args!("{low:>3.1} - {high:>3.1}"),
                        (true, false) => format_args!("    > {low:>3.1}"),
                        (false, true) => format_args!("    < {high:>3.1}"),
                        (false, false) => unreachable!(),
                    },
                };
                writeln!(f, "{marker} {value} {unit} | {label}")?;
            }
            Ok(())
        };

        writeln!(f, "=== Ketone interpretation ===")?;
        print_category(f, &[
            (f32::NEG_INFINITY, 0.6, "normal / negative"),
            (0.6, 1.5, "low (mild ketosis)"),
            (1.5, 3.0, "moderate ketosis"),
            (3.0, f32::INFINITY, "high (ketoacidosis risk)"),
        ])?;

        Ok(())
    }
}
