stem_wire
=========

[`Wire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/wire/trait.Wire.html
[`RoundWire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/round/struct.RoundWire.html
[`RectangularWire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/rectangular/struct.RectangularWire.html
[`StrandedWire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/stranded/struct.StrandedWire.html
[`SffWire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/sff/struct.SffWire.html
[`CastWire`]: https://docs.rs/stem_wire/0.1.2/stem_wire/cast/struct.CastWire.html
[`resistance`]: https://docs.rs/stem_wire/0.1.2/stem_wire/resistance/

[![Documentation](https://docs.rs/stem_wire/badge.svg)](https://docs.rs/stem_wire)

Wire definition for stem - a Simulation Toolbox for Electric Motors.

The full API documentation is available at https://docs.rs/stem_wire/0.1.2/stem_wire.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on GitHub.

This crate contains the [`Wire`] trait which serves as the basic building block
for defining wires within the stem (Simulation Toolbox for Electric Motors)
ecosystem, see the [stem book](https://stefanmathis.github.io/stem_book/) for an
introduction to the framework.

 Additionally, the following predefined wire types are provided:
- [`RoundWire`]: A round, possible hollow wire with insulation.
- [`RectangularWire`]: A rectangular wire with insulation.
- [`StrandedWire`]: A stranded wire composed of multiple other wires.
- [`SffWire`]: An abstract wire type defined by its slot fill factor.
- [`CastWire`]: A wire bar casted directly into the slot of a magnetic core.

The [`resistance`] module provides formulas for calculating resistances of
different basic geometric bodies like cylinders, cuboids, spheres etc.

# Serialization and deserialization

If the `serde` feature is enabled, all wire types from this crate can be
serialized and deserialized. During deserialization, the invariants are
validated (to e.g. prevent negative diameters for a [`RoundWire`]).

Units and quantities can be deserialized from strings representing SI units via
the [dyn_quantity](https://crates.io/crates/dyn_quantity) crate. Similarily,
it is possible to serialize the quantities of a wire as value-unit strings using
the [serialize_with_units](https://docs.rs/dyn_quantity/latest/dyn_quantity/quantity/serde_impl/fn.serialize_with_units.html) function.

See the chapter [serialization and deserialization](https://stefanmathis.github.io/stem_book/serialization_and_deserialization.html) of the [stem book](https://stefanmathis.github.io/stem_book/)
for details.

# Documentation

The full API documentation is available at
[https://docs.rs/stem_wire/0.1.2/stem_wire/](https://docs.rs/stem_wire/0.1.2/stem_wire/).