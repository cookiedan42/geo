use super::{ContainsProperly, impl_contains_properly_convex_poly};
use crate::Contains;
use crate::GeoNum;
use crate::geometry::*;

impl<T> ContainsProperly<Coord<T>> for Triangle<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        // neither can be empty
        self.contains(rhs)
    }
}

impl_contains_properly_convex_poly!(Triangle<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
Triangle<T>, Rect<T>,
GeometryCollection<T>
]);
