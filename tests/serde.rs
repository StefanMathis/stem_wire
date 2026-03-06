use indoc::indoc;
use serde_mosaic::{DatabaseManager, SerdeYaml};
use stem_wire::prelude::*;

fn create_dbm() -> DatabaseManager {
    return DatabaseManager::open("stem_test_database/src", SerdeYaml).expect("must exist");
}

#[test]
fn test_wire_rectangular() {
    {
        let yaml = indoc! {"
        ---
        conductor_material:
            name: Copper
        height: 5.5 mm
        width: 6.7 mm
        insulation_thickness: 0.0 mm
        "};

        let wire = create_dbm()
            .from_str::<RectangularWire, SerdeYaml>(yaml)
            .unwrap();
        assert_eq!(wire.overall_area().get::<square_meter>(), 5.5 * 6.7 * 1e-6);
    }

    {
        // Fails because insulation thickness is negative
        let yaml = indoc! {"
        ---
        conductor_material:
            name: Copper
        height: 5.5 mmm
        width: 6.7 mmm
        insulation_thickness: -1.0 mm
        "};

        assert!(
            create_dbm()
                .from_str::<RectangularWire, SerdeYaml>(yaml)
                .is_err()
        );
    }
}

#[test]
fn test_load_wire_round() {
    // Read from the database
    let yaml = indoc! {"
        outer_diameter: 1 mm
        inner_diameter: 0 mm
        insulation_thickness: 0 mm
        conductor_material:
          name: Copper
        "};

    let wire = create_dbm().from_str::<RoundWire, SerdeYaml>(yaml).unwrap();
    approx::assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<millimeter>(100.0),
            Area::new::<square_meter>(0.0),
            1,
            &[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]
        )
        .get::<ohm>(),
        0.00316718,
        epsilon = 1e-6
    );
}

#[test]
fn test_deserialize_with_bad_parameters() {
    // Read from the database
    let yaml = indoc! {"
        outer_diameter: 1 mm
        inner_diameter: 2 mm # <== INNER DIAMETER BIGGER THAN OUTER_DIAMETER
        insulation_thickness: 0.0
        material:
          name: Copper
        "};

    assert!(create_dbm().from_str::<RoundWire, SerdeYaml>(yaml).is_err());
}

#[test]
fn test_load_wire_stranded() {
    // Read from the database
    let yaml = indoc! {"
    ---
    - wire:
        RoundWire:
          conductor_material:
            name: Copper
          outer_diameter: 1 mm
          inner_diameter: 0 mm
          insulation_thickness: 0 mm
      number_wires: 1
    - wire:
        RoundWire:
          conductor_material:
            name: Copper
          outer_diameter: 1 mm
          inner_diameter: 0 mm
          insulation_thickness: 0 mm
      number_wires: 2
    "};

    let wire = create_dbm()
        .from_str::<StrandedWire, SerdeYaml>(yaml)
        .unwrap();
    approx::assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<meter>(0.1),
            Area::new::<square_millimeter>(0.0),
            1,
            &[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]
        )
        .get::<ohm>(),
        0.00316718 / 3.0,
        epsilon = 1e-6
    );

    // ======================================================================

    // Read from the database
    let yaml = indoc! {"
    ---
    - wire:
        RoundWire:
          conductor_material:
            name: Copper
          outer_diameter: 0.63 mm
          inner_diameter: 0 mm
          insulation_thickness: 0.04725 mm
      number_wires: 3
    "};

    let wire = create_dbm()
        .from_str::<StrandedWire, SerdeYaml>(yaml)
        .unwrap();
    approx::assert_abs_diff_eq!(
        wire.resistance(
            Length::new::<meter>(0.1),
            Area::new::<square_millimeter>(0.0),
            1,
            &[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]
        )
        .get::<ohm>(),
        0.0079798 / 3.0,
        epsilon = 1e-6
    );
}
