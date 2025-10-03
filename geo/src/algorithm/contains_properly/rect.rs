use super::{
    ContainsProperly, impl_contains_properly_from_iter, impl_contains_properly_from_relate,
};
use crate::{CoordsIter, geometry::*};
use crate::{GeoFloat, GeoNum};

impl<T> ContainsProperly<Coord<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        self.min().x < rhs.x && self.min().y < rhs.y && self.max().x > rhs.x && self.max().y > rhs.y
    }
}

impl<T> ContainsProperly<Point<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Point<T>) -> bool {
        self.contains_properly(&rhs.0)
    }
}

impl<T> ContainsProperly<Rect<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Rect<T>) -> bool {
        self.min().x < rhs.min().x
            && self.min().y < rhs.min().y
            && self.max().x > rhs.max().x
            && self.max().y > rhs.max().y
    }
}

impl_contains_properly_from_relate!(Rect<T>, [
MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>
]);

impl_contains_properly_from_iter!(coords_iter: Triangle<T>);
