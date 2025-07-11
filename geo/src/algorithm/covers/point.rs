use super::{impl_covers_from_relate, Covers};
use crate::covers::impl_covers_from_intersects;
use crate::geometry::*;
use crate::{CoordsIter, Intersects};
use crate::{GeoFloat, GeoNum};

impl<T, G> Covers<G> for Coord<T>
where
    T: GeoFloat,
    Point<T>: Covers<G>,
{
    fn covers(&self, rhs: &G) -> bool {
        Point::new(self.x, self.y).covers(rhs)
    }
}

impl<T> Covers<Coord<T>> for Point<T>
where
    T: GeoFloat,
    Self: Covers<Point<T>>,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.covers(&Point::new(rhs.x, rhs.y))
    }
}

impl<T> Covers<Point<T>> for Point<T>
where
    T: GeoFloat,
{
    fn covers(&self, rhs: &Point<T>) -> bool {
        self.x() == rhs.x() && self.y() == rhs.y()
    }
}

impl_covers_from_intersects!(Point<T>, [MultiPoint<T>]);

impl_covers_from_intersects!(Point<T>, [Line<T>]);
impl_covers_from_intersects!(Point<T>, [ LineString<T>,  MultiLineString<T>]);
impl_covers_from_intersects!(Point<T>, [ Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Point<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_intersects!(Point<T>, [Geometry<T>, GeometryCollection<T>]);

impl<T> Covers<Coord<T>> for MultiPoint<T>
where
    T: GeoFloat,
    Self: Covers<Point<T>>,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.covers(&Point::new(rhs.x, rhs.y))
    }
}

impl_covers_from_intersects!(MultiPoint<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(MultiPoint<T>, [Line<T>]);
impl_covers_from_relate!(MultiPoint<T>, [ LineString<T>,  MultiLineString<T>]);
impl_covers_from_relate!(MultiPoint<T>, [ Rect<T>, Triangle<T>]);
impl_covers_from_relate!(MultiPoint<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(MultiPoint<T>, [Geometry<T>, GeometryCollection<T>]);
