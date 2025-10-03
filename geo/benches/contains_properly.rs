use criterion::{Criterion, criterion_group, criterion_main};
use geo::CoordsIter;
use geo::algorithm::{ContainsProperly, Convert, Relate};
use geo::geometry::*;
use geo::wkt;
use geo_types::PointsIter;

fn compare_poly_in_poly(c: &mut Criterion) {
    use geo::algorithm::{Contains, Relate};

    c.bench_function("contains_properly", |bencher| {
        let poly1: Polygon<f64> =
            wkt! {POLYGON((9 0,9 9,0 9,0 0,9 0),(6 3,6 6,3 6,3 3,6 3))}.convert();
        let poly2: Polygon<f64> =
            wkt! {POLYGON((8 1,8 8,1 8,1 1,8 1),(7 2,7 7,2 7,2 2,7 2))}.convert();

        bencher.iter(|| {
            assert!(criterion::black_box(&poly1).contains_properly(criterion::black_box(&poly2)));
        });
    });
    c.bench_function("contains", |bencher| {
        let poly1: Polygon<f64> =
            wkt! {POLYGON((9 0,9 9,0 9,0 0,9 0),(6 3,6 6,3 6,3 3,6 3))}.convert();
        let poly2: Polygon<f64> =
            wkt! {POLYGON((8 1,8 8,1 8,1 1,8 1),(7 2,7 7,2 7,2 2,7 2))}.convert();

        bencher.iter(|| {
            assert!(criterion::black_box(&poly1).contains(criterion::black_box(&poly2)));
        });
    });

    c.bench_function("relate", |bencher| {
        let poly1: Polygon<f64> =
            wkt! {POLYGON((9 0,9 9,0 9,0 0,9 0),(6 3,6 6,3 6,3 3,6 3))}.convert();
        let poly2: Polygon<f64> =
            wkt! {POLYGON((8 1,8 8,1 8,1 1,8 1),(7 2,7 7,2 7,2 2,7 2))}.convert();

        bencher.iter(|| {
            assert!(
                criterion::black_box(&poly1)
                    .relate(criterion::black_box(&poly2))
                    .is_contains_properly()
            );
        });
    });
}

criterion_group!(
    benches,
    compare_poly_in_poly,
);
criterion_main!(benches);
