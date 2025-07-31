use super::{impl_covers_from_intersects, impl_covers_from_relate, Covers};
use crate::{geometry::*, Contains, CoordsIter, Intersects};
use crate::{GeoFloat, GeoNum};
use crate::{HasDimensions, LinesIter};

impl<T> Covers<Coord<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        self.intersects(rhs)
    }
}

impl_covers_from_intersects!(LineString<T>, [Point<T>, MultiPoint<T>]);

impl<T> Covers<Line<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Line<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        if rhs.start == rhs.end {
            // LineString contains Point is different from LineString covers Point
            // so use covers implementation
            return self.covers(&rhs.start);
        }
        // LineString contains Line is equivalent to LineString covers Line
        // so use the existing implementation
        self.contains(rhs)
    }
}

impl<T> Covers<LineString<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &LineString<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.lines_iter().all(|l| self.covers(&l))
    }
}

impl<T> Covers<MultiLineString<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &MultiLineString<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.lines_iter().all(|l| self.covers(&l))
    }
}

impl_covers_from_relate!(LineString<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_relate!(LineString<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(LineString<T>, [GeometryCollection<T>]);

impl<T> Covers<Coord<T>> for MultiLineString<T>
where
    T: GeoNum,
    Self: Covers<Point<T>>,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        self.intersects(rhs)
    }
}

// MultiLineString Implementations

impl_covers_from_intersects!(MultiLineString<T>, [Point<T>, MultiPoint<T>]);

impl<T> Covers<Line<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Line<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        if rhs.start == rhs.end {
            // LineString contains Point is different from LineString covers Point
            // so use covers implementation
            return self.covers(&rhs.start);
        }
        // We copy the line as we may truncate the line as
        // we find partial matches.
        let mut line = *rhs;
        let mut first_cut = None;

        let lines_iter = self.lines_iter();
        let num_lines = lines_iter.count();

        // We need to repeat the logic twice to handle cases
        // where the linestring starts at the middle of the line.
        for (i, segment) in self.lines_iter().chain(self.lines_iter()).enumerate() {
            if i >= num_lines {
                // The first loop was done. If we never cut
                // the line, or at the cut segment again, we
                // can exit now.
                if let Some(upto_i) = first_cut {
                    if i >= num_lines + upto_i {
                        break;
                    }
                } else {
                    break;
                }
            }
            // Look for a segment that intersects at least
            // one of the end points.
            let other = if segment.intersects(&line.start) {
                line.end
            } else if segment.intersects(&line.end) {
                line.start
            } else {
                continue;
            };

            // If the other end point also intersects this
            // segment, then we are done.
            let new_inside = if segment.intersects(&other) {
                return true;
            }
            // otoh, if the line contains one of the ends of
            // the segments, then we truncate the line to
            // the part outside.
            else if line.contains(&segment.start) {
                segment.start
            } else if line.contains(&segment.end) {
                segment.end
            } else {
                continue;
            };

            first_cut = first_cut.or(Some(i));
            if other == line.start {
                line.end = new_inside;
            } else {
                line.start = new_inside;
            }
        }

        false
    }
}

impl<T> Covers<LineString<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &LineString<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.lines_iter().all(|l| self.covers(&l))
    }
}

impl<T> Covers<MultiLineString<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &MultiLineString<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.lines_iter().all(|l| self.covers(&l))
    }
}

// polygon types can only be true iff they are 1d ~ linestring/ multilinestring  

impl_covers_from_relate!(MultiLineString<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_relate!(MultiLineString<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(MultiLineString<T>, [GeometryCollection<T>]);
