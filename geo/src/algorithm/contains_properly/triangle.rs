use super::ContainsProperly;
use crate::Contains;
use crate::GeoNum;
use crate::geometry::*;

macro_rules! impl_contains_properly_from_coords {
    ([$($target:ty),*]) => {
        $(
            impl<T> ContainsProperly<$target> for Triangle<T>
            where
                T: GeoNum
            {
                fn contains_properly(&self, target: &$target) -> bool {
                    use $crate::CoordsIter;
                    use $crate::algorithm::dimensions::HasDimensions;
                    if  HasDimensions::is_empty(target) {return false;}
                    target
                        .exterior_coords_iter()
                        .all(|c| self.contains_properly(&c))
                }
            }
        )*
    };
}

impl<T> ContainsProperly<Coord<T>> for Triangle<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        // same impl
        self.contains(rhs)
    }
}

impl_contains_properly_from_coords!( [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);
