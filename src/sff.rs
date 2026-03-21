/*!
This module contains the predefined [`Wire`] type [`SffWire`].
 */

use std::sync::Arc;

use compare_variables::compare_variables;
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

use super::wire::Wire;

/**
An "abstract" wire defined by its material and its slot fill factor(s).

This is a very simple wire type which is fully specified by the following three
properties:
- `slot_fill_factor_conductor`: Portion of the winding zone / slot filled with
conducting material. Must be between 0 and 1
(`0 <= slot_fill_factor_conductor <= 1`).
- `slot_fill_factor_overall`: Portion of the winding zone / slot filled with
wire material (conductor + insulation). Must be between 0 and 1 and must be
larger than or equal to `slot_fill_factor_conductor`
(`slot_fill_factor_conductor <= slot_fill_factor_overall <= 1`).
- `conductor_material`: The material of the conductor.

This wire type can be useful when modeling a winding where only its slot fill
factor is known and the exact geometry (single conductor? multiple WireGroupd
stranded conductor?) is undefined. This also means that the area of the wire
depends on the space available to it:

```
use std::sync::Arc;
use std::f64::consts::PI;

use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;

let wire_round = SffWire::new(
    Arc::new(Material::default()),
    0.5, // slot_fill_factor_conductor
    0.6, // slot_fill_factor_overall
).expect("valid inputs");

// 10 wires in an area of 20 mm² -> Each wire has a space of 2 mm² available for
// it. Since the conductor slot fill factor is 0.5, the effective conductor area
// is 1. Similarily, the total wire area is 1.2
assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(20.0), 10).get::<square_millimeter>(),
    1.0,
    epsilon = 1e-3
);
assert_abs_diff_eq!(
    wire_round.effective_overall_area(Area::new::<square_millimeter>(20.0), 10).get::<square_millimeter>(),
    1.2,
    epsilon = 1e-3
);

// Now the area is increased -> Area per wire increases proportionally:
assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(40.0), 10).get::<square_millimeter>(),
    2.0,
    epsilon = 1e-3
);
assert_abs_diff_eq!(
    wire_round.effective_overall_area(Area::new::<square_millimeter>(40.0), 10).get::<square_millimeter>(),
    2.4,
    epsilon = 1e-3
);

// More wires in the same area means less area per wire:
assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(20.0), 20).get::<square_millimeter>(),
    0.5,
    epsilon = 1e-3
);
assert_abs_diff_eq!(
    wire_round.effective_overall_area(Area::new::<square_millimeter>(20.0), 20).get::<square_millimeter>(),
    0.6,
    epsilon = 1e-3
);
```
 */
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct SffWire {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link",))]
    conductor_material: Arc<Material>,
    slot_fill_factor_conductor: f64,
    slot_fill_factor_overall: f64,
}

impl SffWire {
    /**
    Returns a new instance of [`SffWire`] if the given field values fulfill
    the following conditions:
    - `slot_fill_factor_conductor` must be between 0 and 1
    (`0 <= slot_fill_factor_overall <= 1`).
    - `slot_fill_factor_overall` must be between 0 and 1 and equal to or larger
    than `slot_fill_factor_conductor`
    (`slot_fill_factor_conductor <= slot_fill_factor_overall <= 1`).

    See the struct docstring [`SffWire`] for more.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    assert!(SffWire::new(Arc::new(Material::default()), 0.5, 0.6).is_ok());

    // Conductor slot fill factor negative
    assert!(SffWire::new(Arc::new(Material::default()), -0.2, 0.6).is_err());

    // Conductor slot fill factor larger than 1
    assert!(SffWire::new(Arc::new(Material::default()), 1.1, 0.6).is_err());

    // Overall slot fill factor larger than 1
    assert!(SffWire::new(Arc::new(Material::default()), 0.5, 1.1).is_err());

    // Overall slot fill factor smaller than conductor slot fill factor
    assert!(SffWire::new(Arc::new(Material::default()), 0.5, 0.4).is_err());
    ```
     */
    pub fn new(
        conductor_material: Arc<Material>,
        slot_fill_factor_conductor: f64,
        slot_fill_factor_overall: f64,
    ) -> Result<Self, crate::error::Error> {
        return SffWire {
            conductor_material,
            slot_fill_factor_conductor,
            slot_fill_factor_overall,
        }
        .check();
    }

    /// Checks if the values of `self` are within their valid ranges.
    fn check(self) -> Result<Self, crate::error::Error> {
        compare_variables!(0.0 <= self.slot_fill_factor_conductor <= 1.0)?;
        compare_variables!(0.0 <= self.slot_fill_factor_overall <= 1.0)?;
        compare_variables!(self.slot_fill_factor_conductor <= self.slot_fill_factor_overall)?;
        return Ok(self);
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wire for SffWire {
    fn slot_fill_factor_conductor(&self, _zone_area: Area, _turns: usize) -> f64 {
        return self.slot_fill_factor_conductor;
    }

    fn slot_fill_factor_overall(&self, _zone_area: Area, _turns: usize) -> f64 {
        return self.slot_fill_factor_overall;
    }

    fn material(&self) -> &Material {
        return &*self.conductor_material;
    }

    fn material_arc(&self) -> Arc<Material> {
        return self.conductor_material.clone();
    }

    fn effective_conductor_area(&self, zone_area: Area, turns: usize) -> Area {
        return self.slot_fill_factor_conductor(zone_area, turns) * zone_area / turns as f64;
    }

    fn effective_overall_area(&self, zone_area: Area, turns: usize) -> Area {
        return self.slot_fill_factor_overall(zone_area, turns) * zone_area / turns as f64;
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SffWire {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_mosaic::deserialize_arc_link;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct SffWireSerde {
            #[serde(deserialize_with = "deserialize_arc_link")]
            conductor_material: Arc<Material>,
            slot_fill_factor_conductor: f64,
            slot_fill_factor_overall: f64,
        }

        let wire_serde = SffWireSerde::deserialize(deserializer)?;
        return SffWire {
            conductor_material: wire_serde.conductor_material,
            slot_fill_factor_conductor: wire_serde.slot_fill_factor_conductor,
            slot_fill_factor_overall: wire_serde.slot_fill_factor_overall,
        }
        .check()
        .map_err(serde::de::Error::custom);
    }
}
