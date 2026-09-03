/*!
[`Wire`]: crate::wire::Wire
[`RoundWire`]: crate::round::RoundWire
[`RectangularWire`]: crate::rectangular::RectangularWire
[`StrandedWire`]: crate::stranded::StrandedWire
[`SffWire`]: crate::sff::SffWire
[`CastWire`]: crate::cast::CastWire
[`resistance`]: crate::resistance

Composable serialization and deserialization for Rust structs.

 */
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

pub mod error;
pub mod resistance;
pub mod wire;

pub mod cast;
pub mod rectangular;
pub mod round;
pub mod sff;
pub mod stranded;
pub use stem_material;

pub mod prelude {
    /*!
    This module reexports all wire types defined in stem_wire, the
    [`Wire`] trait as well as the
    [`stem_material::prelude`](https://docs.rs/stem_material/latest/stem_material/prelude/index.html)
    module to simplify the usage of this crate.
     */

    pub use crate::cast::CastWire;
    pub use crate::rectangular::RectangularWire;
    pub use crate::round::RoundWire;
    pub use crate::sff::SffWire;
    pub use crate::stranded::{StrandedWire, WireGroup};
    pub use crate::wire::Wire;
    pub use stem_material;

    // Prevent rustdoc from documenting the stem_material dependency
    #[doc(hidden)]
    pub use stem_material::prelude::*;
}
