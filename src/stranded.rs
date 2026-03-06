/*!
This module contains the predefined [`Wire`] type [`StrandedWire`] formed from
multiple [`WireGroup`]s.
 */

use std::num::NonZeroUsize;
use std::sync::Arc;

use rayon::prelude::*;
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::wire::Wire;
use crate::error::Error;

/**
A group of identical wires within a [`StrandedWire`].

`WireGroup` represents multiple occurrences of the same [`Wire`] type. Instead
of storing identical wire instances repeatedly, a single [`Wire`] type is stored
together with the number of times it appears in the stranded conductor.

This is useful for modeling conductors that consist of severals strands of the
same wire:

# Example

A stranded wire consisting of three wires of type `A` and two wires
of type `B` can be represented as:

- `WireGroup { wire: A, number_wires: 3 }`
- `WireGroup { wire: B, number_wires: 2 }`

These groups together form a [`StrandedWire`]:

```
use std::sync::Arc;
use std::num::NonZero;

use stem_wire::prelude::*;

let wire_a = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.5),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).unwrap();
let wire_b = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.4),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).unwrap();

// Returns OK because the vector of wires was not empty
assert!(StrandedWire::new(vec![
    WireGroup::new(Box::new(wire_a), NonZero::new(3).unwrap()),
    WireGroup::new(Box::new(wire_b), NonZero::new(2).unwrap()),
]).is_ok());
```
*/
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WireGroup {
    /// The wire used in the group.
    pub wire: Box<dyn Wire>,
    /// Number of times [`WireGroup::wire`] appears in the group.
    pub number_wires: NonZeroUsize,
}

impl WireGroup {
    /// Returns a new wire group from the wire type and the number of wires.
    pub fn new(wire: Box<dyn Wire>, number_wires: NonZeroUsize) -> Self {
        return Self { wire, number_wires };
    }
}

impl Clone for WireGroup {
    fn clone(&self) -> Self {
        Self {
            wire: dyn_clone::clone_box(&*self.wire),
            number_wires: self.number_wires.clone(),
        }
    }
}

/**
A stranded wire consistings of multiple [`WireGroup`]s.

A stranded wire is composed of a number of small wires bundled or wrapped
together to form a larger conductor. Compared to a single solid wire, this
configuration is more mechanically flexible and allows the usage of a few
standardized wires to realize various different cross-sections.

To simplify the implementation, each wire within the given [`WireGroup`]s must
have the same material. In practice, this is not a huge issue, as conductors
of different materials are usually not used together in a stranded wire.

# Example

A stranded wire consisting of three wires of type `A` and two wires
of type `B` can be represented as:

- `WireGroup { wire: A, number_wires: 3 }`
- `WireGroup { wire: B, number_wires: 2 }`

These groups together form a [`StrandedWire`]:

```
use std::sync::Arc;
use std::num::NonZero;

use stem_wire::prelude::*;

let wire_a = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.5),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).unwrap();
let wire_b = RoundWire::new(
        Arc::new(Material::default()),
        Length::new::<millimeter>(0.4),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).unwrap();

// Returns OK because the vector of wires was not empty
assert!(StrandedWire::new(vec![
    WireGroup::new(Box::new(wire_a), NonZero::new(3).unwrap()),
    WireGroup::new(Box::new(wire_b), NonZero::new(2).unwrap()),
]).is_ok());
```
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct StrandedWire(Vec<WireGroup>);

impl StrandedWire {
    /**
    Returns a new stranded wire if the given vector of [`WireGroup`] fulfills
    the following conditions:
    1) It is not empty.
    2) All wires must have the same material.

    # Examples

    ```
    use std::sync::Arc;
    use std::num::NonZero;

    use stem_wire::prelude::*;

    let wire_a = RoundWire::new(
            Arc::new(Material::default()),
            Length::new::<millimeter>(0.5),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1)
        ).unwrap();
    let wire_b = RoundWire::new(
            Arc::new(Material::default()),
            Length::new::<millimeter>(0.4),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1)
        ).unwrap();
    let mut material = Material::default();

    material.electrical_resistivity = ElectricalResistivity::new::<ohm_meter>(0.1).into();
    let wire_c = RoundWire::new(
            Arc::new(material),
            Length::new::<millimeter>(0.4),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1)
        ).unwrap();

    assert!(StrandedWire::new(vec![
        WireGroup::new(Box::new(wire_a.clone()), NonZero::new(3).unwrap()),
        WireGroup::new(Box::new(wire_b), NonZero::new(2).unwrap()),
    ]).is_ok());

    // Empty list
    assert!(StrandedWire::new(Vec::new()).is_err());

    // Not all wires have the same material
    assert!(StrandedWire::new(vec![
        WireGroup::new(Box::new(wire_a), NonZero::new(3).unwrap()),
        WireGroup::new(Box::new(wire_c), NonZero::new(2).unwrap()),
    ]).is_err());
    ```
     */
    pub fn new(wire_groups: Vec<WireGroup>) -> Result<Self, Error> {
        return StrandedWire(wire_groups).check();
    }

    /// Check if self is properly defined (no illegal parameter values)
    fn check(self) -> Result<Self, Error> {
        if let Some(first_wire) = self.0.first() {
            for wire_group in self.0.iter().skip(1) {
                if first_wire.wire.material() != wire_group.wire.material() {
                    return Err(Error::InequalMaterials);
                }
            }
        } else {
            return Err(Error::EmptyStrandList);
        }
        return Ok(self);
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Wire for StrandedWire {
    fn material(&self) -> &Material {
        // SAFETY: Constructor makes sure there is at least one element in the
        // vector.
        return unsafe { self.0.get_unchecked(0) }.wire.material();
    }

    fn material_arc(&self) -> Arc<Material> {
        // SAFETY: Constructor makes sure there is at least one element in the
        // vector.
        return unsafe { self.0.get_unchecked(0) }.wire.material_arc();
    }

    fn effective_conductor_area(&self, zone_area: Area, turns: usize) -> Area {
        return self
            .0
            .as_slice()
            .par_iter()
            .map(|strand| {
                return strand.wire.effective_conductor_area(zone_area, turns)
                    * (usize::from(strand.number_wires) as f64);
            })
            .sum();
    }

    fn effective_overall_area(&self, zone_area: Area, turns: usize) -> Area {
        return self
            .0
            .as_slice()
            .par_iter()
            .map(|strand| {
                return strand.wire.effective_overall_area(zone_area, turns)
                    * (usize::from(strand.number_wires) as f64);
            })
            .sum();
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for StrandedWire {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StrandedWireSerde(Vec<WireGroup>);

        let wire_serde = StrandedWireSerde::deserialize(deserializer)?;
        return StrandedWire(wire_serde.0)
            .check()
            .map_err(serde::de::Error::custom);
    }
}
