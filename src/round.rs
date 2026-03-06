/*!
This module contains the predefined [`Wire`] type [`RoundWire`].
 */

use std::sync::Arc;

use compare_variables::compare_variables;
use std::f64::consts::PI;
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

#[cfg(feature = "serde")]
use stem_material::prelude::serialize_quantity;

use super::wire::Wire;

/**
A flexible, round bar made of a conducting material, usually a metal such as
copper, possibly with an insulation layer.

This is the "classic" round wire used in the vast majority of electrical
machines. It is defined by the following fields:
- `outer_diameter`: Outer diameter of the conductor. Must be positive.
- `inner_diameter`: Inner diameter of the conductor. Must be positive or zero,
but smaller than `outer_diameter` (positive means the conductor is hollow, e.g.
to form a channel for cooling fluids).
- `insulation_thickness`: Thickness of the insulation layer wrapped around the
outer diameter. Must be positive or zero (zero means no insulation layer).
- `conductor_material`: The material of the conductor.

The effective conductor area is the space between the inner and the outer
diameter:

```
use std::sync::Arc;
use std::f64::consts::PI;

use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;

let wire_round = RoundWire::new(
    Arc::new(Material::default()),
    Length::new::<millimeter>(2.0), // outer_diameter
    Length::new::<millimeter>(1.0), // inner_diameter
    Length::new::<millimeter>(0.1), // insulation_thickness
).expect("valid inputs");

assert_abs_diff_eq!(
    wire_round.effective_conductor_area(Area::new::<square_millimeter>(20.0), 3).get::<square_millimeter>(),
    PI - 0.25*PI,
    epsilon = 1e-3
);
```

# Deserialization

All length fields accept SI units during deserialization (e.g. `8 mm`, `0.5 m`).
See [crate-level](crate) and [dyn_quantity](dyn_quantity) documentation for
details.
 */
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct RoundWire {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link"))]
    conductor_material: Arc<Material>,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    outer_diameter: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    inner_diameter: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    insulation_thickness: Length,
}

impl RoundWire {
    /**
    Returns a new instance of [`RoundWire`] if the given field values fulfill
    the following conditions:
    - `outer_diameter` must be positive.
    - `inner_diameter` must be positive or zero.
    - `insulation_thickness` must be positive or zero.

    See the struct docstring [`RoundWire`] for more.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).is_ok());

    // Outer diameter negative
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(-2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Outer diameter zero
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Inner diameter larger than outer diameter
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(2.2),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Inner diameter equals outer diameter
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Inner diameter negative
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(-1.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Insulation negative
    assert!(RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(-0.1)
    ).is_err());
    ```
     */
    pub fn new(
        conductor_material: Arc<Material>,
        outer_diameter: Length,
        inner_diameter: Length,
        insulation_thickness: Length,
    ) -> Result<Self, crate::error::Error> {
        return RoundWire {
            conductor_material,
            outer_diameter,
            inner_diameter,
            insulation_thickness,
        }
        .check();
    }

    /// Checks if the values of `self` are within their valid ranges.
    fn check(self) -> Result<Self, crate::error::Error> {
        let zero_length = Length::new::<meter>(0.0);
        compare_variables!(self.inner_diameter < self.outer_diameter)?;
        compare_variables!(zero_length <= self.inner_diameter)?;
        compare_variables!(zero_length <= self.insulation_thickness)?;
        return Ok(self);
    }

    /**
    Returns the "insulation" diameter of the wire (outer diameter plus two times
    insulation layer)

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    let wire = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_eq!(wire.insulation_diameter().get::<millimeter>(), 2.2);
    ```
     */
    pub fn insulation_diameter(&self) -> Length {
        return self.outer_diameter + 2.0 * self.insulation_thickness;
    }

    /// Returns the outer diameter of the conductor.
    pub fn outer_diameter(&self) -> Length {
        return self.outer_diameter;
    }

    /// Returns the inner diameter of the conductor.
    pub fn inner_diameter(&self) -> Length {
        return self.inner_diameter;
    }

    /// Returns the thickness of the insulation layer.
    pub fn insulation_thickness(&self) -> Length {
        return self.insulation_thickness;
    }

    /**
    Returns the conductor area of the wire.

    This function returns the same value as [`Wire::effective_conductor_area`],
    but does not require `zone_area` and `turns` due to all needed information
    being stored within the [`RoundWire`] struct.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    let wire = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_eq!(
        wire.conductor_area(),
        wire.effective_conductor_area(Default::default(), 0)
    );
    ```
     */
    pub fn conductor_area(&self) -> Area {
        use stem_material::uom::typenum::P2;
        return (self.outer_diameter.powi(P2::new()) - self.inner_diameter.powi(P2::new())) * PI
            / 4.0;
    }

    /**
    Returns the overall area of the wire.

    This function returns the same value as [`Wire::effective_overall_area`],
    but does not require `zone_area` and `turns` due to all needed information
    being stored within the [`RoundWire`] struct.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    let wire = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_eq!(
        wire.overall_area(),
        wire.effective_overall_area(Default::default(), 0)
    );
    ```
     */
    pub fn overall_area(&self) -> Area {
        use stem_material::uom::typenum::P2;
        return (self.outer_diameter + 2.0 * self.insulation_thickness).powi(P2::new()) * PI / 4.0;
    }
}

impl Default for RoundWire {
    fn default() -> Self {
        Self {
            conductor_material: Default::default(),
            outer_diameter: Length::new::<meter>(1.0),
            inner_diameter: Default::default(),
            insulation_thickness: Default::default(),
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wire for RoundWire {
    fn material(&self) -> &Material {
        return &*self.conductor_material;
    }

    fn material_arc(&self) -> Arc<Material> {
        return self.conductor_material.clone();
    }

    fn effective_conductor_area(&self, _zone_area: Area, _turns: usize) -> Area {
        return self.conductor_area();
    }

    fn effective_overall_area(&self, _zone_area: Area, _turns: usize) -> Area {
        return self.overall_area();
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for RoundWire {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_mosaic::deserialize_arc_link;
        use stem_material::prelude::deserialize_quantity;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RoundWireSerde {
            #[serde(deserialize_with = "deserialize_arc_link")]
            conductor_material: Arc<Material>,
            #[serde(deserialize_with = "deserialize_quantity")]
            outer_diameter: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            inner_diameter: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            insulation_thickness: Length,
        }

        let wire_serde = RoundWireSerde::deserialize(deserializer)?;
        return RoundWire {
            conductor_material: wire_serde.conductor_material,
            outer_diameter: wire_serde.outer_diameter,
            inner_diameter: wire_serde.inner_diameter,
            insulation_thickness: wire_serde.insulation_thickness,
        }
        .check()
        .map_err(serde::de::Error::custom);
    }
}
