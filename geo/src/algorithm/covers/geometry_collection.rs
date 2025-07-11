use super::{Covers, impl_covers_from_relate};
use crate::CoordsIter;
use crate::HasDimensions;
use crate::Intersects;
use crate::covers::impl_covers_from_intersects;
use crate::geometry::*;
use crate::{GeoFloat, GeoNum};

impl_covers_from_intersects!(coord:GeometryCollection<T>);
impl_covers_from_intersects!(GeometryCollection<T>, [Point<T>]);

impl_covers_from_relate!(GeometryCollection<T>, [Line<T>, LineString<T>, Polygon<T>, MultiPoint<T>, MultiLineString<T>, MultiPolygon<T>, GeometryCollection<T>, Rect<T>, Triangle<T>]);
