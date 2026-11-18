use shared::vessel::VesselKind;

use super::*;

#[test]
fn every_vessel_has_parts_and_fills_its_cells() {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    for vessel in VesselKind::ALL {
        let parts = vessel_parts(vessel, &mut meshes, &mut materials);
        assert!(parts.len() >= 3, "{vessel:?}");
        let shape = vessel.shape();
        assert_eq!(cells(vessel), (shape.width, shape.height));
    }
}
