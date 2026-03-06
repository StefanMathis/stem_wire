/*!
Provides analytical formulas for computing the electrical resistance of
homogeneous conductors with idealized geometries, including rectangular
prisms (cuboids), cylindrical segments (axial, radial, and tangential),
and hollow spheres.

All functions assume:

- Steady-state conduction (no displacement current),
- Homogeneous and isotropic material properties,
- Idealized geometric shapes,
- Current distributions consistent with the analytical solutions.

The parameter `fraction`, where present, represents the geometric fraction
of the full body (e.g. `1.0` = full geometry, `0.5` = half).

These utilities are primarily intended for use in
[`Wire::resistance`](crate::Wire::resistance).
*/

use std::f64::consts::PI;
use std::f64::consts::TAU;
use stem_material::uom::si::f64::*;

/**
Calculates the electrical resistance of a homogeneous conductor
with uniform cross-sectional area.

Implemented relation:

`R = L / (σ · A)`

where:

- `L` is the conductor length,
- `A` is the cross-sectional area,
- `σ` is the electrical conductivity.

Division by zero (e.g. zero area or conductivity) results in infinite resistance.

# Examples

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::conductor_area;

let conductivity = ElectricalConductivity::new::<siemens_per_meter>(6e7);
let length = Length::new::<meter>(6.0);
let area = Area::new::<square_meter>(1e-6);

let r = conductor_area(conductivity, length, area);
assert_abs_diff_eq!(r.get::<ohm>(), 0.1, epsilon=1e-3);
```
*/
pub fn conductor_area(
    conductivity: ElectricalConductivity,
    length: Length,
    area: Area,
) -> ElectricalResistance {
    length / (area * conductivity)
}

/**
Calculates the electrical resistance of a homogeneous rectangular
prism (cuboid) traversed by a current flowing between two opposing faces.

The current flows along `length`, while `width` and `height`
define the rectangular cross-sectional area.

Implemented relation:

R = L / (σ · width · height)

Division by zero results in infinite resistance.

# Examples

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::quader;

let result = quader(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
);
assert_eq!(result.get::<ohm>(), 2.0);

// Zero cross-sectional dimension
let result = quader(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(0.0),
);
assert!(result.is_infinite());
```
*/
pub fn quader(
    conductivity: ElectricalConductivity,
    length: Length,
    width: Length,
    height: Length,
) -> ElectricalResistance {
    length / (width * height * conductivity)
}

/**
Calculates the electrical resistance of a hollow cylindrical segment
with current flowing axially between its circular end faces.

Cross-sectional area:

A = π (r_outer² − r_inner²)

Implemented relation:

R = L / (σ · A · fraction)

Requires `r_outer ≥ r_inner`.
Division by zero results in infinite resistance.

# Examples

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::cylinder_axial;

// Full cylinder
let result = cylinder_axial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(0.0),
    1.0
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.318309, epsilon = 1e-6);

// Half cylinder
let result = cylinder_axial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(0.0),
    0.5
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.636619, epsilon = 1e-6);
```
*/
pub fn cylinder_axial(
    conductivity: ElectricalConductivity,
    length: Length,
    outer_radius: Length,
    inner_radius: Length,
    fraction: f64,
) -> ElectricalResistance {
    use stem_material::uom::typenum::P2;

    let area = PI * (outer_radius.powi(P2::new()) - inner_radius.powi(P2::new()));

    length / (area * conductivity * fraction)
}

/**
Calculates the electrical resistance of a hollow cylindrical segment
with current flowing radially from the inner radius to the outer radius.

Implemented relation:

R = ln(r_outer / r_inner) / (2π L σ · fraction)

Requires `r_outer > r_inner`.
Division by zero results in infinite resistance.

# Examples

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::cylinder_radial;

// Full cylinder
let result = cylinder_radial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    1.0
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.0011031, epsilon = 1e-6);

// Half cylinder
let result = cylinder_radial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    0.5
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.0022064, epsilon = 1e-6);
```
*/
pub fn cylinder_radial(
    conductivity: ElectricalConductivity,
    length: Length,
    outer_radius: Length,
    inner_radius: Length,
    fraction: f64,
) -> ElectricalResistance {
    (outer_radius / inner_radius).ln() / (TAU * length * conductivity * fraction)
}

/**
Calculates the electrical resistance of a hollow cylindrical segment
with current flowing tangentially (circumferential direction).

Implemented relation:

R = (2π · fraction) / (σ · L · ln(r_outer / r_inner))

Requires `r_outer > r_inner`.
Division by zero results in infinite resistance.

# Examples

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::cylinder_tangential;

// Full cylinder
let result = cylinder_tangential(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    1.0
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.0906472, epsilon = 1e-6);

// Half cylinder
let result = cylinder_tangential(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(20.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    0.5
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.0906472/2.0, epsilon = 1e-6);
```
*/
pub fn cylinder_tangential(
    conductivity: ElectricalConductivity,
    length: Length,
    outer_radius: Length,
    inner_radius: Length,
    fraction: f64,
) -> ElectricalResistance {
    TAU * fraction / (length * conductivity * (outer_radius / inner_radius).ln())
}

/**
Calculates the electrical resistance of a hollow spherical shell
with current flowing radially from the inner sphere to the outer sphere.

Implemented relation:

R = (1 / (4π σ · fraction)) · (1/r_inner − 1/r_outer)

Requires `r_outer > r_inner`.
Division by zero results in infinite resistance.

```
use approx::assert_abs_diff_eq;

use stem_wire::prelude::*;
use stem_wire::resistance::sphere_radial;

// Full sphere
let result = sphere_radial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    1.0
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 0.0079577, epsilon = 1e-6);

// Half sphere
let result = sphere_radial(
    ElectricalConductivity::new::<siemens_per_meter>(5.0),
    Length::new::<meter>(2.0),
    Length::new::<meter>(1.0),
    0.5
);
approx::assert_abs_diff_eq!(result.get::<ohm>(), 2.0*0.0079577, epsilon = 1e-6);
```
*/
pub fn sphere_radial(
    conductivity: ElectricalConductivity,
    outer_radius: Length,
    inner_radius: Length,
    fraction: f64,
) -> ElectricalResistance {
    (f64::from(outer_radius / inner_radius) - 1.0)
        / (4.0 * PI * conductivity * outer_radius * fraction)
}
