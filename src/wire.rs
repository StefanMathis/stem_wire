/*!
The [`Wire`] trait for defining wires in stem.

A winding of an electrical machines creates a purposefully designed magnetic
field when current passes through it. It is made up of individual wires
(electric conductors) which are arranged in a certain geometric configuration
which shapes the magnetic field. In stem, any type can be used as the wire of
a winding if it implements the [`Wire`] trait. See its docstring for more.
 */

use dyn_clone::DynClone;
use std::any::Any;
use std::sync::Arc;
use stem_material::prelude::*;

/**
A trait for defining wires for usage in windings.

In stem, a "wire" is a conductor for electric currents with two terminals. This
encompasses both the traditional definition of a wire as a flexible, round bar
of metal and also other conductor variants such as e.g. the massive, rigid bars
found in the cage of asynchronous motors. A winding in stem always consists of
a single wire (implementor of this trait) which provides the calculation
routines for properties such as the resistance, slot filling factor or cross
section. A conductor consisting of multiple individual physical wires can be
represented by abstractions such as [`StrandedWire`](crate::StrandedWire)).

Depending on the wire type, the geometric dimensions of the wire might
either be a property of the wire itself (see e.g.
[`RoundWire`](crate::RoundWire)) or depend on the geometry of the magnetic core
which holds the corresponding winding. For this reason, the methods of this
trait often require additional information. For example,
[`Wire::conductor_area`] needs both the area covered by the winding zone and
the number of turns within the zone.

Unless explicitly mentioned otherwise, the following assumptions are made for
all wires:
- Steady-state conduction (no displacement current)
- Homogeneous and isotropic material properties
- Idealized geometric shapes

This crate provides multiple predefined wire types which implement this trait:
- [`CastWire`](crate::CastWire): A "bar" wire used in cage windings.
- [`RectangularWire`](crate::RectangularWire): A rectangular bar e.g. for hair
pin windings.
- [`RoundWire`](crate::RoundWire): A round wire as used in the vast majority of
electrical machines.
- [`SffWire`](crate::SffWire): An abstract wire described by its slot filling
factor where the exact geometry of the conductor is not defined.
- [`StrandedWire`](crate::StrandedWire): A WireGroupd wire formed from multiple
individual conductors (e.g. [`RoundWire`](crate::RoundWire)) which are connected
in parallel.

By implementing this trait for a custom type, user-defined wires can be used in
stem in the same way as those predefined types.
 */
#[cfg_attr(feature = "serde", typetag::serde)]
pub trait Wire: Sync + Send + DynClone + std::fmt::Debug + Any {
    /**
    Returns a shared reference to the conductor material of the wire.
     */
    fn material(&self) -> &Material;

    /**
    Returns the current-carrying cross section of the wire.

    Depending on the wire type, it might be necessary to provide the area
    convered by the winding (`zone_area`) and the number of `turns` in that zone.
    An example is the [`SffWire`](crate::SffWire) type:
    It is an abstract wire defined by just the slot filling factor; its cross
    section is hence calculated as `slot_filling_factor * zone_area / turns`.
    On the other hand, the cross section of a [`RoundWire`](crate::RoundWire)
    is just `pi * radius²` and both `zone_area` and `turns` are not used at all.

    # Examples

    ```
    use std::sync::Arc;
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use stem_wire::prelude::*;

    let m = Arc::new(Material::default());

    // SffWire: Cross section depends on zone area
    let wire_sff = SffWire::new(m.clone(), 0.5, 0.6).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_sff.effective_conductor_area(Area::new::<square_millimeter>(20.0), 3).get::<square_millimeter>(),
        3.333, epsilon = 1e-3);
    assert_abs_diff_eq!(
        wire_sff.effective_conductor_area(Area::new::<square_millimeter>(200.0), 25).get::<square_millimeter>(),
        4.0, epsilon = 1e-3);

    // RoundWire: Cross section is defined by wire properties
    let wire_round = RoundWire::new(
        m,
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_round.effective_conductor_area(Area::new::<square_millimeter>(20.0), 3).get::<square_millimeter>(),
        PI, epsilon = 1e-3);
    assert_abs_diff_eq!(
        wire_round.effective_conductor_area(Area::new::<square_millimeter>(200.0), 25).get::<square_millimeter>(),
        PI, epsilon = 1e-3);
    ```
     */
    fn effective_conductor_area(&self, zone_area: Area, turns: usize) -> Area;

    /**
    Returns the overall area covered by the wire.

    As with [`Wire::effective_conductor_area`], some wire types require
    specifying the area convered by the winding (`zone_area`) and
    the number of `turns` in that zone.

    # Examples

    ```
    use std::sync::Arc;
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use stem_wire::prelude::*;

    let m = Arc::new(Material::default());

    // SffWire: Cross section depends on zone area
    let wire_sff = SffWire::new(m.clone(), 0.5, 0.6).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_sff.effective_overall_area(Area::new::<square_millimeter>(20.0), 3).get::<square_millimeter>(),
        4.0, epsilon = 1e-3);
    assert_abs_diff_eq!(
        wire_sff.effective_overall_area(Area::new::<square_millimeter>(200.0), 25).get::<square_millimeter>(),
        4.8, epsilon = 1e-3);

    // RoundWire: Cross section is defined by wire properties
    let wire_round = RoundWire::new(
        m,
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_round.effective_overall_area(Area::new::<square_millimeter>(20.0), 4).get::<square_millimeter>(),
        1.21*PI, epsilon = 1e-3);
    assert_abs_diff_eq!(
        wire_round.effective_overall_area(Area::new::<square_millimeter>(200.0), 25).get::<square_millimeter>(),
        1.21*PI, epsilon = 1e-3);
    ```
     */
    fn effective_overall_area(&self, zone_area: Area, turns: usize) -> Area;

    // =========================================================================

    /**
    Returns the conductor material as a reference-counted [`Arc`].

    The default implementation clones the underlying [`Material`]
    and wraps it in a new `Arc`.

    Implementors that internally store their material in an
    [`Arc<Material>`] may override this method to return a clone
    of that `Arc` instead, avoiding an additional allocation
    and material clone.
     */
    fn material_arc(&self) -> Arc<Material> {
        Arc::new(self.material().clone())
    }

    /**
    Returns the resistance of a wire with the given `length` under influence of
    the specified `conditions`.

    The default implementation of this method returns the resistance `R` as

    `R = L *  ρ / A`

    where:

    - `L` is the conductor `length`,
    - `A` is the cross-sectional area from [`Wire::conductor_area`],
    - `ρ` is the electrical resistivity from [`Material::electrical_resistivity`].

    `zone_area` and `turns` are forwarded to [`Wire::conductor_area`], while
    the conditions are used as the input to the [`VarQuantity::get`] method of
    the [`Material::electrical_resistivity`] field.

    # Examples

    ```
    use std::sync::Arc;
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use stem_wire::prelude::{*, unary::Linear};

    let mut mat = Material::default();
    mat.set_electrical_resistivity(
        VarQuantity::try_from_quantity_function(Linear::new(
            1.0.into(),
            DynQuantity::new(2.0, PredefUnit::ElectricResistivity),
        ))
        .unwrap(),
    );

    let wire = RoundWire::new(
        Arc::new(mat),
        Length::new::<millimeter>(1000.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(100.0),
    )
    .expect("valid inputs");

    assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<millimeter>(2000.0),
            Area::new::<square_millimeter>(0.0),
            1,
            &[ThermodynamicTemperature::new::<degree_celsius>(0.0).into()]
        )
        .get::<ohm>(),
        5.092,
        epsilon = 1e-3
    );
    assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<millimeter>(2000.0),
            Area::new::<square_millimeter>(0.0),
            1,
            &[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()]
        )
        .get::<ohm>(),
        5.092,
        epsilon = 1e-3
    );
    ```
     */
    fn resistance(
        &self,
        length: Length,
        zone_area: Area,
        turns: usize,
        conditions: &[DynQuantity<f64>],
    ) -> ElectricalResistance {
        return length * self.material().electrical_resistivity.get(conditions)
            / self.effective_conductor_area(zone_area, turns);
    }

    /**
    Returns the electrical slot filling factor.

    This is the ratio between the
    [`effective_conductor_area`](Wire::effective_conductor_area) and the total
    area available for a single turn (`zone_area / turns`). Usually, there is no
    need to overwrite this method (except if the slot filling factor is already
    known and the cross section is calculated from it, as is the case for
    [`SffWire`](crate::SffWire)).

    # Examples

    ```
    use std::sync::Arc;
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use stem_wire::prelude::*;

    let m = Arc::new(Material::default());

    // SffWire: Cross section depends on zone area
    let wire_sff = SffWire::new(m.clone(), 0.5, 0.6).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_sff.slot_fill_factor_conductor(Area::new::<square_millimeter>(200.0), 25),
        0.5, epsilon = 1e-3);

    // RoundWire: Cross section is defined by wire properties
    let wire_round = RoundWire::new(
        m,
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_round.slot_fill_factor_conductor(Area::new::<square_millimeter>(200.0), 25),
        0.393, epsilon = 1e-3);
    ```
     */
    fn slot_fill_factor_conductor(&self, zone_area: Area, turns: usize) -> f64 {
        return (self.effective_conductor_area(zone_area, turns) * turns as f64 / zone_area)
            .get::<ratio>();
    }

    /**
    Returns the mechanical slot filling factor.

    This is the ratio between the
    [`effective_overall_area`](Wire::effective_overall_area) and the total
    area available for a single turn (`zone_area / turns`). Usually, there is no
    need to overwrite this method (except if the slot filling factor is already
    known and the cross section is calculated from it, as is the case for
    [`SffWire`](crate::SffWire)).

    # Examples

    ```
    use std::sync::Arc;
    use std::f64::consts::PI;

    use approx::assert_abs_diff_eq;

    use stem_wire::prelude::*;

    let m = Arc::new(Material::default());

    // SffWire: Cross section depends on zone area
    let wire_sff = SffWire::new(m.clone(), 0.5, 0.6).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_sff.slot_fill_factor_overall(Area::new::<square_millimeter>(200.0), 25),
        0.6, epsilon = 1e-3);

    // RoundWire: Cross section is defined by wire properties
    let wire_round = RoundWire::new(
        m,
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1)
    ).expect("valid inputs");
    assert_abs_diff_eq!(
        wire_round.slot_fill_factor_overall(Area::new::<square_millimeter>(200.0), 25),
        0.475, epsilon = 1e-3);
    ```
    */
    fn slot_fill_factor_overall(&self, zone_area: Area, turns: usize) -> f64 {
        return (self.effective_overall_area(zone_area, turns) * turns as f64 / zone_area)
            .get::<ratio>();
    }
}

dyn_clone::clone_trait_object!(Wire);
