// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

use crate::cli::Cli;

/// Supply subcommand `gki` with all parameters.
#[test]
fn test_gki_with_all_values() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "gki", "67", "mg/dL", "1.2", "mmol/L"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert_eq!(
        f,
        "\
Glucose/Ketone Index (GKI): 3.1 mg/dL

Blood glucose level: 67.0 mg/dL = 3.7 mmol/L.
Blood ketone level: 1.2 mmol/L = 21.6 mg/dL.

=== GKI Interpretation ===
      < 1.0 | therapeutic ketosis
  1.0 - 3.0 | high ketosis
* 3.0 - 6.0 | moderate ketosis
  6.0 - 9.0 | low ketosis
      > 9.0 | no ketosis
"
    );
}

/// Supply subcommand `gki` with glucose and ketones only (defaults).
#[test]
fn test_gki_with_defaults() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "gki", "100", "1.0"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert_eq!(
        f,
        "\
Glucose/Ketone Index (GKI): 5.6 mg/dL

Blood glucose level: 100.0 mg/dL = 5.6 mmol/L.
Blood ketone level: 1.0 mmol/L = 18.0 mg/dL.

=== GKI Interpretation ===
      < 1.0 | therapeutic ketosis
  1.0 - 3.0 | high ketosis
* 3.0 - 6.0 | moderate ketosis
  6.0 - 9.0 | low ketosis
      > 9.0 | no ketosis
"
    );
}

/// Supply subcommand `gki` with glucose unit and ketones.
#[test]
fn test_gki_with_glucose_unit() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "gki", "5.5", "mmol/L", "1.0"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert_eq!(
        f,
        "\
Glucose/Ketone Index (GKI): 5.5 mg/dL

Blood glucose level: 5.5 mmol/L = 99.1 mg/dL.
Blood ketone level: 1.0 mmol/L = 18.0 mg/dL.

=== GKI Interpretation ===
      < 1.0 | therapeutic ketosis
  1.0 - 3.0 | high ketosis
* 3.0 - 6.0 | moderate ketosis
  6.0 - 9.0 | low ketosis
      > 9.0 | no ketosis
"
    );
}

/// Supply subcommand `gki` with glucose and ketones unit.
#[test]
fn test_gki_with_ketone_unit() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "gki", "100", "18", "mg/dL"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert_eq!(
        f,
        "\
Glucose/Ketone Index (GKI): 5.6 mg/dL

Blood glucose level: 100.0 mg/dL = 5.6 mmol/L.
Blood ketone level: 18.0 mg/dL = 1.0 mmol/L.

=== GKI Interpretation ===
      < 1.0 | therapeutic ketosis
  1.0 - 3.0 | high ketosis
* 3.0 - 6.0 | moderate ketosis
  6.0 - 9.0 | low ketosis
      > 9.0 | no ketosis
"
    );
}

/// Test glucose interpretation with normal fasting value.
#[test]
fn test_glucose_fasting_normal() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "glucose", "90"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*  70 - 100 mg/dL | normal"));
    assert!(f.contains("Fasting"));
    assert!(f.contains("2h after last meal"));
    assert!(f.contains("Right after last meal"));
}

/// Test glucose interpretation with low value.
#[test]
fn test_glucose_low() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "glucose", "55"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*     <  60 mg/dL | very low"));
}

/// Test glucose interpretation with high value.
#[test]
fn test_glucose_high() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "glucose", "150"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*     > 125 mg/dL | diabetes suspected"));
}

/// Test glucose with mmol/L unit.
#[test]
fn test_glucose_mmol() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "glucose", "5.0", "mmol/L"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*  3.9 -  5.6 mmol/L | normal"));
}

/// Test ketone interpretation with normal value.
#[test]
fn test_ketone_normal() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "ketone", "0.3"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*     < 0.6 mmol/L | normal / negative"));
}

/// Test ketone interpretation with mild ketosis.
#[test]
fn test_ketone_mild() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "ketone", "1.0"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("* 0.6 - 1.5 mmol/L | low (mild ketosis)"));
}

/// Test ketone interpretation with moderate ketosis.
#[test]
fn test_ketone_moderate() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "ketone", "2.0"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("* 1.5 - 3.0 mmol/L | moderate ketosis"));
}

/// Test ketone interpretation with high ketosis.
#[test]
fn test_ketone_high() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "ketone", "4.0"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("*     > 3.0 mmol/L | high (ketoacidosis risk)"));
}

/// Test ketone with mg/dL unit.
#[test]
fn test_ketone_mgdl() {
    let mut f = String::new();
    Cli::try_parse_from([PROG_NAME, "ketone", "18", "mg/dL"])
        .unwrap()
        .show(&mut f)
        .unwrap();
    assert!(f.contains("* 11 - 27 mg/dL | low (mild ketosis)"));
}

const PROG_NAME: &str = "blood";
