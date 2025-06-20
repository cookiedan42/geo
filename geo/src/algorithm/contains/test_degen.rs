
#[cfg(test)]
mod degenerate_test {
    use std::ops::Mul;

    use geo_types::MultiLineString;

    use super::*;
    use crate::algorithm::contains::Contains;
    use crate::algorithm::relate::Relate;
    use crate::line_measures::LengthMeasurable;
    use crate::{coord, linestring_segment, Area, Coord, CoordsIter, Euclidean, Length};
    use crate::{Line, LineString}; // 1d
    use crate::{MultiPoint, Point}; // 0d
    use crate::{MultiPolygon, Polygon, Rect, Triangle}; // 2d

    const zero: Coord<f64> = coord! {x:0.0,y:0.0};
    const one: Coord<f64> = coord! {x:1.0,y:1.0};

    fn a() {
        let linestring = Rect::new(coord! {x:0,y:0}, coord! {x:10,y:10});
        let poly = linestring.to_polygon();
        let ls = poly.exterior();

        let ll: std::iter::Copied<std::slice::Iter<'_, Coord<i32>>> = ls.coords_iter();
    }

    #[test]
    fn test_0d_contains_degenerate_1d() {
        let line = Line::new(zero, zero);
        let linestring = LineString::new(vec![zero, zero]);
        let multilinestring = MultiLineString::new(vec![linestring.clone()]);

        let point: Point<f64> = zero.into();
        let multipoint = MultiPoint::new(vec![point]);

        assert!(point.contains(&line));
        assert!(point.contains(&linestring));
        assert!(point.contains(&multilinestring));
        assert!(!multipoint.contains(&line));
        assert!(!multipoint.contains(&linestring));
        assert!(!multipoint.contains(&multilinestring));

        assert!(!point.relate(&line).is_contains());
        assert!(point.relate(&linestring).is_contains());
        assert!(point.relate(&multilinestring).is_contains());
        assert!(!multipoint.relate(&line).is_contains());
        assert!(multipoint.relate(&linestring).is_contains());
        assert!(multipoint.relate(&multilinestring).is_contains());
    }

    #[test]
    fn test_1d_contains_degenerate_2d() {
        let line = Line::new(zero, one);
        let linestring = LineString::new(vec![zero, one]);
        let multilinestring = MultiLineString::new(vec![linestring.clone()]);

        // degenerate as points
        let polygon = Polygon::new(vec![zero, zero, zero].into(), vec![]);
        let multipolygon = MultiPolygon::new(vec![polygon.clone()]);
        let rectangle = Rect::new(zero, zero);
        let triangle = Triangle::new(zero, zero, zero);

        assert!(!line.contains(&polygon));
        assert!(!line.contains(&multipolygon));
        assert!(!line.contains(&rectangle));
        assert!(!line.contains(&triangle));

        assert!(!linestring.contains(&polygon));
        assert!(!linestring.contains(&multipolygon));
        assert!(!linestring.contains(&rectangle));
        assert!(!linestring.contains(&triangle));

        assert!(!multilinestring.contains(&polygon));
        assert!(!multilinestring.contains(&multipolygon));
        assert!(!multilinestring.contains(&rectangle));
        assert!(!multilinestring.contains(&triangle));

        assert!(!line.relate(&polygon).is_contains());
        assert!(!line.relate(&multipolygon).is_contains());
        assert!(!line.relate(&rectangle).is_contains());
        assert!(!line.relate(&triangle).is_contains());

        assert!(!linestring.relate(&polygon).is_contains());
        assert!(!linestring.relate(&multipolygon).is_contains());
        assert!(!linestring.relate(&rectangle).is_contains());
        assert!(!linestring.relate(&triangle).is_contains());

        assert!(!multilinestring.relate(&polygon).is_contains());
        assert!(!multilinestring.relate(&multipolygon).is_contains());
        assert!(!multilinestring.relate(&rectangle).is_contains());
        assert!(!multilinestring.relate(&triangle).is_contains());

        // degenerate as lines
        let polygon = Polygon::new(vec![zero, one, zero].into(), vec![]);
        let multipolygon = MultiPolygon::new(vec![polygon.clone()]);
        let rectangle = Rect::new(zero, one);
        let triangle = Triangle::new(zero, one, zero);

        assert!(!line.contains(&polygon));
        assert!(!line.contains(&multipolygon));
        assert!(!line.contains(&rectangle));
        assert!(!line.contains(&triangle));

        assert!(!linestring.contains(&polygon));
        assert!(!linestring.contains(&multipolygon));
        assert!(!linestring.contains(&rectangle));
        assert!(!linestring.contains(&triangle));

        assert!(!multilinestring.contains(&polygon));
        assert!(!multilinestring.contains(&multipolygon));
        assert!(!multilinestring.contains(&rectangle));
        assert!(!multilinestring.contains(&triangle));

        assert!(!line.relate(&polygon).is_contains());
        assert!(!line.relate(&multipolygon).is_contains());
        assert!(!line.relate(&rectangle).is_contains());
        assert!(!line.relate(&triangle).is_contains());

        assert!(!linestring.relate(&polygon).is_contains());
        assert!(!linestring.relate(&multipolygon).is_contains());
        assert!(!linestring.relate(&rectangle).is_contains());
        assert!(!linestring.relate(&triangle).is_contains());

        assert!(!multilinestring.relate(&polygon).is_contains());
        assert!(!multilinestring.relate(&multipolygon).is_contains());
        assert!(!multilinestring.relate(&rectangle).is_contains());
        assert!(!multilinestring.relate(&triangle).is_contains());
    }

    #[test]
    fn test_0d_contains_degenerate_2d() {
        let polygon = Polygon::new(vec![zero, zero, zero].into(), vec![]);
        let multipolygon = MultiPolygon::new(vec![polygon.clone()]);
        let rectangle = Rect::new(zero, zero);
        let triangle = Triangle::new(zero, zero, zero);

        let point: Point = zero.into();
        let multipoint = MultiPoint::new(vec![point.clone()]);

        assert!(point.contains(&polygon));
        assert!(point.contains(&multipolygon));
        assert!(point.contains(&rectangle));
        assert!(point.contains(&triangle));

        assert!(!multipoint.contains(&polygon));
        assert!(!multipoint.contains(&multipolygon));
        assert!(!multipoint.contains(&rectangle));
        assert!(!multipoint.contains(&triangle));

        assert!(!point.relate(&polygon).is_contains());
        assert!(!point.relate(&multipolygon).is_contains());
        assert!(!point.relate(&rectangle).is_contains());
        assert!(!point.relate(&triangle).is_contains());

        assert!(!multipoint.relate(&polygon).is_contains());
        assert!(!multipoint.relate(&multipolygon).is_contains());
        assert!(!multipoint.relate(&rectangle).is_contains());
        assert!(!multipoint.relate(&triangle).is_contains());
    }

    #[test]
    fn test_degenerate_2d_contains_degenerate_1d() {
        //1d as 0d
        let line = Line::new(zero, zero);
        let linestring = LineString::new(vec![zero, zero]);
        let multilinestring = MultiLineString::new(vec![linestring.clone()]);

        // 2d as 1d
        let polygon = Polygon::new(vec![zero, one, zero].into(), vec![]);
        let multipolygon = MultiPolygon::new(vec![polygon.clone()]);
        let rectangle = Rect::new(zero, one);
        let triangle = Triangle::new(zero, one, zero);

        // 2d as 0d
        let polygon = Polygon::new(vec![zero, zero, zero].into(), vec![]);
        let multipolygon = MultiPolygon::new(vec![polygon.clone()]);
        let rectangle = Rect::new(zero, zero);
        let triangle = Triangle::new(zero, zero, zero);
    }

    #[test]
    fn test_degenerate_2d_contains_0d() {}
    #[test]
    fn test_degenerate_1d_contains_0d() {}
}
