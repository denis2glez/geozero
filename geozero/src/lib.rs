//! Zero-Copy reading and writing of geospatial data.
//!
//! GeoZero defines an API for reading geospatial data formats without an intermediate representation.
//! It defines traits which can be implemented to read and convert to an arbitrary format
//! or render geometries directly.
//!
//! Supported geometry types:
//! * [OGC Simple Features](https://en.wikipedia.org/wiki/Simple_Features)
//! * Circular arcs as defined by SQL-MM Part 3
//! * TIN
//!
//! Supported dimensions: X, Y, Z, M, T
//!
//! Available implementations:
//! * [geozero-shp](https://docs.rs/geozero-shp)
//! * [flatgeobuf](https://docs.rs/flatgeobuf)
//!
//! ## Format conversion overview
//!
//! | Format / trait |            [GeozeroGeometry]            |             [GeozeroDatasource]              |     [GeomProcessor]      | Geometry Conversion |
//! |----------------|-----------------------------------------|----------------------------------------------|--------------------------|---------------------|
//! | geo-types      | `geo_types::Geometry<f64>`              | -                                            | [geo_types::GeoWriter]   | [ToGeo]             |
//! | GeoJSON        | `GeoJson`                               | [geojson::GeoJsonReader], [geojson::GeoJson] | [geojson::GeoJsonWriter] | [ToJson]            |
//! | GDAL           | `gdal::vector::Geometry`                | -                                            | [gdal::GdalWriter]       | [ToGdal]            |
//! | GEOS           | `geos::Geometry`                        | -                                            | [geos::GeosWriter]       | [ToGeos]            |
//! | SVG            | -                                       | -                                            | [svg::SvgWriter]         | [ToSvg]             |
//! | WKB            | [wkb::Wkb], [wkb::Ewkb], [wkb::GpkgWkb] | -                                            | [wkb::WkbWriter]         | [ToWkb]             |
//! | WKT            | -                                       | -                                            | [wkt::WktWriter]         | [ToWkt]             |

mod api;
pub mod error;
mod feature_processor;
mod geometry_processor;
mod multiplex;
mod property_processor;

pub use api::*;
pub use feature_processor::*;
pub use geometry_processor::*;
pub use multiplex::*;
pub use property_processor::*;

#[cfg(feature = "with-gdal")]
pub mod gdal;
#[cfg(feature = "with-gdal")]
pub use crate::gdal::conversion::*;

#[cfg(feature = "with-geo")]
pub mod geo_types;
#[cfg(feature = "with-geo")]
pub use crate::geo_types::conversion::*;

#[cfg(feature = "with-geojson")]
pub mod geojson;
#[cfg(feature = "with-geojson")]
pub use crate::geojson::conversion::*;

#[cfg(feature = "with-geos")]
pub mod geos;
#[cfg(feature = "with-geos")]
pub use crate::geos::conversion::*;

#[cfg(feature = "with-gpkg")]
pub mod gpkg;

#[cfg(any(feature = "with-postgis-postgres", feature = "with-postgis-sqlx"))]
pub mod postgis;

#[cfg(feature = "with-svg")]
pub mod svg;
#[cfg(feature = "with-svg")]
pub use crate::svg::conversion::*;

#[cfg(feature = "with-tesselator")]
pub mod tessellator;

#[cfg(feature = "with-wkb")]
pub mod wkb;
#[cfg(feature = "with-wkb")]
pub use crate::wkb::conversion::*;

#[cfg(feature = "with-wkt")]
pub mod wkt;
#[cfg(feature = "with-wkt")]
pub use crate::wkt::conversion::*;

/// Empty processor implementation
pub struct ProcessorSink;

impl ProcessorSink {
    pub fn new() -> ProcessorSink {
        ProcessorSink {}
    }
}

impl feature_processor::FeatureProcessor for ProcessorSink {}
impl geometry_processor::GeomProcessor for ProcessorSink {}
impl property_processor::PropertyProcessor for ProcessorSink {}
