use std::rc::Rc;

use super::edge::DirectedVehicleSpecificEdgeInformation;
pub use super::edge::Edge;

#[non_exhaustive]
pub enum WeightCalculator {
    Distance(DistanceWeight),
    TravelTime(TravelTimeWeight),
}

impl WeightCalculator {
    #[inline(always)]
    pub fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>) -> f64 {
        match self {
            WeightCalculator::Distance(v) => v.calc_weight(edge),
            WeightCalculator::TravelTime(v) => v.calc_weight(edge),
        }
    }
}

pub struct DistanceWeight {}

impl DistanceWeight {
    #[inline(always)]
    fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>) -> f64 {
        edge.get_distance()
    }
}

pub struct TravelTimeWeight {}

impl TravelTimeWeight {
    #[inline(always)]
    fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>) -> f64 {
        let speed = edge.get_speed();
        if speed > 0.0 {
            return edge.get_distance() / speed;
        }
        f64::INFINITY
    }
}
