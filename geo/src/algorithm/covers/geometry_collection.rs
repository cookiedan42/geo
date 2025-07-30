use super::{impl_covers_from_relate, Covers};
use crate::geometry::*;
use crate::Intersects;
use crate::{GeoFloat, GeoNum};

impl<T> Covers<Coord<T>> for GeometryCollection<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.iter().any(|geometry| geometry.intersects(rhs))
    }
}

impl<T> Covers<Point<T>> for GeometryCollection<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Point<T>) -> bool {
        self.iter().any(|geometry| geometry.intersects(rhs))
    }
}

impl_covers_from_relate!(GeometryCollection<T>, [Line<T>, LineString<T>, Polygon<T>, MultiPoint<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Rect<T>, Triangle<T>]);
