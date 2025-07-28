use super::{has_disjoint_bboxes, Intersects};
use crate::*;

impl<T> Intersects<Coord<T>> for Rect<T>
where
    T: CoordNum,
{
    fn intersects(&self, rhs: &Coord<T>) -> bool {
        rhs.x >= self.min().x
            && rhs.y >= self.min().y
            && rhs.x <= self.max().x
            && rhs.y <= self.max().y
    }
}

symmetric_intersects_impl!(Rect<T>, LineString<T>);
symmetric_intersects_impl!(Rect<T>, MultiLineString<T>);

// Same logic as Polygon<T>: Intersects<Line<T>>, but avoid
// an allocation.
impl<T> Intersects<Line<T>> for Rect<T>
where
    T: GeoNum,
{
    fn intersects(&self, rhs: &Line<T>) -> bool {
        let lb = self.min();
        let rt = self.max();
        let lt = Coord::from((lb.x, rt.y));
        let rb = Coord::from((rt.x, lb.y));
        // If either rhs.{start,end} lies inside Rect, then true
        self.intersects(&rhs.start)
            || self.intersects(&rhs.end)
            || Line::new(lt, rt).intersects(rhs)
            || Line::new(rt, rb).intersects(rhs)
            || Line::new(lb, rb).intersects(rhs)
            || Line::new(lt, lb).intersects(rhs)
    }
}

symmetric_intersects_impl!(Rect<T>, Point<T>);
symmetric_intersects_impl!(Rect<T>, MultiPoint<T>);

impl<T> Intersects<Polygon<T>> for Rect<T>
where
    T: GeoNum,
{
    fn intersects(&self, rhs: &Polygon<T>) -> bool {
        // simplified logic based on Polygon intersects Polygon

        if has_disjoint_bboxes(self, rhs) {
            return false;
        }

        // if any of the polygon's corners intersect the rectangle
        rhs.coords_iter().take(1).any(|p| self.intersects(&p))

        // or any of the polygon's lines intersect the rectangle's lines
        || rhs.lines_iter().any(|rhs_line| {
            self.lines_iter()
                .any(|self_line| self_line.intersects(&rhs_line))
        })

        // or any point of the rectangle intersects the polygon
        // pt.intersects(polygon) is the most expensive, so check it last
        // required only if rectangle sits fully inside polygon or both are disjoint
        // therefore only need to check one point
        || self.min().intersects(rhs)
    }
}

symmetric_intersects_impl!(Rect<T>, MultiPolygon<T>);

impl<T> Intersects<Rect<T>> for Rect<T>
where
    T: CoordNum,
{
    fn intersects(&self, other: &Rect<T>) -> bool {
        if self.max().x < other.min().x {
            return false;
        }

        if self.max().y < other.min().y {
            return false;
        }

        if self.min().x > other.max().x {
            return false;
        }

        if self.min().y > other.max().y {
            return false;
        }

        true
    }
}

impl<T> Intersects<Triangle<T>> for Rect<T>
where
    T: GeoNum,
    {
        
    fn intersects(&self, rhs: &Triangle<T>) -> bool {
        // simplified logic based on Polygon intersects Polygon
        
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }

        // if any of the triangle's corners intersect the rectangle
        self.intersects(&rhs.0)

        // or any of the triangle's lines intersect the rectangle's lines
        || rhs.lines_iter().any(|rhs_line| {
            self.lines_iter()
                .any(|self_line| self_line.intersects(&rhs_line))
        })

        // or some corner of the triangle intersects the rectangle
        || self.min().intersects(rhs)
    }
}
