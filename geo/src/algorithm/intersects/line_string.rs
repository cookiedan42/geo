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
blanket_intersects_linestring!(LineString<T>);
blanket_intersects_linestring!(MultiLineString<T>);
blanket_intersects_linestring!(Line<T>);
blanket_intersects_linestring!(Point<T>);
blanket_intersects_linestring!(MultiPoint<T>);
blanket_intersects_linestring!(Polygon<T>);
blanket_intersects_linestring!(MultiPolygon<T>);
blanket_intersects_linestring!(Rect<T>);
blanket_intersects_linestring!(Triangle<T>);
blanket_intersects_linestring!(Geometry<T>);
blanket_intersects_linestring!(GeometryCollection<T>);


// Blanket implementation from LineString<T>
impl<T, G> Intersects<G> for MultiLineString<T>
where
    T: CoordNum,
    LineString<T>: Intersects<G>,
    G: BoundingRect<T>,
{
    fn intersects(&self, rhs: &G) -> bool {
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }
        self.iter().any(|p| p.intersects(rhs))
    }
}
