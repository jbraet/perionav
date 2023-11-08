pub struct Node {
    pub osm_id: i64,
    pub lat: f64,
    pub lon: f64,
}

impl Node {
    #[inline]
    pub fn new(osm_id: i64, lat: f64, lon:f64) -> Self {
        Node {
            osm_id,
            lat,
            lon,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(0,0.0,0.0)
    }
}
