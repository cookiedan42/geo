use super::{ContainsProperly, impl_contains_properly_convex_poly};
use crate::GeoNum;
use crate::geometry::*;

impl<T> ContainsProperly<Coord<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        // neither can be empty
        self.max().x > rhs.x && self.max().y > rhs.y && self.min().x < rhs.x && self.min().y < rhs.y
    }
}

impl<T> ContainsProperly<Rect<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Rect<T>) -> bool {
        // neither can be empty
        self.max().x > rhs.max().x
            && self.max().y > rhs.max().y
            && self.min().x < rhs.min().x
            && self.min().y < rhs.min().y
    }
}

impl_contains_properly_convex_poly!(Rect<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
Triangle<T>,
GeometryCollection<T>
]);
