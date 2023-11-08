use std::{collections::HashMap, rc::Rc};


pub struct Edge {
    forward: bool, //TODO review if we need this

    edge_info: Rc<HashMap<VehicleTypes, VehicleSpecificEdgeInformation>>,
}

#[derive(Eq,PartialEq, Hash)]
pub enum VehicleTypes {
    Car,
    Bike,
}

pub struct VehicleSpecificEdgeInformation {
    //properties that can possibly change depending on the direction
    directed_info: (Rc<DirectedVehicleSpecificEdgeInformation>,Rc<DirectedVehicleSpecificEdgeInformation>), //fwd & bwd
}

//properties that stay the same in either direction
pub struct UndirectedVehicleSpecificEdgeInformation {
    distance: f64,   
}

pub struct DirectedVehicleSpecificEdgeInformation {
    undirected_data: Rc<UndirectedVehicleSpecificEdgeInformation>,
    speed: f64,
    _acecssible: bool,
}

impl Edge {
    #[inline]
    pub fn new(distance: f64, is_forward: bool, is_backward: bool) -> Self { //TODO probably some other constructors
        let undirected_data = Rc::new(UndirectedVehicleSpecificEdgeInformation{
            distance,
        });
        let mut edge_info = HashMap::new();
        edge_info.insert(VehicleTypes::Car, VehicleSpecificEdgeInformation{
            directed_info: (Rc::new(DirectedVehicleSpecificEdgeInformation{
                undirected_data: Rc::clone(&undirected_data),
                speed: 1.0,
                _acecssible: is_forward,
            }),Rc::new(DirectedVehicleSpecificEdgeInformation{
                undirected_data: Rc::clone(&undirected_data),
                speed: 1.0,
                _acecssible: is_backward,
            }))
        });

        Edge {
            forward: true,
            edge_info: Rc::new(edge_info),
        }
    }

    pub fn create_opposite(&self) -> Self {
        Self { 
            forward: !self.forward, 
            edge_info: Rc::clone(&self.edge_info) 
        }
    }

    pub fn is_forward(&self, vehicle_type:VehicleTypes,) -> bool {
       self.edge_info.get(&vehicle_type).is_some_and(|e| {
            if self.forward {
                e.directed_info.0._acecssible
            } else {
                e.directed_info.1._acecssible
            }
        })
    }

    pub fn is_backward(&self, vehicle_type:VehicleTypes,) -> bool {
        self.edge_info.get(&vehicle_type).is_some_and(|e| {
            if self.forward {
                e.directed_info.1._acecssible
            } else {
                e.directed_info.0._acecssible
            }
        })
    }

    pub fn get_directed_vehicle_specific_edge_information(&self, vehicle_type:VehicleTypes, reverse: bool) -> Option<Rc<DirectedVehicleSpecificEdgeInformation>> {
        self.edge_info.get(&vehicle_type).map(|e| {
            if reverse^self.forward {
                Rc::clone(&e.directed_info.1)
            } else {
                Rc::clone(&e.directed_info.0)
            }
        })
    }
}

impl DirectedVehicleSpecificEdgeInformation {
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn get_distance(&self) -> f64 {
        self.undirected_data.distance
    }
}
