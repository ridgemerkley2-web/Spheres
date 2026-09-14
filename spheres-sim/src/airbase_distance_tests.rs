use super::*;
fn point(lon: f64, lat: f64) -> BaseLocation {
    BaseLocation {
        district: String::new(),
        name: String::new(),
        lon,
        lat,
    }
}
#[test]
fn great_circle_range_is_stable_at_poles_dateline_and_boundary() {
    let origin = point(0.0, 0.0);
    assert_eq!(distance_km(&origin, &origin), 0.0);
    for (b, expected) in [
        (point(90.0, 0.0), 10007.543398010286),
        (point(180.0, 0.0), 20015.086796020572),
        (point(0.0, 90.0), 10007.543398010286),
        (point(1.0, 0.0), 111.19492664455873),
    ] {
        assert!((distance_km(&origin, &b) - expected).abs() < 1e-5);
    }
    assert!(
        (distance_km(&point(179.0, 0.0), &point(-179.0, 0.0)) - 222.38985328911747).abs() < 1e-5
    );
    assert!(distance_km(&point(-180.0, 90.0), &point(180.0, 90.0)) < 1e-8);
    for radius in [700.0, 945.0, 1400.0, 1890.0] {
        let target = point((radius / 6371.0) * 180.0 / std::f64::consts::PI, 0.0);
        assert!((distance_km(&origin, &target) - radius).abs() < 1e-7);
        assert_eq!(distance_km(&origin, &target), distance_km(&target, &origin));
    }
}
