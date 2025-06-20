use crate::{CoordNum, GeometryCollection, MultiLineString, MultiPoint, MultiPolygon, Point, Line, LineString, Polygon, Rect, Triangle};

pub trait MultiPartGeom<T> {}
pub trait SinglePartGeom<T> {}


impl<T> MultiPartGeom<T> for MultiPoint<T> where T:CoordNum{}
impl<T> MultiPartGeom<T> for MultiLineString<T> where T:CoordNum{}
impl<T> MultiPartGeom<T> for MultiPolygon<T> where T:CoordNum{}
impl<T> MultiPartGeom<T> for GeometryCollection<T> where T:CoordNum{}

impl<T> SinglePartGeom<T> for Point<T> where T:CoordNum{}
impl<T> SinglePartGeom<T> for Line<T> where T:CoordNum{}
impl<T> SinglePartGeom<T> for LineString<T> where T:CoordNum{}
impl<T> SinglePartGeom<T> for Polygon<T> where T:CoordNum{}
impl<T> SinglePartGeom<T> for Rect<T> where T:CoordNum{}
impl<T> SinglePartGeom<T> for Triangle<T> where T:CoordNum{}


pub trait ZeroDimensionGeom<T> {}
pub trait OneDimensionGeom<T> {}
pub trait TwoDimensionGeom<T> {}

impl<T> ZeroDimensionGeom<T> for Point<T> where T:CoordNum{}
impl<T> ZeroDimensionGeom<T> for MultiPoint<T> where T:CoordNum{}

impl<T> OneDimensionGeom<T> for Line<T> where T:CoordNum{}
impl<T> OneDimensionGeom<T> for LineString<T> where T:CoordNum{}
impl<T> OneDimensionGeom<T> for MultiLineString<T> where T:CoordNum{}

impl<T> TwoDimensionGeom<T> for Polygon<T> where T:CoordNum{}
impl<T> TwoDimensionGeom<T> for MultiPolygon<T> where T:CoordNum{}
impl<T> TwoDimensionGeom<T> for Rect<T> where T:CoordNum{}
impl<T> TwoDimensionGeom<T> for Triangle<T> where T:CoordNum{}

impl<T> TwoDimensionGeom<T> for GeometryCollection<T> where T:CoordNum{}
