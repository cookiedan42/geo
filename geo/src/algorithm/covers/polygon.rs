use super::{Covers, impl_covers_from_intersects, impl_covers_from_relate};
use crate::HasDimensions;
use crate::{CoordsIter, Intersects, geometry::*};
use crate::{GeoFloat, GeoNum};

impl_covers_from_intersects!(coord:Polygon<T>);
impl_covers_from_intersects!(Polygon<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(Polygon<T>, [Line<T>, LineString<T>,  MultiLineString<T>,Rect<T>, Triangle<T>,Polygon<T>,  MultiPolygon<T>,GeometryCollection<T>]);

//
// MultiPolygon Implementations
//

impl_covers_from_intersects!(coord:MultiPolygon<T>);
impl_covers_from_intersects!(MultiPolygon<T>, [Point<T>, MultiPoint<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Line<T>, LineString<T>,  MultiLineString<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Rect<T>, Triangle<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [Polygon<T>,  MultiPolygon<T>]);
impl_covers_from_relate!(MultiPolygon<T>, [GeometryCollection<T>]);
