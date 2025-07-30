use super::impl_covers_geometry_for;
use super::Covers;
use crate::geometry::*;
use crate::geometry_delegate_impl;
use crate::Intersects;
use crate::{GeoFloat, GeoNum};

impl<T> Covers<Coord<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn covers(&self, rhs: &Coord<T>) -> bool {
        self.intersects(rhs)
    }
}

impl<T> Covers<Point<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, point: &Point<T>) -> bool;
    }
}

impl<T> Covers<Line<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, line: &Line<T>) -> bool;
    }
}

impl<T> Covers<LineString<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, line_string: &LineString<T>) -> bool;
    }
}

impl<T> Covers<Polygon<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, polygon: &Polygon<T>) -> bool;
    }
}

impl<T> Covers<MultiPoint<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, multi_point: &MultiPoint<T>) -> bool;
    }
}

impl<T> Covers<MultiLineString<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, multi_line_string: &MultiLineString<T>) -> bool;
    }
}

impl<T> Covers<MultiPolygon<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, multi_line_string: &MultiPolygon<T>) -> bool;
    }
}

impl<T> Covers<GeometryCollection<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, geometry_collection: &GeometryCollection<T>) -> bool;
    }
}

impl<T> Covers<Rect<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, rect: &Rect<T>) -> bool;
    }
}

impl<T> Covers<Triangle<T>> for Geometry<T>
where
    T: GeoFloat,
{
    geometry_delegate_impl! {
        fn covers(&self, triangle: &Triangle<T>) -> bool;
    }
}

impl<T> Covers<Geometry<T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn covers(&self, other: &Geometry<T>) -> bool {
        match other {
            Geometry::Point(geom) => self.covers(geom),
            Geometry::Line(geom) => self.covers(geom),
            Geometry::LineString(geom) => self.covers(geom),
            Geometry::Polygon(geom) => self.covers(geom),
            Geometry::MultiPoint(geom) => self.covers(geom),
            Geometry::MultiLineString(geom) => self.covers(geom),
            Geometry::MultiPolygon(geom) => self.covers(geom),
            Geometry::GeometryCollection(geom) => self.covers(geom),
            Geometry::Rect(geom) => self.covers(geom),
            Geometry::Triangle(geom) => self.covers(geom),
        }
    }
}

impl_covers_geometry_for!(Coord<T>);
impl_covers_geometry_for!(Point<T>);
impl_covers_geometry_for!(MultiPoint<T>);

impl_covers_geometry_for!(Line<T>);
impl_covers_geometry_for!(LineString<T>);
impl_covers_geometry_for!(MultiLineString<T>);

impl_covers_geometry_for!(Rect<T>);
impl_covers_geometry_for!(Triangle<T>);
impl_covers_geometry_for!(Polygon<T>);
impl_covers_geometry_for!(MultiPolygon<T>);

impl_covers_geometry_for!(GeometryCollection<T>);
