use super::{impl_covers_from_intersects, impl_covers_from_relate, Covers};
use crate::HasDimensions;
use crate::{geometry::*, CoordsIter, Intersects};
use crate::{GeoFloat, GeoNum};

impl<T> Covers<Coord<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl_covers_from_intersects!(Polygon<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(Polygon<T>, [Line<T>]);
impl_covers_from_relate!(Polygon<T>, [LineString<T>,  MultiLineString<T>]);
impl_covers_from_relate!(Polygon<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_relate!(Polygon<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(Polygon<T>, [GeometryCollection<T>]);

impl<T> Covers<Coord<T>> for MultiPolygon<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}
impl_covers_from_intersects!(MultiPolygon<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Line<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [LineString<T>,  MultiLineString<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [GeometryCollection<T>]);
