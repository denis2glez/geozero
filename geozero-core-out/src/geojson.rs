use geozero_lib::geometry_reader::GeomReader;
use std::io::Write;

struct GeoJsonEmitter<'a, W: Write> {
    out: &'a mut W,
}

impl<'a, W: Write> GeoJsonEmitter<'a, W> {
    fn new(out: &'a mut W) -> GeoJsonEmitter<'a, W> {
        GeoJsonEmitter { out }
    }
    fn comma(&mut self, idx: usize) {
        if idx > 0 {
            self.out.write(b",").unwrap();
        }
    }
}

impl<W: Write> GeomReader for GeoJsonEmitter<'_, W> {
    fn pointxy(&mut self, x: f64, y: f64, idx: usize) {
        self.comma(idx);
        self.out
            .write(&format!("[{},{}]", x, y).as_bytes())
            .unwrap();
    }
    fn point_begin(&mut self, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "Point", "coordinates": "#)
            .unwrap();
    }
    fn point_end(&mut self) {
        self.out.write(b"}").unwrap();
    }
    fn multipoint_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "MultiPoint", "coordinates": ["#)
            .unwrap();
    }
    fn multipoint_end(&mut self) {
        self.out.write(b"]}").unwrap();
    }
    fn line_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "LineString", "coordinates": ["#)
            .unwrap();
    }
    fn line_end(&mut self, _idx: usize) {
        self.out.write(b"]}").unwrap();
    }
    fn multiline_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "MultiLineString", "coordinates": ["#)
            .unwrap();
    }
    fn multiline_end(&mut self) {
        self.out.write(b"]}").unwrap();
    }
    fn ring_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out.write(b"[").unwrap();
    }
    fn ring_end(&mut self, _idx: usize) {
        self.out.write(b"]").unwrap();
    }
    fn poly_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "Polygon", "coordinates": ["#)
            .unwrap();
    }
    fn poly_end(&mut self, _idx: usize) {
        self.out.write(b"]").unwrap();
    }
    fn subpoly_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out.write(b"[").unwrap();
    }
    fn subpoly_end(&mut self, _idx: usize) {
        self.out.write(b"]").unwrap();
    }
    fn multipoly_begin(&mut self, _size: usize, idx: usize) {
        self.comma(idx);
        self.out
            .write(br#"{"type": "MultiPolygon", "coordinates": ["#)
            .unwrap();
    }
    fn multipoly_end(&mut self) {
        self.out.write(b"]}").unwrap();
    }
}
