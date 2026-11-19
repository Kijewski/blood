// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;
use std::num::ParseFloatError;
use std::process::exit;
use std::str::FromStr;

use clap::{CommandFactory, Parser};

use crate::units::Unit;

/// Calculate and interpret the Glucose Ketone Index (GKI).
///
/// The Glucose Ketone Index (GKI) is a metric used to track metabolic health,
/// particularly for those following ketogenic diets or managing diabetes.
/// It is calculated as: `GKI = glucose / ketones` (with unified units).
#[derive(Debug, Clone, Parser)]
#[command(version, about, propagate_version = true)]
pub(super) struct Gki {
    // FIXME: Is there a proper way to have optional positional arguments in between
    //        required arguments? The current implementation works, but hurts my eyes.
    //        Both the code, and the `--help` output.
    /// Glucose value, optional glucose unit, ketone value, optional ketone unit.
    #[arg(
        id = "<GLUCOSE> [<GLUCOSE_UNIT>] <KETONE> [<KETONE_UNIT>]",
        required = true,
        num_args(1),
        verbatim_doc_comment,
        long_help = "
Arguments:
  <GLUCOSE>       Blood glucose value
  [GLUCOSE_UNIT]  Unit of the glucose value [default: mg/dL]
  <KETONE>        Blood ketone value
  [KETONE_UNIT]   Unit of the ketone value [default: mmol/L]
"
    )]
    args: Vec<ValueOrUnit>,
}

impl Gki {
    pub(super) fn show<W: Write + ?Sized>(self, f: &mut W) -> std::fmt::Result {
        let ExpandedGki {
            glucose,
            glucose_unit,
            ketone,
            ketone_unit,
        } = ExpandedGki::new(self, f);

        let gki = glucose_unit.to(ketone_unit, glucose) / ketone;

        writeln!(f, "Glucose/Ketone Index (GKI): {gki:.1} mg/dL")?;
        writeln!(f)?;
        writeln!(
            f,
            "Blood glucose level: {glucose:.1} {glucose_unit} = {:.1} {}.",
            glucose_unit.to_other(glucose),
            !glucose_unit,
        )?;
        writeln!(
            f,
            "Blood ketone level: {ketone:.1} {ketone_unit} = {:.1} {}.",
            ketone_unit.to_other(ketone),
            !ketone_unit,
        )?;
        writeln!(f)?;

        let print_category = |f: &mut W, ranges: &[(f32, f32, &str)]| -> std::fmt::Result {
            for &(low, high, label) in ranges {
                let marker = if gki >= low && gki < high { "*" } else { " " };
                let high_str = if high.is_infinite() {
                    format_args!("    > {low:.1}")
                } else if low.is_infinite() {
                    format_args!("    < {high:.1}")
                } else {
                    format_args!("{low:.1} - {high:.1}")
                };
                writeln!(f, "{marker} {high_str} | {label}")?;
            }
            Ok(())
        };

        writeln!(f, "=== GKI Interpretation ===")?;
        print_category(f, &[
            (f32::NEG_INFINITY, 1.0, "therapeutic ketosis"),
            (1.0, 3.0, "high ketosis"),
            (3.0, 6.0, "moderate ketosis"),
            (6.0, 9.0, "low ketosis"),
            (9.0, f32::INFINITY, "no ketosis"),
        ])?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum ValueOrUnit {
    Value(f32),
    Unit(Unit),
}

impl FromStr for ValueOrUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(leading) = s.chars().next() else {
            return Err("argument must not be empty".into());
        };
        if leading.is_ascii_alphabetic() {
            Ok(Self::Unit(s.parse()?))
        } else {
            Ok(Self::Value(
                s.parse().map_err(|err: ParseFloatError| err.to_string())?,
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ExpandedGki {
    glucose: f32,
    glucose_unit: Unit,
    ketone: f32,
    ketone_unit: Unit,
}

impl ExpandedGki {
    fn new(args: Gki, f: &mut (impl Write + ?Sized)) -> Self {
        match *args.args.as_slice() {
            [ValueOrUnit::Value(glucose), ValueOrUnit::Value(ketone)] => Self {
                glucose,
                glucose_unit: Unit::MgPerDeciliter,
                ketone,
                ketone_unit: Unit::MmolPerLiter,
            },
            [
                ValueOrUnit::Value(glucose),
                ValueOrUnit::Unit(glucose_unit),
                ValueOrUnit::Value(ketone),
            ] => Self {
                glucose,
                glucose_unit,
                ketone,
                ketone_unit: Unit::MmolPerLiter,
            },
            [
                ValueOrUnit::Value(glucose),
                ValueOrUnit::Value(ketone),
                ValueOrUnit::Unit(ketone_unit),
            ] => Self {
                glucose,
                glucose_unit: Unit::MgPerDeciliter,
                ketone,
                ketone_unit,
            },
            [
                ValueOrUnit::Value(glucose),
                ValueOrUnit::Unit(glucose_unit),
                ValueOrUnit::Value(ketone),
                ValueOrUnit::Unit(ketone_unit),
            ] => Self {
                glucose,
                glucose_unit,
                ketone,
                ketone_unit,
            },
            _ => {
                let mut cmd = super::Cli::command();
                if let Some(cmd) = cmd.find_subcommand_mut("gki") {
                    let _: std::fmt::Result = writeln!(f, "{}", cmd.render_usage());
                }
                exit(1);
            }
        }
    }
}
