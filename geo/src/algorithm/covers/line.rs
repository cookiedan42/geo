use super::{impl_covers_from_intersects, impl_covers_from_relate, Covers};
use crate::{geometry::*, CoordsIter, Intersects};
use crate::{GeoFloat, GeoNum};

impl<T> Covers<Coord<T>> for Line<T>
where
    T: GeoFloat,
    Self: Covers<Point<T>>,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.covers(&Point::new(rhs.x, rhs.y))
    }
}

impl_covers_from_intersects!(Line<T>, [ Point<T>, MultiPoint<T>]);
impl_covers_from_intersects!(Line<T>, [Line<T>]);
impl_covers_from_intersects!(Line<T>, [ LineString<T>,  MultiLineString<T>]);
impl_covers_from_intersects!(Line<T>, [ Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Line<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_intersects!(Line<T>, [Geometry<T>, GeometryCollection<T>]);
