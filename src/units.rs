// SPDX-FileCopyrightText: 2026 René Kijewski <crates.io@k6i.de>
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Unit {
    #[default]
    MgPerDeciliter,
    MmolPerLiter,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Self::MgPerDeciliter => "mg/dL",
            Self::MmolPerLiter => "mmol/L",
        })
    }
}

impl ops::Not for Unit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::MgPerDeciliter => Self::MmolPerLiter,
            Self::MmolPerLiter => Self::MgPerDeciliter,
        }
    }
}

impl Unit {
    pub(crate) fn to_other(self, value: f32) -> f32 {
        self.to(!self, value)
    }

    pub(crate) fn from(self, source: Self, value: f32) -> f32 {
        source.to(self, value)
    }

    pub(crate) fn to(self, dest: Self, value: f32) -> f32 {
        match (self, dest) {
            (Self::MgPerDeciliter, Self::MmolPerLiter) => value / 18.0156,
            (Self::MmolPerLiter, Self::MgPerDeciliter) => value * 18.0156,
            (Self::MgPerDeciliter, Self::MgPerDeciliter)
            | (Self::MmolPerLiter, Self::MmolPerLiter) => value,
        }
    }
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mg/dl" | "mg/dL" => Ok(Self::MgPerDeciliter),
            "mmol/l" | "mmol/L" => Ok(Self::MmolPerLiter),
            s => Err(format!(
                "invalid unit: `{}`; expected mg/dL, or mmol/L",
                s.escape_debug()
            )),
        }
    }
}
