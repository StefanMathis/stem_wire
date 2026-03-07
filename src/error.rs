/*!
This module contains the [`Error`] enum, which represents the different ways
building one of the predefined wires can fail due to invalid input data. The
[`Error::Other`] variants supports arbitrary errors resulting from user-created
wire types.
*/

use compare_variables::ComparisonError;
use stem_material::uom::si::f64::Length;

/**
An enum representing errors returned by wire constructors.
*/
#[derive(Debug)]
pub enum Error {
    /**
    A given physical [`Length`] is not within its allowed value range (as
    specified inside the [`ComparisonError`], usually a length needs to be
    positive).
     */
    InvalidLength(ComparisonError<Length>),
    /**
    A given [`f64`] value is not within its allowed value range (as specified
    inside the [`ComparisonError`]).
     */
    InvalidF64(ComparisonError<f64>),
    /**
    A given [`usize`] value is not within its allowed value range (as
    specified inside the [`ComparisonError`]).
     */
    InvalidUsize(ComparisonError<usize>),
    /**
    This error is returned when building a
    [`StrandedWire`](crate::stranded::StrandedWire) out of wires with
    inequal materials; [`StrandedWire`](crate::stranded::StrandedWire)
    requires that all of its component wires use the same material.
     */
    InequalMaterials,
    /**
    The strand list / vector provided to
    [`StrandedWire::new`](crate::stranded::StrandedWire::new) was
    empty.
     */
    EmptyStrandList,
    /**
    Fallback variant for arbitrary other errors (e.g. from custom wire
    implementations).
     */
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength(comparison_error) => comparison_error.fmt(f),
            Error::InvalidF64(comparison_error) => comparison_error.fmt(f),
            Error::InvalidUsize(comparison_error) => comparison_error.fmt(f),
            Error::InequalMaterials => write!(f, "all strand materials must be equal"),
            Error::EmptyStrandList => write!(f, "strand list must not be empty"),
            Error::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<ComparisonError<Length>> for Error {
    fn from(value: ComparisonError<Length>) -> Self {
        return Error::InvalidLength(value);
    }
}

impl From<ComparisonError<f64>> for Error {
    fn from(value: ComparisonError<f64>) -> Self {
        return Error::InvalidF64(value);
    }
}

impl From<ComparisonError<usize>> for Error {
    fn from(value: ComparisonError<usize>) -> Self {
        return Error::InvalidUsize(value);
    }
}
