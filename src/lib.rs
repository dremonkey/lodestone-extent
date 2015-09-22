/// Main crate for lodestone-extent
/// 
/// ## Overview
/// 
/// Calculates the extent (aka bounding box) for a FeaturePolygon. Extent coordinates
/// are returned in WSEN order.
/// 
/// Inspired by [turf-extent](https://github.com/Turfjs/turf-extent).


// Standard lib modules
use std::f64::{INFINITY, NEG_INFINITY};

// Third party crates
extern crate lodestone_polygon;

use lodestone_polygon::FeaturePolygon;

/// ## Return
/// * Array - Contains coordinates in WSEN order (west, south, east, north)
pub extern fn extent(polygon: FeaturePolygon) -> Vec<f64> {
  
  let mut extent = vec![INFINITY, INFINITY, NEG_INFINITY, NEG_INFINITY];
  let ref ring = polygon.coordinates()[0]; // only use outer
  
  for coord in ring {
    if extent[0] > coord[0] { extent[0] = coord[0] } 
    if extent[1] > coord[1] { extent[1] = coord[1] }
    if extent[2] < coord[0] { extent[2] = coord[0] } 
    if extent[3] < coord[1] { extent[3] = coord[1] } 
  }

  extent
}

#[cfg(test)]
mod tests {
  use lodestone_polygon::FeaturePolygon;
  use super::extent;
  
  #[test]
  fn test_simple() {
    let ring = vec![vec![0.0, 0.0], vec![1.0, 2.0], vec![2.0, 0.0], vec![0.0, 0.0]];
    let polygon = FeaturePolygon::new(vec![ring]);
    
    let expected = vec![0.0, 0.0, 2.0, 2.0];
    let bbox = extent(polygon);

    assert_eq!(expected, bbox);
  }
}
