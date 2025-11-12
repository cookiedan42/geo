use super::ContainsProperly;
use crate::GeoNum;
use crate::geometry::*;

// bbox is ~ 2n to 4n checks for bbox followed by rect-rect check
macro_rules! impl_contains_properly_from_bbox {
    ([$($target:ty),*]) => {
        $(
            impl<T> ContainsProperly<$target> for Rect<T>
            where
                T: GeoNum
            {
                fn contains_properly(&self, target: &$target) -> bool {
                    use $crate::BoundingRect;

                    let Some(bbox) = target.bounding_rect() else {return false;};
                    self.contains_properly(&bbox)
                }
            }
        )*
    };
}

impl<T> ContainsProperly<Coord<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        rhs.x < self.max().x && rhs.y < self.max().y && rhs.x > self.min().x && rhs.y > self.min().y
    }
}

impl<T> ContainsProperly<Point<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Point<T>) -> bool {
        self.contains_properly(&rhs.0)
    }
}

impl<T> ContainsProperly<Line<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Line<T>) -> bool {
        self.contains_properly(&rhs.start) && self.contains_properly(&rhs.end)
    }
}

impl<T> ContainsProperly<Triangle<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Triangle<T>) -> bool {
        self.contains_properly(&rhs.0)
            && self.contains_properly(&rhs.1)
            && self.contains_properly(&rhs.2)
    }
}

impl<T> ContainsProperly<Rect<T>> for Rect<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Rect<T>) -> bool {
        rhs.max().x < self.max().x
            && rhs.max().y < self.max().y
            && rhs.min().x > self.min().x
            && rhs.min().y > self.min().y
    }
}

impl_contains_properly_from_bbox!( [
MultiPoint<T>,
LineString<T>, MultiLineString<T>,
Polygon<T>,MultiPolygon<T>,
GeometryCollection<T>
]);
