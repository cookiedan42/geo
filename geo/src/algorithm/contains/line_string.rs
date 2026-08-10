use super::{Contains, impl_contains_from_relate, impl_contains_geometry_for};
use crate::algorithm::covers::line_string::cover_lines_iter;
use crate::geometry::*;
use crate::{CoordNum, GeoFloat, GeoNum, HasDimensions, Intersects, Orientation};

// ┌────────────────────────────────┐
// │ Implementations for LineString │
// └────────────────────────────────┘

impl<T> Contains<Coord<T>> for LineString<T>
where
    T: GeoNum,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        if self.0.is_empty() {
            return false;
        }

        if coord == &self.0[0] || coord == self.0.last().unwrap() {
            return self.is_closed();
        }

        // since it is already known that coord != linestring start or end
        // it is sufficient to check if the coord intersects any line,
        self.lines().any(|ln| ln.intersects(coord))
    }
}

impl<T> Contains<Point<T>> for LineString<T>
where
    T: GeoNum,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

impl<T> Contains<Line<T>> for LineString<T>
where
    T: GeoNum,
{
    fn contains(&self, line: &Line<T>) -> bool {
        if line.start == line.end {
            return self.contains(&line.start);
        }

        cover_lines_iter::<Self, T>(self, line)
    }
}

impl<T> Contains<LineString<T>> for LineString<T>
where
    T: GeoNum,
{
    fn contains(&self, rhs: &LineString<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }

        // handle degenerate linestring case
        if rhs.dimensions() == Dimensions::ZeroDimensional {
            return self.contains(&rhs.0[0]);
        }

        // filter out zero-length segments
        // it is known from != Dimensions::ZeroDimensional && !self.is_empty()
        // that there must be at least one segment with non-zero length
        rhs.lines()
            .filter(|l| l.start != l.end)
            .all(|l| self.contains(&l))
    }
}

impl_contains_from_relate!(LineString<T>, [Polygon<T>, MultiPoint<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Rect<T>, Triangle<T>]);
impl_contains_geometry_for!(LineString<T>);

// ┌─────────────────────────────────────┐
// │ Implementations for MultiLineString │
// └─────────────────────────────────────┘

impl_contains_from_relate!(MultiLineString<T>, [Line<T>, LineString<T>, Polygon<T>, MultiPoint<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Rect<T>, Triangle<T>]);
impl_contains_geometry_for!(MultiLineString<T>);

impl<T> Contains<Coord<T>> for MultiLineString<T>
where
    T: CoordNum,
    LineString<T>: Contains<Coord<T>>,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        self.iter().any(|ls| ls.contains(coord))
    }
}

impl<T> Contains<Point<T>> for MultiLineString<T>
where
    T: CoordNum,
    LineString<T>: Contains<Point<T>>,
{
    fn contains(&self, rhs: &Point<T>) -> bool {
        self.iter().any(|ls| ls.contains(rhs))
    }
}

#[cfg(test)]
mod test {
    use crate::{Contains, Relate};
    use crate::{Convert, wkt};
    use crate::{Line, LineString, Validation};

    #[test]
    fn linestring_component_with_zero_length() {
        let ls: LineString<f64> = wkt! {LINESTRING(0 0, 2 2)}.convert();

        // these are valid geometries
        let ls_start: LineString<f64> = wkt! {LINESTRING(0 0, 0 0,0 0, 1 1)}.convert();
        let ls_end: LineString<f64> = wkt! {LINESTRING(0 0, 1 1,1 1,1 1)}.convert();
        assert!(ls_start.is_valid());
        assert!(ls_end.is_valid());

        // these are invalid geometries but we handle degenerate geometries as points
        let degen_start: LineString<f64> = wkt! {LINESTRING(0 0, 0 0)}.convert();
        let degen_end: LineString<f64> = wkt! {LINESTRING(2 2, 2 2)}.convert();
        assert!(!degen_start.is_valid());
        assert!(!degen_end.is_valid());

        assert!(ls.relate(&ls_start).is_contains());
        assert!(ls.contains(&ls_start));
        assert!(ls.relate(&ls_end).is_contains());
        assert!(ls.contains(&ls_end));

        assert!(!ls.contains(&degen_start));
        assert!(!ls.relate(&degen_start).is_contains());
        assert!(!ls.contains(&degen_end));
        assert!(!ls.relate(&degen_end).is_contains());
    }

    #[test]
    fn triangles() {
        let ln: Line<f64> = wkt! {LINE(0 0, 10 0)}.convert();
        let ls: LineString<f64> = wkt! {LINESTRING(0 0, 1 1, 2 0, 4 0, 5 1, 6 0, 8 0, 9 1,10 0, 8 0, 7 -1, 6 0, 4 0, 3 -1, 2 0 , 0 0 )}.convert();

        // ln and ls are valid
        assert!(ln.is_valid());
        assert!(ls.is_valid());

        assert_eq!(
            ls.relate(&ln).is_contains(), // true
            ls.contains(&ln)              // true
        );
    }

    #[test]
    fn test_start_end() {
        let ls: LineString<f64> = wkt! {LINESTRING(0 0,0 1, 1 1)}.convert();
        let ln_start: Line<f64> = wkt! {LINE(0 0, 0 0)}.convert();
        let ln_end: Line<f64> = wkt! {LINE(1 1, 1 1)}.convert();

        assert!(!ls.contains(&ln_start));
        assert!(!ls.contains(&ln_end));
    }

    #[test]
    fn test_vertical() {
        let ls1: LineString<f64> = wkt! {LINESTRING(0 0,0 5,0 10)}.convert();
        let ls2: LineString<f64> = wkt! {LINESTRING(0 10,0 5, 0 0)}.convert();

        let ln: Line<f64> = wkt! {LINE(0 0, 0 9)}.convert();

        assert!(ls1.contains(&ln));
        assert!(ls2.contains(&ln));
    }
}
