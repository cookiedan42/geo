use super::{Covers, impl_covers_from_intersects};
use crate::GeoNum;
use crate::{CoordsIter, HasDimensions, Intersects, geometry::*};

/*
    If self is a single line
    and all points of other intersect self,
    then self covers other.
*/

impl_covers_from_intersects!(coord: Line<T>);
impl_covers_from_intersects!(Line<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_intersects!(Line<T>, [Line<T>]);
impl_covers_from_intersects!(Line<T>, [LineString<T>,  MultiLineString<T>]);
impl_covers_from_intersects!(Line<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_intersects!(Line<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_intersects!(Line<T>, [GeometryCollection<T>]);
