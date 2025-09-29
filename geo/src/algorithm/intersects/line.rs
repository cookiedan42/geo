use super::{Intersects, point_in_rect};
use crate::*;

impl<T> Intersects<Coord<T>> for Line<T>
where
    T: GeoNum,
{
    fn intersects(&self, rhs: &Coord<T>) -> bool {
        // First we check if the point is collinear with the line.
        T::Ker::orient2d(self.start, self.end, *rhs) == Orientation::Collinear
        // In addition, the point must have _both_ coordinates
        // within the start and end bounds.
            && point_in_rect(*rhs, self.start, self.end)
    }
}

symmetric_intersects_impl!(Line<T>, LineString<T>);
symmetric_intersects_impl!(Line<T>, MultiLineString<T>);

impl<T> Intersects<Line<T>> for Line<T>
where
    T: GeoNum,
{
    fn intersects(&self, line: &Line<T>) -> bool {
        // Special case: self is equiv. to a point.
        if self.start == self.end {
            return line.intersects(&self.start);
        }

        // Precondition: start and end are distinct.

        // Check if orientation of rhs.{start,end} are different
        // with respect to self.{start,end}.
        let check_1_1 = T::Ker::orient2d(self.start, self.end, line.start);
        let check_1_2 = T::Ker::orient2d(self.start, self.end, line.end);

        if check_1_1 != check_1_2 {
            // Since the checks are different,
            // rhs.{start,end} are distinct, and rhs is not
            // collinear with self. Thus, there is exactly
            // one point on the infinite extensions of rhs,
            // that is collinear with self.

            // By continuity, this point is not on the
            // exterior of rhs. Now, check the same with
            // self, rhs swapped.

            let check_2_1 = T::Ker::orient2d(line.start, line.end, self.start);
            let check_2_2 = T::Ker::orient2d(line.start, line.end, self.end);

            // By similar argument, there is (exactly) one
            // point on self, collinear with rhs. Thus,
            // those two have to be same, and lies (interior
            // or boundary, but not exterior) on both lines.
            check_2_1 != check_2_2
        } else if check_1_1 == Orientation::Collinear {
            // Special case: collinear line segments.

            // Equivalent to 4 point-line intersection
            // checks, but removes the calls to the kernel
            // predicates.

           if line.start.x == line.end.x {
                // vertical line, we should compare y value
                let (p1, p2) = if line.start.y < line.end.y {
                    (&line.start.y, &line.end.y)
                } else {
                    (&line.end.y, &line.start.y)
                };
                let (q1, q2) = if self.start.y < self.end.y {
                    (&self.start.y, &self.end.y)
                } else {
                    (&self.end.y, &self.start.y)
                };
                p1 <= q2 && q1 <= p2
            } else {
                // non-vertical line, valid to compare x value
                let (p1, p2) = if line.start.x < line.end.x {
                    (&line.start.x, &line.end.x)
                } else {
                    (&line.end.x, &line.start.x)
                };
                let (q1, q2) = if self.start.x < self.end.x {
                    (&self.start.x, &self.end.x)
                } else {
                    (&self.end.x, &self.start.x)
                };
                p1 <= q2 && q1 <= p2
            }
        } else {
        false
        }
    }
}

symmetric_intersects_impl!(Line<T>, Point<T>);
symmetric_intersects_impl!(Line<T>, MultiPoint<T>);

symmetric_intersects_impl!(Line<T>, Polygon<T>);
symmetric_intersects_impl!(Line<T>, MultiPolygon<T>);

symmetric_intersects_impl!(Line<T>, Rect<T>);

impl<T> Intersects<Triangle<T>> for Line<T>
where
    T: GeoNum,
{
    fn intersects(&self, rhs: &Triangle<T>) -> bool {
        self.intersects(&rhs.to_polygon())
    }
}
