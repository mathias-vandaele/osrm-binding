use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Point {
    pub latitude : f64,
    pub longitude : f64,
}