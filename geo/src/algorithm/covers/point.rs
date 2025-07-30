use super::{impl_covers_from_relate, Covers};
use crate::covers::impl_covers_from_intersects;
use crate::dimensions::{Dimensions, HasDimensions};
use crate::geometry::*;
use crate::utils::lex_cmp;
use crate::{CoordsIter, Intersects};
use crate::{GeoFloat, GeoNum};

/*
    If self is a sngle point
    and all points of other intersect self,
    then self covers other.
*/

impl<T, G> Covers<G> for Coord<T>
where
    T: GeoFloat,
    Point<T>: Covers<G>,
{
    fn covers(&self, rhs: &G) -> bool {
        Point::new(self.x, self.y).covers(rhs)
    }
}

impl<T> Covers<Coord<T>> for Point<T>
where
    T: GeoFloat,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl_covers_from_intersects!(Point<T>, [Point<T>, MultiPoint<T>]);

impl_covers_from_intersects!(Point<T>, [Line<T>, LineString<T>, MultiLineString<T>]);
impl_covers_from_intersects!(Point<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Point<T>, [Polygon<T>, MultiPolygon<T>]);
impl_covers_from_intersects!(Point<T>, [Geometry<T>, GeometryCollection<T>]);

/*
    If self is a multi point
    and all parts of other are covered by some part of self,
    then self covers other.
*/

impl<T> Covers<Coord<T>> for MultiPoint<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl<T> Covers<MultiPoint<T>> for MultiPoint<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &MultiPoint<T>) -> bool {
        if self.is_empty() || rhs.is_empty() {
            return false;
        }

        let self_order = {
            let mut s = self.coords_iter().collect::<Vec<_>>();
            s.sort_by(lex_cmp);
            s
        };
        let other_order = {
            let mut s = rhs.coords_iter().collect::<Vec<_>>();
            s.sort_by(lex_cmp);
            s
        };

        let mut self_iter = self_order.iter().peekable();
        let mut other_iter = other_order.iter().peekable();

        loop {
            // other has been exhausted
            if other_iter.peek().is_none() {
                return true;
            }
            // self has been exhausted but other has not been exhausted
            if self_iter.peek().is_none() {
                return false;
            }

            match lex_cmp(self_iter.peek().unwrap(), other_iter.peek().unwrap()) {
                std::cmp::Ordering::Equal => {
                    // other only ensures that we don't step past duplicate other points
                    other_iter.next();
                }
                std::cmp::Ordering::Less => {
                    self_iter.next();
                }
                std::cmp::Ordering::Greater => {
                    return false;
                }
            }
        }
    }
}

impl_covers_from_intersects!(MultiPoint<T>, [Point<T>]);

macro_rules! impl_multipoint_covers_multi_part {
    ( [$($target:ty),*]) => {
        $(
            impl<T> Covers<$target> for MultiPoint<T>
            where
                T: GeoNum,
                Self: Intersects<Coord<T>>,
                Self: HasDimensions,
                $target: HasDimensions,
            {

                fn covers(&self, rhs: &$target) -> bool {
                    if self.is_empty() && rhs.is_empty() && rhs.dimensions() == Dimensions::ZeroDimensional {
                        return false;
                    }
                    // convert to multi point
                    let multipt: MultiPoint<T> = MultiPoint::<T>::from_iter(
                        rhs.iter()
                            .filter_map(|ls| ls.coords_iter().nth(0))
                            .map(|p| Point::<T>::new(p.x, p.y))
                    );

                    return self.covers(&multipt);
                }
            }
        )*
    };
}

macro_rules! impl_multipoint_covers_single_part {
    ( [$($target:ty),*]) => {
        $(
            impl<T> Covers<$target> for MultiPoint<T>
            where
                T: GeoNum,
                Self: Intersects<Coord<T>>,
                Self: HasDimensions,
                $target: HasDimensions,
            {

                fn covers(&self, rhs: &$target) -> bool {
                    if self.is_empty() || rhs.is_empty() || self.dimensions() == Dimensions::ZeroDimensional {
                        return false;
                    }
                    let Some(coord) =  rhs.coords_iter().nth(0) else {return false;};
                    self.intersects(&coord)
                }
            }
        )*
    };
}

impl_multipoint_covers_single_part!([Line<T>, LineString<T>,Rect<T>,Triangle<T>,Polygon<T>]);
impl_multipoint_covers_multi_part!([MultiLineString<T>, MultiPolygon<T>]);

impl_covers_from_relate!(MultiPoint<T>, [Geometry<T>, GeometryCollection<T>]);
