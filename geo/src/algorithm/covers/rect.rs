use super::{impl_covers_from_intersects, Covers};
use crate::GeoNum;
use crate::{geometry::*, CoordsIter, Intersects};

impl<T> Covers<Coord<T>> for Rect<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl<T> Covers<Point<T>> for Rect<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Point<T>) -> bool {
        self.intersects(rhs)
    }
}

impl<T> Covers<Polygon<T>> for Rect<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Polygon<T>) -> bool {
        rhs.exterior_coords_iter().all(|c| self.intersects(&c))
    }
}

impl<T> Covers<MultiPolygon<T>> for Rect<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &MultiPolygon<T>) -> bool {
        rhs.exterior_coords_iter().all(|c| self.intersects(&c))
    }
}

impl_covers_from_intersects!(Rect<T>, [MultiPoint<T>]);
impl_covers_from_intersects!(Rect<T>, [Line<T>, LineString<T>, MultiLineString<T>]);
impl_covers_from_intersects!(Rect<T>, [ Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Rect<T>, [Geometry<T>, GeometryCollection<T>]);
