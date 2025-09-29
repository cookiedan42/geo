use criterion::{Criterion, criterion_group, criterion_main};
use geo::algorithm::{Contains, Convert, Relate};
use geo::coordinate_position::CoordPos;
use geo::{BoundingRect, coord, point, polygon};
use geo::{CoordinatePosition, geometry::*};

#[path = "utils/random.rs"]
mod random;
use rand::thread_rng;
use random::*;

const NUMPOINTS: i32 = 1_000_000;

fn whee(c: &mut Criterion) {
    c.bench_function("linestring contains point", |bencher| {
        let linestring = LineString(vec![
            coord!(x: 0.0, y: 0.0),
            coord!(x: 1.0, y: 0.0),
            coord!(x: 1.0, y: 1.0),
            coord!(x: 0.0, y: 1.0),
            coord!(x: 0.0, y: 0.0),
        ]);
        let point = Point::new(0.5, 0.0);
        bencher.iter(|| {
            assert!(criterion::black_box(&linestring).contains(criterion::black_box(&point)));
        });
    });
}

criterion_group!(benches, whee,);
criterion_main!(benches);
