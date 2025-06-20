use super::{impl_contains_from_relate, impl_contains_geometry_for, Contains};
use crate::geometry::*;
use crate::{GeoFloat, GeoNum};
use crate::{HasDimensions, Relate};

// ┌─────────────────────────────┐
// │ Implementations for Polygon │
// └─────────────────────────────┘
impl<T> Contains<Coord<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        use crate::coordinate_position::{CoordPos, CoordinatePosition};

        self.coordinate_position(coord) == CoordPos::Inside
    }
}

impl<T> Contains<Point<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

impl<T> Contains<MultiPoint<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains(&self, mp: &MultiPoint<T>) -> bool {
        use crate::coordinate_position::{CoordPos, CoordinatePosition};
        // at least one point must be fully within
        // others can be on the boundary
        mp.iter().any(|p| self.contains(p))
            && mp
                .iter()
                .all(|p| self.coordinate_position(&p.0) != CoordPos::Outside)
    }
}

impl_contains_from_relate!(Polygon<T>, [Line<T>, LineString<T>, Polygon<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Rect<T>, Triangle<T>]);
impl_contains_geometry_for!(Polygon<T>);

// ┌──────────────────────────────────┐
// │ Implementations for MultiPolygon │
// └──────────────────────────────────┘

impl<T> Contains<Coord<T>> for MultiPolygon<T>
where
    T: GeoNum,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        self.iter().any(|poly| poly.contains(coord))
    }
}

impl<T> Contains<Point<T>> for MultiPolygon<T>
where
    T: GeoNum,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

impl<T: GeoNum> Contains<MultiPoint<T>> for MultiPolygon<T> {
    fn contains(&self, rhs: &MultiPoint<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.iter().all(|point| self.contains(point))
    }
}

impl<F> Contains<Line<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &Line<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<LineString<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &LineString<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<MultiLineString<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &MultiLineString<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<Polygon<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &Polygon<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<MultiPolygon<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &MultiPolygon<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<GeometryCollection<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &GeometryCollection<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<Rect<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &Rect<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

impl<F> Contains<Triangle<F>> for MultiPolygon<F>
where
    F: GeoFloat,
{
    fn contains(&self, rhs: &Triangle<F>) -> bool {
        rhs.relate(self).is_within()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{polygon, MultiPoint, Relate};

    #[test]
    fn test_polygon_contains_empty_multipoint() {
        let poly = polygon![
            (x: 0.0, y: 0.0),
            (x: 10.0, y: 0.0),
            (x: 10.0, y: 10.0),
            (x: 0.0, y: 10.0),
            (x: 0.0, y: 0.0),
        ];
        let empty: MultiPoint<f64> = MultiPoint::new(Vec::new());

        assert!(!poly.contains(&empty));
        assert!(!poly.relate(&empty).is_contains());
    }
}
