use super::{ContainsProperly, impl_contains_properly_from_relate};
use crate::CoordsIter;
use crate::HasDimensions;
use crate::Intersects;
use crate::LinesIter;
use crate::coordinate_position::{CoordPos, coord_pos_relative_to_ring};
use crate::geometry::*;
use crate::{GeoFloat, GeoNum};

impl<T> ContainsProperly<Coord<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Coord<T>) -> bool {
        use crate::coordinate_position::{CoordPos, CoordinatePosition};
        self.coordinate_position(rhs) == CoordPos::Inside
    }
}

impl<T> ContainsProperly<Point<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Point<T>) -> bool {
        self.contains_properly(&rhs.0)
    }
}

impl<T> ContainsProperly<MultiPoint<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &MultiPoint<T>) -> bool {
        rhs.coords_iter().all(|p| self.contains_properly(&p))
    }
}

impl<T> ContainsProperly<Line<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Line<T>) -> bool {
        (!self.lines_iter().any(|l| l.intersects(rhs))) && self.contains_properly(&rhs.start)
    }
}

impl<T> ContainsProperly<LineString<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &LineString<T>) -> bool {
        !self.is_empty()
            && !rhs.is_empty()
            && (!self.lines_iter().any(|l| l.intersects(rhs)))
            && self.contains_properly(&rhs.0[0])
    }
}

impl<T> ContainsProperly<MultiLineString<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &MultiLineString<T>) -> bool {
        !self.is_empty()
            && !rhs.is_empty()
            && (!self.lines_iter().any(|l| l.intersects(rhs)))
            && rhs
                .0
                .iter()
                .all(|ls| self.contains_properly(ls.0.first().unwrap()))
    }
}

impl<T> ContainsProperly<Polygon<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Polygon<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }

        // no boundary intersection
        if boundary_intersects::<T, Polygon<T>, Polygon<T>>(self, rhs) {
            return false;
        }
        // all rings are concentric or disjoint

        // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior
        let Some(rhs_ext_coord) = rhs.exterior().0.first() else {
            return false;
        };

        if !coord_in_ring(*rhs_ext_coord, self.exterior()) {
            return false;
        }

        // every self_hole must be in a rhs_hole
        for self_hole in self.interiors() {
            // if self_hole is empty, then it is hole covered by rhs
            let Some(self_hole_first_coord) = self_hole.0.first() else {
                continue;
            };
            if !coord_in_any_ring(*self_hole_first_coord, rhs.interiors().iter()) {
                return false;
            }
        }

        true
    }
}

impl<T> ContainsProperly<MultiPolygon<T>> for Polygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &MultiPolygon<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        rhs.iter().all(|poly| self.contains_properly(poly))
    }
}

impl_contains_properly_from_relate!(Polygon<T>, [
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);

impl<T> ContainsProperly<Polygon<T>> for MultiPolygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &Polygon<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }
        if boundary_intersects::<T, MultiPolygon<T>, Polygon<T>>(self, rhs) {
            return false;
        }
        // all rings are concentric or disjoint

        // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior
        let Some(rhs_ext_coord) = rhs.exterior().0.first() else {
            return false;
        };

        let self_ext_rings = self.0.iter().map(|poly| poly.exterior());
        if !coord_in_any_ring(*rhs_ext_coord, self_ext_rings) {
            return false;
        }

        // all holes of self must be covered by some hole of rhs
        // if any hole of self is uncovered by a hole of rhs, then there exists a point of rhs which does not lie in self
        // and hence return false
        // since we know all rings are concentric or disjoint, we can just check the first point of each hole

        // check for disjoint
        let mut is_disjoint = true;

        let candidates = self
            .0
            .iter()
            .filter(|poly| poly.contains_properly(rhs_ext_coord));

        // there should be exactly one candidate

        for self_poly in candidates {
            is_disjoint = false;

            for self_hole in self_poly.interiors() {
                // if self_hole is empty, then it is covered by rhs
                let Some(self_hole_first_coord) = self_hole.0.first() else {
                    continue;
                };
                if !coord_in_any_ring(*self_hole_first_coord, rhs.interiors().iter()) {
                    return false;
                }
            }
        }

        !is_disjoint
    }
}
impl<T> ContainsProperly<MultiPolygon<T>> for MultiPolygon<T>
where
    T: GeoNum,
{
    fn contains_properly(&self, rhs: &MultiPolygon<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }

        if boundary_intersects::<T, MultiPolygon<T>, MultiPolygon<T>>(self, rhs) {
            return false;
        }

        // all rings are concentric or disjoint
        // if both are valid, then each rhs_poly will have 0..1 candidates
        // and each candidate will have 0..n rhs_poly

        for rhs_poly in rhs.0.iter() {
            // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior

            let Some(rhs_ext_coord) = rhs_poly.exterior().0.first() else {
                return false;
            };
            // since no boundary intersection, rhs must be constrained within one of the polygons or totally disjoint
            let mut is_disjoint = true;
            let candidates = self
                .0
                .iter()
                .filter(|poly| poly.contains_properly(rhs_ext_coord));

            for self_poly in candidates {
                is_disjoint = false;
                for self_hole in self_poly.interiors() {
                    // if self_hole is empty, then it is covered by rhs
                    let Some(self_hole_first_coord) = self_hole.0.first() else {
                        continue;
                    };
                    if !coord_in_any_ring(*self_hole_first_coord, rhs_poly.interiors().iter()) {
                        return false;
                    }
                }
            }
            if is_disjoint {
                return false;
            }
        }
        true
    }
}
/// Return true if the coord lies `Inside` any of the rings
fn coord_in_any_ring<'a, T, I>(coord: Coord<T>, rings: I) -> bool
where
    T: GeoNum + 'a,
    I: Iterator<Item = &'a LineString<T>>,
{
    rings
        .map(|ring| coord_pos_relative_to_ring(coord, ring))
        .any(|pos| pos == CoordPos::Inside)
}

/// Return true if the boundary of lhs intersects any of the boundaries of rhs
/// where lhs and rhs are both polygons/multipolygons
fn boundary_intersects<'a, T, G1, G2>(lhs: &'a G1, rhs: &'a G2) -> bool
where
    T: GeoNum,
    G1: LinesIter<'a, Scalar = T>,
    G2: LinesIter<'a, Scalar = T>,
    Line<T>: Intersects<Line<T>>,
{
    lhs.lines_iter()
        .flat_map(|self_l| rhs.lines_iter().map(move |rhs_l| (self_l, rhs_l)))
        .any(|(self_l, rhs_l)| self_l.intersects(&rhs_l))
}

fn coord_in_ring<T>(coord: Coord<T>, ring: &LineString<T>) -> bool
where
    T: GeoNum,
{
    coord_pos_relative_to_ring(coord, ring) == CoordPos::Inside
}

impl_contains_properly_from_relate!(MultiPolygon<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Convert;
    use crate::wkt;
    use crate::{MultiPolygon, Polygon};

    #[test]
    fn test_contains_properly_donut() {
        let poly1: Polygon<f64> =
            wkt! {POLYGON((9 0,9 9,0 9,0 0,9 0),(6 3,6 6,3 6,3 3,6 3))}.convert();
        let poly2: Polygon<f64> =
            wkt! {POLYGON((8 1,8 8,1 8,1 1,8 1),(7 2,7 7,2 7,2 2,7 2))}.convert();

        assert!(poly1.contains_properly(&poly2));
    }

    #[test]
    fn test_contains_properly_donut_multi_multi() {
        let poly1: MultiPolygon<f64> =
            wkt! {MULTIPOLYGON(((9 0,9 9,0 9,0 0,9 0),(6 3,6 6,3 6,3 3,6 3)))}.convert();
        let poly2: MultiPolygon<f64> =
            wkt! {MULTIPOLYGON(((8 1,8 8,1 8,1 1,8 1),(7 2,7 7,2 7,2 2,7 2)))}.convert();

        assert!(poly1.contains_properly(&poly2));
    }

    #[test]
    fn test_contains_properly_donut_multi_poly() {
        let mp: MultiPolygon<f64> = wkt!{MULTIPOLYGON(((9 0,9 9,0 9,0 0,9 0),(8 1,8 8,1 8,1 1,8 1)),((7 2,7 7,2 7,2 2,7 2)))}.convert();
        let poly2: Polygon<f64> = wkt! {POLYGON((6 3,6 6,3 6,3 3,6 3))}.convert();

        assert!(mp.contains_properly(&poly2));
    }
}
