/*!
This module contains the predefined [`Wire`] type [`CastWire`].
 */

use std::sync::Arc;

use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_mosaic::{deserialize_arc_link, serialize_arc_link};

use super::wire::Wire;

/**
A cast "wire" filling an entire slot without insulation.

Casted "wires" are usually found in cage windings of asynchronous motors. They
are created by filling liquid material into the slots of magnetic cores. Since
the number of "turns" is always one for those kind of windings, the induced
voltages are very low, meaning that typically no insulation is required. Hence,
the wire fills the entire slot and is fully defined by its material. The slot
fill factor is always 100%.

# Examples

```
use std::sync::Arc;
use std::f64::consts::PI;

use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;

let wire_round = CastWire::new(Arc::new(Material::default()));

assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(20.0), 1).get::<square_millimeter>(),
    20.0,
    epsilon = 1e-3
);
assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(50.0), 1).get::<square_millimeter>(),
    50.0,
    epsilon = 1e-3
);

// Giving a number of turns greater than 1 does not make any sense, but the
// function will return a sensible result anyway.
assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(50.0), 2).get::<square_millimeter>(),
    50.0,
    epsilon = 1e-3
);

assert_eq!(wire_round.slot_fill_factor_conductor(Area::new::<square_millimeter>(20.0), 1), 1.0);
assert_eq!(wire_round.slot_fill_factor_overall(Area::new::<square_millimeter>(20.0), 1), 1.0);
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct CastWire(
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_arc_link",
            deserialize_with = "deserialize_arc_link"
        )
    )]
    Arc<Material>,
);

impl CastWire {
    /// Returns a new instance of `Self`
    pub fn new(material: Arc<Material>) -> Self {
        return CastWire(material);
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wire for CastWire {
    fn material(&self) -> &Material {
        return &*self.0;
    }

    fn material_arc(&self) -> Arc<Material> {
        return self.0.clone();
    }

    fn effective_conductor_area(&self, zone_area: Area, _turns_per_slot: usize) -> Area {
        return zone_area;
    }

    fn effective_overall_area(&self, zone_area: Area, _turns_per_slot: usize) -> Area {
        return zone_area;
    }

    fn slot_fill_factor_conductor(&self, _zone_area: Area, _turns: usize) -> f64 {
        return 1.0;
    }

    fn slot_fill_factor_overall(&self, _zone_area: Area, _turns: usize) -> f64 {
        return 1.0;
    }
}
