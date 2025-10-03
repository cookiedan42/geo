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
        if self
            .lines_iter()
            .any(|self_l| rhs.lines_iter().any(|rhs_l| self_l.intersects(&rhs_l)))
        {
            return false;
        }
        // all rings are concentric or disjoint

        // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior
        let Some(rhs_ext_coord) = rhs.exterior().0.first() else {
            return false;
        };

        if coord_pos_relative_to_ring(*rhs_ext_coord, self.exterior()) != CoordPos::Inside {
            return false;
        }

        // all holes of self must be covered by some hole of rhs
        // if any hole of self is uncovered by a hole of rhs, then there exists a point of rhs which does not lie in self
        // and hence return false
        // since we know all rings are concentric or disjoint, we can just check the first point of each hole

        for self_hole in self.interiors() {
            // if self_hole is empty, then it is covered by rhs
            let Some(self_hole_first_coord) = self_hole.0.first() else {
                continue;
            };
            if rhs.interiors().iter().any(|rhs_hole| {
                coord_pos_relative_to_ring(*self_hole_first_coord, rhs_hole) != CoordPos::Inside
            }) {
                return false;
            }
        }

        return true;
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

        // no boundary intersection
        if self
            .lines_iter()
            .any(|self_l| rhs.lines_iter().any(|rhs_l| self_l.intersects(&rhs_l)))
        {
            return false;
        }
        // all rings are concentric or disjoint

        // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior
        let Some(rhs_ext_coord) = rhs.exterior().0.first() else {
            return false;
        };
        // since no boundary intersection, rhs must be constrained within one of the polygons or totally disjoint
        if self.0.iter().map(|poly|poly.exterior()).any(|ls|coord_pos_relative_to_ring(*rhs_ext_coord, ls) != CoordPos::Inside) {
            return false;
        }


        // all holes of self must be covered by some hole of rhs
        // if any hole of self is uncovered by a hole of rhs, then there exists a point of rhs which does not lie in self
        // and hence return false
        // since we know all rings are concentric or disjoint, we can just check the first point of each hole

        for poly in self.0.iter() {
            if coord_pos_relative_to_ring(*rhs_ext_coord, poly.exterior()) != CoordPos::Inside {
                continue;
            }

            for self_hole in poly.interiors() {
                // if self_hole is empty, then it is covered by rhs
                let Some(self_hole_first_coord) = self_hole.0.first() else {
                    continue;
                };
                if rhs.interiors().iter().any(|rhs_hole| {
                    coord_pos_relative_to_ring(*self_hole_first_coord, rhs_hole) != CoordPos::Inside
                }) {
                    return false;
                }
            }
        }
        return true;
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

        // no boundary intersection
        if self
            .lines_iter()
            .any(|self_l| rhs.lines_iter().any(|rhs_l| self_l.intersects(&rhs_l)))
        {
            return false;
        }
        // all rings are concentric or disjoint


        for rhs_poly in rhs.0.iter() {
             // if any point of rhs exterior lies within self.exterior, then all points of rhs exterior lie within self.exterior
        let Some(rhs_ext_coord) = rhs_poly.exterior().0.first() else {
            return false;
        };
        // since no boundary intersection, rhs must be constrained within one of the polygons or totally disjoint
        if self.0.iter().map(|poly|poly.exterior()).any(|ls|coord_pos_relative_to_ring(*rhs_ext_coord, ls) != CoordPos::Inside) {
            return false;
        }


        // all holes of self must be covered by some hole of rhs
        // if any hole of self is uncovered by a hole of rhs, then there exists a point of rhs which does not lie in self
        // and hence return false
        // since we know all rings are concentric or disjoint, we can just check the first point of each hole

        for poly in self.0.iter() {
            if coord_pos_relative_to_ring(*rhs_ext_coord, poly.exterior()) != CoordPos::Inside {
                continue;
            }

            for self_hole in poly.interiors() {
                // if self_hole is empty, then it is covered by rhs
                let Some(self_hole_first_coord) = self_hole.0.first() else {
                    continue;
                };
                if rhs_poly.interiors().iter().any(|rhs_hole| {
                    coord_pos_relative_to_ring(*self_hole_first_coord, rhs_hole) != CoordPos::Inside
                }) {
                    return false;
                }
            }
        }
        }
        true


    }
}

impl_contains_properly_from_relate!(MultiPolygon<T>, [
Point<T>,MultiPoint<T>,
Line<T>, LineString<T>, MultiLineString<T>,
GeometryCollection<T>,
Rect<T>,Triangle<T>
]);

#[cfg(test)]
mod tests {
    use crate::*;
    use wkt::ToWkt;

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
}
