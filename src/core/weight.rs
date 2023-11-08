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
    pub fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>, base_node: i32) -> f64 {
        match self {
            WeightCalculator::Distance(v) => v.calc_weight(edge, base_node),
            WeightCalculator::TravelTime(v) => v.calc_weight(edge, base_node),
        }
    }
}

pub struct DistanceWeight {}

impl DistanceWeight {
    #[inline(always)]
    fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>, _base_node: i32) -> f64 {
        edge.get_distance()
    }
}

pub struct TravelTimeWeight {}

impl TravelTimeWeight {
    #[inline(always)]
    fn calc_weight(&self, edge: &Rc<DirectedVehicleSpecificEdgeInformation>, _base_node: i32) -> f64 { //TODO remove _base_node param
        let speed = edge.get_speed();
        if speed > 0.0 {
            return edge.get_distance() / speed;
        }
        f64::INFINITY
    }
}
