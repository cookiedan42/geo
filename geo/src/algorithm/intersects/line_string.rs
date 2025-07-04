use super::{has_disjoint_bboxes, Intersects};
use crate::BoundingRect;
use crate::*;

macro_rules! blanket_intersects_linestring {
    ($t:ty) => {
        impl<T> $crate::Intersects<$t> for LineString<T>
        where
            T: CoordNum,
            Line<T>: Intersects<$t>,
            $t: BoundingRect<T>,
        {
            fn intersects(&self, rhs: &$t) -> bool {
                if has_disjoint_bboxes(self, rhs) {
                return false;
            }
            self.lines().any(|l| l.intersects(rhs))
            }
        }
    };
}

blanket_intersects_linestring!(Coord<T>);
blanket_intersects_linestring!(MultiLineString<T>);
blanket_intersects_linestring!(Point<T>);
blanket_intersects_linestring!(MultiPoint<T>);
blanket_intersects_linestring!(Polygon<T>);
blanket_intersects_linestring!(MultiPolygon<T>);
blanket_intersects_linestring!(Rect<T>);
blanket_intersects_linestring!(Triangle<T>);
blanket_intersects_linestring!(Geometry<T>);
blanket_intersects_linestring!(GeometryCollection<T>);

impl <T> Intersects<LineString<T>> for LineString<T>
where
    T: CoordNum,
    Line<T>: Intersects<Line<T>>,
    LineString<T>: BoundingRect<T>,
{
    fn intersects(&self, rhs: &LineString<T>) -> bool {
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }
        self.lines().any(|l| rhs.lines().any(|other| l.intersects(&other)))
    }
}


impl <T> Intersects<Line<T>> for LineString<T>
where
    T: CoordNum,
    Line<T>: Intersects<Line<T>>,
    LineString<T>: BoundingRect<T>,
{
    fn intersects(&self, rhs: &Line<T>) -> bool {
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }
        self.lines().any(|l| l.intersects(&rhs))
    }
}

macro_rules! blanket_intersects_multilinestring {
    ($t:ty) => {
        impl<T> $crate::Intersects<$t> for MultiLineString<T>
        where
            T: CoordNum,
            LineString<T>: Intersects<$t>,
            $t: BoundingRect<T>,
        {
            fn intersects(&self, rhs: &$t) -> bool {
                if has_disjoint_bboxes(self, rhs) {
                return false;
            }
        self.iter().any(|p| p.intersects(rhs))
            }
        }
    };
}

blanket_intersects_multilinestring!(Coord<T>);
symmetric_intersects_impl!(MultiLineString<T>, LineString<T>);
blanket_intersects_multilinestring!(Line<T>);
blanket_intersects_multilinestring!(Point<T>);
blanket_intersects_multilinestring!(MultiPoint<T>);
blanket_intersects_multilinestring!(Polygon<T>);
blanket_intersects_multilinestring!(MultiPolygon<T>);
blanket_intersects_multilinestring!(Rect<T>);
blanket_intersects_multilinestring!(Triangle<T>);
blanket_intersects_multilinestring!(Geometry<T>);
blanket_intersects_multilinestring!(GeometryCollection<T>);

impl <T> Intersects<MultiLineString<T>> for MultiLineString<T>
where
    T: CoordNum,
    LineString<T>: Intersects<LineString<T>>,
    LineString<T>: BoundingRect<T>,
{
    fn intersects(&self, rhs: &MultiLineString<T>) -> bool {
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }
        self.iter().any(|l| rhs.iter().any(|other| l.intersects(&other)))
    }
}
