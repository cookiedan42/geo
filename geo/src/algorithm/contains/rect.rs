use geo_types::CoordFloat;
use crate::dimensions::Dimensions;

use super::{impl_contains_from_relate, impl_contains_geometry_for, Contains};
use crate::{geometry::*, Area, CoordsIter, HasDimensions, Intersects};
use crate::{CoordNum, GeoFloat};

// ┌──────────────────────────┐
// │ Implementations for Rect │
// └──────────────────────────┘

impl<T> Contains<Coord<T>> for Rect<T>
where
    T: CoordNum,
{
    fn contains(&self, coord: &Coord<T>) -> bool {
        coord.x > self.min().x
            && coord.x < self.max().x
            && coord.y > self.min().y
            && coord.y < self.max().y
    }
}

impl<T> Contains<Point<T>> for Rect<T>
where
    T: CoordNum,
{
    fn contains(&self, p: &Point<T>) -> bool {
        self.contains(&p.0)
    }
}

impl<T> Contains<Rect<T>> for Rect<T>
where
    T: CoordNum,
{
    fn contains(&self, other: &Rect<T>) -> bool {
        // TODO: check for degenerate rectangle (which is a line or a point)
        // All points of LineString must be in the polygon ?
        self.min().x <= other.min().x
            && self.max().x >= other.max().x
            && self.min().y <= other.min().y
            && self.max().y >= other.max().y
    }
}

impl<T> Contains<Polygon<T>> for Rect<T>
where
    T: CoordFloat,
    Self: Intersects<Coord<T>>,
    Line<T>: Contains<Polygon<T>>,
    Point<T>: Contains<Polygon<T>>,
{
    fn contains(&self, rhs: &Polygon<T>) -> bool {
        // the polygon must not be empty
        if rhs.is_empty() {
            return false;
        }

        match self.dimensions() {
            Dimensions::TwoDimensional => {
                rhs.exterior_coords_iter().all(|c| self.intersects(&c))
                && (rhs.exterior_coords_iter().any(|c| self.contains(&c)) ||!rhs.signed_area().is_zero())
            }
            Dimensions::OneDimensional => Line::new(self.min(), self.max()).contains(rhs),
            Dimensions::ZeroDimensional =>                 return Point::from(self.max()).contains(rhs),
            Dimensions::Empty => return false,

        }

    }
}

impl_contains_from_relate!(Rect<T>, [Line<T>, LineString<T>, MultiPoint<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Triangle<T>]);
impl_contains_geometry_for!(Rect<T>);


#[cfg(test)]
mod tests_polygon {
    use super::*;
    use crate::{polygon, Point, Relate};

    #[test]
    fn rect_contains_degenerate_polygon() {
        let rect = Rect::new(Point::new(0., 0.), Point::new(10., 5.));

        let poly = polygon![
            exterior: [(x: 0., y: 0.), (x: 0., y: 1.), (x: 0., y: 5.), (x: 0., y: 0.)],
            interiors: [],
        ];

        assert!(!rect.contains(&poly));
    }

    #[test]
    fn degenerate_rect_contains_degenerate_polygon() {
        let rect = Rect::new(Point::new(0., 0.), Point::new(10., 0.));
        let poly = polygon![
            exterior: [(x: 1., y: 0.), (x: 5., y: 0.)],
            interiors: [],
        ];

        let rect_ln = Line::new(Point::new(0., 0.), Point::new(10., 0.));
        let poly_ln = Line::new(Point::new(1., 0.), Point::new(5., 0.));
        assert!(rect_ln.contains(&poly_ln));
        assert!(rect.relate(&poly).is_contains());

        assert!(rect_ln.contains(&poly));
        // assert!(rect.contains(&poly));
    }
}