use super::{Covers, impl_covers_from_intersects};
use crate::GeoNum;
use crate::HasDimensions;
use crate::{CoordsIter, Intersects, geometry::*};

/*
    If self is a simple convex polygon
    and all points of other intersect self,
    then self covers other.
*/

impl_covers_from_intersects!(coord: Rect<T>);

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

impl_covers_from_intersects!(Rect<T>, [Point<T>, MultiPoint<T>, Line<T>, LineString<T>, MultiLineString<T>,Rect<T>, Triangle<T>,GeometryCollection<T>]);
