use super::ContainsProperly;
use crate::GeoNum;
use crate::Contains;
use crate::geometry::*;

// point and multipoint have no boundary
// they have the same impl as contains
macro_rules! impl_contains_properly_from_contains {
    ($for:ty, [$($target:ty),*]) => {
        $(
            impl<T> ContainsProperly<$target> for $for
            where
                T: GeoNum
            {
                fn contains_properly(&self, target: &$target) -> bool {
                    use $crate::CoordsIter;
                    use $crate::algorithm::dimensions::HasDimensions;
                    if  HasDimensions::is_empty(self) || HasDimensions::is_empty(target) {return false;}
                    target
                        .exterior_coords_iter()
                        .all(|c| self.contains(&c))
                }
            }
        )*
    };
}

impl_contains_properly_from_contains!(Point<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);

impl_contains_properly_from_contains!(MultiPoint<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);
