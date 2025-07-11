use super::{Covers, impl_covers_from_intersects, impl_covers_from_relate};
use crate::Kernel;
use crate::Orientation;
use crate::dimensions::Dimensions;
use crate::{CoordsIter, Intersects, geometry::*};
use crate::{GeoFloat, GeoNum};
use crate::{HasDimensions, LinesIter};

impl_covers_from_intersects!(coord: LineString<T>);

impl<T> Covers<Line<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, line: &Line<T>) -> bool {
        if self.is_empty() || line.is_empty() {
            return false;
        }

        if line.start == line.end {
            return self.covers(&line.start);
        }

        cover_lines_iter::<Self, T>(self, line)
    }
}

pub(crate) fn cover_lines_iter<'a, S, T>(s: &'a S, line: &Line<T>) -> bool
where
    T: GeoNum + 'a,
    S: LinesIter<'a, Scalar = T>,
{
    let is_vertical = line.start.x == line.end.x;

    // pre-order the line so that we can use the faster overlap check
    let line = if (is_vertical && line.start.y > line.end.y)
        || (!is_vertical && line.start.x > line.end.x)
    {
        Line::new(line.end, line.start)
    } else {
        *line
    };

    let candidates: Vec<(T, T)> = if is_vertical {
        s.lines_iter()
            .filter(|segment| overlap::y_overlap(&line, segment))
            .filter(|segment| is_collinear(&line, segment))
            .map(|segment| {
                if segment.start.y < segment.end.y {
                    (segment.start.y, segment.end.y)
                } else {
                    (segment.end.y, segment.start.y)
                }
            })
            .collect()
    } else {
        s.lines_iter()
            .filter(|segment| overlap::x_overlap(&line, segment))
            .filter(|segment| is_collinear(&line, segment))
            .map(|segment| {
                if segment.start.x < segment.end.x {
                    (segment.start.x, segment.end.x)
                } else {
                    (segment.end.x, segment.start.x)
                }
            })
            .collect()
    };

    let mut changed = true;

    // use y value instead if x values are identical
    let (mut line_start, mut line_end) = if is_vertical {
        (line.start.y, line.end.y)
    } else {
        (line.start.x, line.end.x)
    };

    // interval-based overlap checks
    while changed {
        changed = false;
        for (c_start, c_end) in candidates.iter() {
            // if no overlap, skip
            if *c_end <= line_start || line_end <= *c_start {
            }
            // if candidate covers line, return true
            else if *c_start <= line_start && line_end <= *c_end {
                return true;
            } else if *c_start <= line_start {
                // trim start
                changed = true;
                line_start = *c_end;
            } else if line_end <= *c_end {
                // trim end
                changed = true;
                line_end = *c_start;
            }
        }
    }

    false
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

impl<T> Covers<Rect<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Rect<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        match rhs.dimensions() {
            Dimensions::Empty => false,
            Dimensions::ZeroDimensional => {
                let Some(pt) = rhs.coords_iter().next() else {
                    return false;
                };
                self.covers(&pt)
            }
            Dimensions::OneDimensional => rhs.lines_iter().all(|l| self.covers(&l)),
            Dimensions::TwoDimensional => false,
        }
    }
}

impl<T> Covers<Triangle<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Triangle<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        match rhs.dimensions() {
            Dimensions::Empty => false,
            Dimensions::ZeroDimensional => {
                let Some(pt) = rhs.coords_iter().next() else {
                    return false;
                };
                self.covers(&pt)
            }
            Dimensions::OneDimensional => rhs.lines_iter().all(|l| self.covers(&l)),
            Dimensions::TwoDimensional => false,
        }
    }
}

// all lines in the exteriors of the polygon must be covered
// and no area / dim = 1
impl<T> Covers<Polygon<T>> for LineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Polygon<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        if rhs.dimensions() >= Dimensions::OneDimensional {
            // if polygon is 2d or 1d, always false
            return false;
        }
        // for degenerate as pt
        rhs.lines_iter().all(|l| self.covers(&l))
    }
}

impl_covers_from_intersects!(LineString<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(LineString<T>, [MultiPolygon<T>]);
impl_covers_from_relate!(LineString<T>, [GeometryCollection<T>]);

//
// MultiLineString Implementations
//

impl<T> Covers<Coord<T>> for MultiLineString<T>
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

impl_covers_from_intersects!(MultiLineString<T>, [Point<T>, MultiPoint<T>]);

impl<T> Covers<Line<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, line: &Line<T>) -> bool {
        if self.is_empty() || line.is_empty() {
            return false;
        }

        if line.start == line.end {
            return self.covers(&line.start);
        }

        cover_lines_iter::<Self, T>(self, line)
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

impl<T> Covers<Rect<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Rect<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        match rhs.dimensions() {
            Dimensions::Empty => false,
            Dimensions::ZeroDimensional => {
                let Some(pt) = rhs.coords_iter().next() else {
                    return false;
                };
                self.covers(&pt)
            }
            Dimensions::OneDimensional => rhs.lines_iter().all(|l| self.covers(&l)),
            Dimensions::TwoDimensional => false,
        }
    }
}

impl<T> Covers<Triangle<T>> for MultiLineString<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Triangle<T>) -> bool {
        if self.is_empty() {
            return false;
        }
        match rhs.dimensions() {
            Dimensions::Empty => false,
            Dimensions::ZeroDimensional => {
                let Some(pt) = rhs.coords_iter().next() else {
                    return false;
                };
                self.covers(&pt)
            }
            Dimensions::OneDimensional => rhs.lines_iter().all(|l| self.covers(&l)),
            Dimensions::TwoDimensional => false,
        }
    }
}
impl_covers_from_relate!(MultiLineString<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(MultiLineString<T>, [GeometryCollection<T>]);

#[inline]
fn is_collinear<T>(l1: &Line<T>, l2: &Line<T>) -> bool
where
    T: GeoNum,
{
    T::Ker::orient2d(l1.start, l1.end, l2.start) == Orientation::Collinear
        && T::Ker::orient2d(l1.start, l1.end, l2.end) == Orientation::Collinear
}

/// Suppose we have 2 pairs (p1,p2) and (q1,q2) where p1 < p2 and q1 < q2
///
/// It is sufficient to show that each lower bound is smaller than the others' upper bound for the ranges to overlap  
mod overlap {
    use super::*;

    #[inline]
    /// Since l1 is ordered, we can execute overlap check in 3 comparisons.  
    /// We use exclusive bounds because we only want to keep segments which can trim the line
    pub(super) fn x_overlap<T: GeoNum>(ordered_l1: &Line<T>, l2: &Line<T>) -> bool {
        debug_assert!(ordered_l1.start.x <= ordered_l1.end.x);

        let (p1, p2) = (ordered_l1.start.x, ordered_l1.end.x);
        let (q1, q2) = if l2.start.x < l2.end.x {
            (l2.start.x, l2.end.x)
        } else {
            (l2.end.x, l2.start.x)
        };

        p1 < q2 && q1 < p2
    }

    #[inline]
    /// Since l1 is ordered, we can execute overlap check in 3 comparisons.  
    /// We use exclusive bounds because we only want to keep segments which can trim the line
    pub(super) fn y_overlap<T: GeoNum>(ordered_l1: &Line<T>, l2: &Line<T>) -> bool {
        debug_assert!(ordered_l1.start.y <= ordered_l1.end.y);

        let (p1, p2) = (ordered_l1.start.y, ordered_l1.end.y);
        let (q1, q2) = if l2.start.y < l2.end.y {
            (l2.start.y, l2.end.y)
        } else {
            (l2.end.y, l2.start.y)
        };

        p1 < q2 && q1 < p2
    }
}

#[cfg(test)]
mod test {
    use crate::{Convert, wkt};

    use super::*;
    use crate::{Polygon, Relate};

    #[test]
    fn degenerate_polygon() {
        // degenerate polygons which topologically might reasonably be covered by a linestring
        let pt: Polygon<f64> = wkt! { POLYGON((0 0, 0 0)) }.convert();
        let ln: Polygon<f64> = wkt! { POLYGON((0 0, 1 0,0 0)) }.convert();
        let ls: Polygon<f64> = wkt! { POLYGON((0 0, 1 0, 2 0, 0 0)) }.convert();

        let base: LineString<f64> = wkt! { LINESTRING(0 0, 4 0) }.convert();

        assert_eq!(base.relate(&pt).is_covers(), base.covers(&pt)); // true
        assert_eq!(base.relate(&ln).is_covers(), base.covers(&ln)); // false
        assert_eq!(base.relate(&ls).is_covers(), base.covers(&ls)); // false
    }
}
