// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;

use clap::Parser;

use crate::units::Unit;

/// Interpret a blood glucose value.
#[derive(Debug, Clone, Parser)]
#[command(version, about, propagate_version = true)]
pub(super) struct Glucose {
    /// Blood glucose value.
    #[arg()]
    glucose: f32,
    /// Unit of the glucose value.
    #[arg(default_value = "mg/dL")]
    unit: Unit,
}

impl Glucose {
    pub(super) fn show<W: Write + ?Sized>(self, f: &mut W) -> std::fmt::Result {
        let Self { glucose, unit } = self;

        writeln!(
            f,
            "Blood glucose level: {glucose:.1} {unit} = {:.1} {}.",
            unit.to_other(glucose),
            !unit,
        )?;
        writeln!(f)?;

        let value_mgdl = unit.to(Unit::MgPerDeciliter, glucose);
        let print_category = |f: &mut W, ranges: &[(f32, f32, &str)]| -> std::fmt::Result {
            for &(low, high, label) in ranges {
                let marker = if value_mgdl >= low && value_mgdl < high {
                    "*"
                } else {
                    " "
                };

                let low = unit.from(Unit::MgPerDeciliter, low);
                let high = unit.from(Unit::MgPerDeciliter, high);
                let value = match unit {
                    Unit::MgPerDeciliter => match (low.is_finite(), high.is_finite()) {
                        (true, true) => format_args!("{low:>3.0} - {high:>3.0}"),
                        (true, false) => format_args!("    > {low:>3.0}"),
                        (false, true) => format_args!("    < {high:>3.0}"),
                        (false, false) => unreachable!(),
                    },
                    Unit::MmolPerLiter => match (low.is_finite(), high.is_finite()) {
                        (true, true) => format_args!("{low:>4.1} - {high:>4.1}"),
                        (true, false) => format_args!("     > {low:>4.1}"),
                        (false, true) => format_args!("     < {high:>4.1}"),
                        (false, false) => unreachable!(),
                    },
                };
                writeln!(f, "{marker} {value} {unit} | {label}")?;
            }
            Ok(())
        };

        writeln!(f, "=== Fasting (>= 8h after last meal) ===")?;
        print_category(f, &[
            (f32::NEG_INFINITY, 60.0, "very low"),
            (60.0, 70.0, "low"),
            (70.0, 100.0, "normal"),
            (100.0, 125.0, "elevated (pre-diabetes)"),
            (125.0, f32::INFINITY, "diabetes suspected"),
        ])?;

        writeln!(f)?;
        writeln!(f, "=== 2h after last meal ===")?;
        print_category(f, &[
            (f32::NEG_INFINITY, 60.0, "very low"),
            (60.0, 100.0, "low"),
            (100.0, 140.0, "normal"),
            (140.0, 200.0, "elevated (pre-diabetes)"),
            (200.0, f32::INFINITY, "diabetes suspected"),
        ])?;

        writeln!(f)?;
        writeln!(f, "=== Right after last meal ===")?;
        print_category(f, &[
            (f32::NEG_INFINITY, 60.0, "very low"),
            (60.0, 100.0, "low"),
            (100.0, 180.0, "normal"),
            (180.0, 250.0, "elevated"),
            (250.0, f32::INFINITY, "very high"),
        ])?;

        Ok(())
    }
}
