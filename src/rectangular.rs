/*!
This module contains the predefined [`Wire`] type [`RectangularWire`].
 */

use std::sync::Arc;

use compare_variables::compare_variables;

use stem_material::prelude::*;

#[cfg(feature = "serde")]
use stem_material::prelude::serialize_quantity;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

use super::wire::Wire;

/**
A rectangular bar made of a conducting material, usually a metal such as
copper, possibly with an insulation layer.

This type of wire is often found in "constructed" windings such as hairpin
windings with a low number of turns per coil. It is defined by the following
fields:
- `height`: Height of the bar. Must be positive.
- `width`: Width of the bar. Must be positive.
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
    Length::new::<millimeter>(2.0), // height
    Length::new::<millimeter>(1.0), // width
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
See [crate-level](crate) and [dyn_quantity] documentation for details.
 */
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct RectangularWire {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link",))]
    conductor_material: Arc<Material>,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    height: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    width: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    insulation_thickness: Length,
}

impl RectangularWire {
    /**
    Returns a new instance of [`RectangularWire`] if the given field values
    fulfill the following conditions:
    - `height` must be positive.
    - `width` must be positive.
    - `insulation_thickness` must be positive or zero.

    See the struct docstring [`RectangularWire`] for more.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    assert!(RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).is_ok());

    // Height is not positive
    assert!(RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Width is not positive
    assert!(RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(-1.0),
        Length::new::<millimeter>(0.1)
    ).is_err());

    // Insulation thickness is negative
    assert!(RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(-0.1)
    ).is_err());
    ```
     */
    pub fn new(
        conductor_material: Arc<Material>,
        height: Length,
        width: Length,
        insulation_thickness: Length,
    ) -> Result<Self, crate::error::Error> {
        return RectangularWire {
            conductor_material,
            height,
            width,
            insulation_thickness,
        }
        .check();
    }

    /// Checks if the values of `self` are within their valid ranges.
    fn check(self) -> Result<Self, crate::error::Error> {
        let zero_length = Length::new::<meter>(0.0);
        compare_variables!(zero_length < self.height)?;
        compare_variables!(zero_length < self.width)?;
        compare_variables!(zero_length <= self.insulation_thickness)?;
        return Ok(self);
    }

    /// Returns the thickness of the insulation layer.
    pub fn insulation_thickness(&self) -> Length {
        return self.insulation_thickness;
    }

    /**
    Returns the conductor area of the wire.

    This function returns the same value as [`Wire::effective_conductor_area`],
    but does not require `zone_area` and `turns` due to all needed information
    being stored within the [`RectangularWire`] struct.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    let wire = RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_eq!(
        wire.conductor_area(),
        wire.effective_conductor_area(Default::default(), 0)
    );
    ```
     */
    pub fn conductor_area(&self) -> Area {
        return self.height * self.width;
    }

    /**
    Returns the overall area of the wire.

    This function returns the same value as [`Wire::effective_overall_area`],
    but does not require `zone_area` and `turns` due to all needed information
    being stored within the [`RectangularWire`] struct.

    # Examples

    ```
    use std::sync::Arc;
    use stem_wire::prelude::*;

    let wire = RectangularWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_eq!(
        wire.overall_area(),
        wire.effective_overall_area(Default::default(), 0)
    );
    ```
     */
    pub fn overall_area(&self) -> Area {
        return (self.height + 2.0 * self.insulation_thickness)
            * (self.width + 2.0 * self.insulation_thickness);
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wire for RectangularWire {
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
impl<'de> Deserialize<'de> for RectangularWire {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_mosaic::deserialize_arc_link;
        use stem_material::prelude::deserialize_quantity;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RectangularWireSerde {
            #[serde(deserialize_with = "deserialize_arc_link")]
            conductor_material: Arc<Material>,
            #[serde(deserialize_with = "deserialize_quantity")]
            height: Length, // Height of the wire (when inserted in the slot)
            #[serde(deserialize_with = "deserialize_quantity")]
            width: Length, // Width of the wire (when inserted in the slot)
            #[serde(deserialize_with = "deserialize_quantity")]
            insulation_thickness: Length, // Thickness of the insulation layer
        }

        let wire_serde = RectangularWireSerde::deserialize(deserializer)?;
        return RectangularWire {
            conductor_material: wire_serde.conductor_material,
            height: wire_serde.height,
            width: wire_serde.width,
            insulation_thickness: wire_serde.insulation_thickness,
        }
        .check()
        .map_err(serde::de::Error::custom);
    }
}
