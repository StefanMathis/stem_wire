use std::sync::Arc;

use approx;

use stem_wire::prelude::*;

#[test]
fn test_slot_filling_factor() {
    let material: Arc<Material> = Default::default();
    let wire_1 = RoundWire::new(
        material.clone(),
        Length::new::<millimeter>(1.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1),
    )
    .unwrap();
    let wire_2 = RoundWire::new(
        material.clone(),
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.1),
    )
    .unwrap();

    let strand_list = vec![
        WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
    ];
    let wire_litz = StrandedWire::new(strand_list).unwrap();

    approx::assert_abs_diff_eq!(
        wire_litz.slot_fill_factor_conductor(Area::new::<square_millimeter>(40.0), 2),
        0.353429,
        epsilon = 1e-6
    );
    approx::assert_abs_diff_eq!(
        wire_litz.slot_fill_factor_overall(Area::new::<square_millimeter>(40.0), 2),
        0.436681,
        epsilon = 1e-6
    );

    // Modify the insulation thickness of wire_2 and repeat the test
    let wire_2 = RoundWire::new(
        material,
        Length::new::<millimeter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.2),
    )
    .unwrap();

    let strand_list = vec![
        WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
    ];
    let wire_litz = StrandedWire::new(strand_list).unwrap();

    approx::assert_abs_diff_eq!(
        wire_litz.slot_fill_factor_conductor(Area::new::<square_millimeter>(40.0), 2),
        0.353429,
        epsilon = 1e-6
    ); // Same value as above
    approx::assert_abs_diff_eq!(
        wire_litz.slot_fill_factor_overall(Area::new::<square_millimeter>(40.0), 2),
        0.508938,
        epsilon = 1e-6
    ); // Changes
}

#[test]
fn test_resistance_calculation() {
    let mut material = Material::default();
    material.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
        ohm_meter,
    >(1.0)));
    let material = Arc::new(material);

    let wire_1 = RoundWire::new(
        material.clone(),
        Length::new::<meter>(1.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.0),
    )
    .unwrap();
    let wire_2 = RoundWire::new(
        material.clone(),
        Length::new::<meter>(2.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(0.0),
    )
    .unwrap();

    let r1_20 = wire_1.resistance(
        Length::new::<meter>(1.0),
        Area::new::<square_millimeter>(0.0),
        1,
        &[],
    );
    approx::assert_abs_diff_eq!(r1_20.get::<ohm>(), 1.273239, epsilon = 1e-6);
    let r2_20 = wire_2.resistance(
        Length::new::<meter>(1.0),
        Area::new::<square_millimeter>(0.0),
        1,
        &[],
    );
    approx::assert_abs_diff_eq!(r2_20.get::<ohm>(), 0.318309, epsilon = 1e-6);

    let strand_list = vec![
        WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
    ];
    let wire_litz = StrandedWire::new(strand_list).unwrap();

    let r_litz20 = wire_litz.resistance(
        Length::new::<meter>(1.0),
        Area::new::<square_millimeter>(0.0),
        1,
        &[],
    );
    let expected_value = (r1_20 * r2_20 / 2.0) / (r1_20 + r2_20 / 2.0);
    approx::assert_abs_diff_eq!(
        r_litz20.get::<ohm>(),
        expected_value.get::<ohm>(),
        epsilon = 1e-6
    );

    // Second test, but with other numbers
    let strand_list = vec![
        WireGroup::new(Box::new(wire_1.clone()), 5.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 7.try_into().unwrap()),
    ];
    let wire_litz = StrandedWire::new(strand_list).unwrap();

    let r_litz20 = wire_litz.resistance(
        Length::new::<meter>(1.0),
        Area::new::<square_millimeter>(0.0),
        1,
        &[],
    );
    let expected_value = (r1_20 / 5.0 * r2_20 / 7.0) / (r1_20 / 5.0 + r2_20 / 7.0);
    approx::assert_abs_diff_eq!(
        r_litz20.get::<ohm>(),
        expected_value.get::<ohm>(),
        epsilon = 1e-6
    );

    // Third test with more wire types
    let strand_list = vec![
        WireGroup::new(Box::new(wire_1.clone()), 5.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 7.try_into().unwrap()),
        WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
        WireGroup::new(Box::new(wire_2.clone()), 1.try_into().unwrap()),
    ];
    let wire_litz = StrandedWire::new(strand_list).unwrap();

    let r_litz20 = wire_litz.resistance(
        Length::new::<meter>(1.0),
        Area::new::<square_millimeter>(0.0),
        1,
        &[],
    );
    approx::assert_abs_diff_eq!(r_litz20.get::<ohm>(), 0.033506, epsilon = 1e-6);
}

/**
Building a stranded wire out of different materials should fail
 */
#[test]
fn test_wire_with_different_materials() {
    // Different pointers
    {
        let mut mat_1 = Material::default();
        mat_1.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(1.0)));
        let mat_1 = Arc::new(mat_1);

        let wire_1 = RoundWire::new(
            mat_1,
            Length::new::<millimeter>(1.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let mut mat_2 = Material::default();
        mat_2.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(2.0)));
        let mat_2 = Arc::new(mat_2);

        let wire_2 = RoundWire::new(
            mat_2,
            Length::new::<millimeter>(2.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let strand_list = vec![
            WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
            WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
        ];
        assert!(StrandedWire::new(strand_list).is_err());
    }

    // Different pointers, but same material data - this succeeds
    {
        let mut mat_1 = Material::default();
        mat_1.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(1.0)));
        let mat_1 = Arc::new(mat_1);

        let wire_1 = RoundWire::new(
            mat_1,
            Length::new::<millimeter>(1.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let mut mat_2 = Material::default();
        mat_2.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(1.0)));
        let mat_2 = Arc::new(mat_2);

        let wire_2 = RoundWire::new(
            mat_2,
            Length::new::<millimeter>(2.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let strand_list = vec![
            WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
            WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
        ];
        assert!(StrandedWire::new(strand_list).is_ok());
    }

    // Different pointers and different material data - this fails
    {
        let mut mat_1 = Material::default();
        mat_1.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(1.0)));
        let mat_1 = Arc::new(mat_1);

        let wire_1 = RoundWire::new(
            mat_1,
            Length::new::<millimeter>(1.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let mut mat_2 = Material::default();
        mat_2.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(2.0)));
        let mat_2 = Arc::new(mat_2);

        let wire_2 = RoundWire::new(
            mat_2,
            Length::new::<millimeter>(2.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let strand_list = vec![
            WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
            WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
        ];
        assert!(StrandedWire::new(strand_list).is_err());
    }

    // Same pointer - this succeeds
    {
        let mut mat_1 = Material::default();
        mat_1.set_electrical_resistivity(VarQuantity::Constant(ElectricalResistivity::new::<
            ohm_meter,
        >(1.0)));
        let mat_1 = Arc::new(mat_1);

        let wire_1 = RoundWire::new(
            mat_1.clone(),
            Length::new::<millimeter>(1.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let wire_2 = RoundWire::new(
            mat_1,
            Length::new::<millimeter>(2.0),
            Length::new::<millimeter>(0.0),
            Length::new::<millimeter>(0.1),
        )
        .unwrap();

        let strand_list = vec![
            WireGroup::new(Box::new(wire_1.clone()), 1.try_into().unwrap()),
            WireGroup::new(Box::new(wire_2.clone()), 2.try_into().unwrap()),
        ];

        assert!(StrandedWire::new(strand_list).is_ok());
    }
}
