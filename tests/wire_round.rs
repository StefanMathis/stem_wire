use std::sync::Arc;

use approx::{self, assert_abs_diff_eq};
use stem_wire::prelude::{unary::Linear, *};

#[test]
fn test_invalid_wire_dimensions() {
    assert!(
        RoundWire::new(
            Arc::new(Material::default()),
            Length::new::<millimeter>(100.0),
            Length::new::<millimeter>(200.0),
            Length::new::<millimeter>(1000.0)
        )
        .is_err()
    );
    assert!(
        RoundWire::new(
            Arc::new(Material::default()),
            Length::new::<millimeter>(-300.0),
            Length::new::<millimeter>(200.0),
            Length::new::<millimeter>(1000.0)
        )
        .is_err()
    );
    assert!(
        RoundWire::new(
            Arc::new(Material::default()),
            Length::new::<millimeter>(300.0),
            Length::new::<millimeter>(200.0),
            Length::new::<millimeter>(-1000.0)
        )
        .is_err()
    );
}

#[test]
fn test_cross_section_area() {
    {
        let wire = RoundWire::default();
        approx::assert_abs_diff_eq!(
            wire.conductor_area().get::<square_meter>(),
            0.785398,
            epsilon = 1e-6
        );
        approx::assert_abs_diff_eq!(
            wire.overall_area().get::<square_meter>(),
            0.785398,
            epsilon = 1e-6
        );
    }
    {
        let wire = RoundWire::new(
            Default::default(),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.0),
            Length::new::<millimeter>(100.0),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(
            wire.conductor_area().get::<square_meter>(),
            0.785398,
            epsilon = 1e-6
        );
        approx::assert_abs_diff_eq!(
            wire.overall_area().get::<square_meter>(),
            1.130973,
            epsilon = 1e-6
        );
    }
}

#[test]
fn test_variable_resistivity() {
    let mut mat = Material::default();
    mat.set_electrical_resistivity(
        VarQuantity::try_from_quantity_function(Linear::new(
            DynQuantity::new(1.0, PredefUnit::ElectricResistivity),
            DynQuantity::new(2.0, PredefUnit::ElectricResistivity),
        ))
        .unwrap(),
    );

    let wire = RoundWire::new(
        Arc::new(mat),
        Length::new::<meter>(1.0),
        Length::new::<meter>(0.0),
        Length::new::<millimeter>(100.0),
    )
    .unwrap();

    let conditions = [ThermodynamicTemperature::new::<degree_celsius>(20.0).into()];

    assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<meter>(1.0),
            Area::new::<square_meter>(0.0),
            1,
            &conditions
        )
        .get::<ohm>(),
        2.546,
        epsilon = 1e-3
    );
}
