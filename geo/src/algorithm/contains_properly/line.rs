use super::{ContainsProperly};
use crate::GeoNum;
use crate::{CoordsIter,HasDimensions,Intersects};
use crate::geometry::*;
// all points intersect the line
// but none of them intersect start or end

macro_rules! impl_contains_properly_from_intersects {
    ([$($target:ty),*]) => {
        $(
            impl<T> ContainsProperly<$target> for Line<T>
            where
                T: GeoNum
            {
                fn contains_properly(&self, target: &$target) -> bool {

                    if  HasDimensions::is_empty(self) || HasDimensions::is_empty(target) {return false;}

                    target.exterior_coords_iter().all(|c| self.intersects(&c) )
                    && target.exterior_coords_iter().all(|c| !self.start.intersects(&c) && !self.end.intersects(&c) )
                }
            }
        )*
    };
}

impl_contains_properly_from_intersects!([
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);
