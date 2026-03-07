#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod error;
pub mod resistance;
pub mod wire;

pub mod cast;
pub mod rectangular;
pub mod round;
pub mod sff;
pub mod stranded;

pub mod prelude {
    /*!
    This module reexports all wire types defined in stem_wire, the
    [`Wire`] trait as well as the [`stem_material::prelude`]
    module to simplify the usage of this crate.
     */

    pub use crate::cast::CastWire;
    pub use crate::rectangular::RectangularWire;
    pub use crate::round::RoundWire;
    pub use crate::sff::SffWire;
    pub use crate::stranded::{StrandedWire, WireGroup};
    pub use crate::wire::Wire;
    pub use stem_material;

    pub use stem_material::prelude::*;
}
