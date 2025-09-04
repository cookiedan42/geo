use super::{impl_covers_from_intersects, Covers};
use crate::{geometry::*, CoordsIter, HasDimensions, Intersects};
use crate::{GeoFloat, GeoNum};

/*
    If self is a single line
    and all points of other intersect self,
    then self covers other.
*/

impl<T> Covers<Coord<T>> for Line<T>
where
    T: GeoFloat,
    Self: Covers<Point<T>>,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl_covers_from_intersects!(Line<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_intersects!(Line<T>, [Line<T>]);
impl_covers_from_intersects!(Line<T>, [LineString<T>,  MultiLineString<T>]);
impl_covers_from_intersects!(Line<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Line<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_intersects!(Line<T>, [GeometryCollection<T>]);
