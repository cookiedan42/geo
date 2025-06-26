use criterion::{criterion_group, criterion_main, Criterion};
use geo::Contains;
use geo::Point;
use geo::Polygon;
use geo::MultiPoint;
use geo::{coord,point,Triangle};

use geo::algorithm::oriented_bounding_box;
use geo::MinimumRotatedRect;

fn criterion_benchmark(c: &mut Criterion) {
     c.bench_function("rect Norway", |bencher| {
        let norway = geo_test_fixtures::norway_main::<f32>();
        let polygon = Polygon::new(norway, vec![]);

        bencher.iter(|| {
            criterion::black_box(criterion::black_box(&polygon).minimum_rotated_rect());
        });
    });

     c.bench_function("obb Norway", |bencher| {
        let norway = geo_test_fixtures::norway_main::<f32>();
        let polygon = Polygon::new(norway, vec![]);

        bencher.iter(|| {
            criterion::black_box(oriented_bounding_box(&polygon.exterior().0));
        });
    });

    c.bench_function("rect Triangle", |bencher| {
        let pts: Vec<geo::Point> = vec![point!{x:0.0,y:0.0},point!{x:0.0,y:1.0},point!{x:1.0,y:1.0}];
        let mp = MultiPoint::new(pts);

        bencher.iter(|| {
            criterion::black_box(criterion::black_box(&mp).minimum_rotated_rect());
        });
    });

     c.bench_function("obb Triangle", |bencher| {
        let pts: Vec<geo::Coord> = vec![coord!{x:0.0,y:0.0},coord!{x:0.0,y:1.0},coord!{x:1.0,y:1.0}];

        bencher.iter(|| {
            criterion::black_box(oriented_bounding_box(&pts));
        });
    });

       c.bench_function("rect filled  Triangle", |bencher| {
        let triangle = Triangle::new(
            coord!(x: 0., y: 0.),
            coord!(x: 10., y: 0.),
            coord!(x: 5., y: 10.),
        );


        let pts = (0..1000).zip(0..1000)
        .map(|(x,y)| {Point::new(x as f64 / 1000., y as f64/1000.)})
        .filter(|x|triangle.contains(x));

        let mp = MultiPoint::from_iter(pts);

        bencher.iter(|| {
            criterion::black_box(criterion::black_box(&mp).minimum_rotated_rect());
        });
    });

     c.bench_function("obb filled Triangle", |bencher| {
         let triangle = Triangle::new(
            coord!(x: 0., y: 0.),
            coord!(x: 10., y: 0.),
            coord!(x: 5., y: 10.),
        );

        let pts: Vec<geo::Coord> = (0..1000).zip(0..1000)
        .map(|(x,y)| coord!{x:x as f64 / 1000., y: y as f64/1000.})
        .filter(|x|triangle.contains(x))
        .collect();
        bencher.iter(|| {
            criterion::black_box(oriented_bounding_box(&pts));
        });
    });




}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
