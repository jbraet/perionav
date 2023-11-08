
use std::collections::HashSet;
use lazy_static::lazy_static;

use super::tags_map::TagsMap;

lazy_static! (
    static ref DEFAULT_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes", "permissive", "designated", "open", "destination", "delivery"].iter().cloned());
    static ref ONEWAY_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1","-1","reverse"]);
    static ref ONEWAY_FORWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1"]);
    static ref ONEWAY_BACKWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["-1","reverse"]);

    //Where we can accept cars by default, without an explicit car tag
    //so if theres no explicit car tag on a road thats not in here then reject
    static ref NORMAL_ROADS: HashSet<&'static str> = HashSet::from_iter(["motorway","motorway_link","motorroad", "trunk", "trunk_link", "primary", "primary_link","secondary","secondary_link","tertiary","tertiary_link","unclassified","residential","living_street","service","road","track"]); 
    static ref SPECIAL_ROADS: HashSet<&'static str> = HashSet::from_iter(["pedestrian", "footway", "path", "bridleway", "cycleway", "steps", "platform", "bus_stop", "busway", "bus_guideway", "emergency_access_point", "no", "proposed", "construction", "abandoned"]);

    static ref FERRY_ROADS: HashSet<&'static str> = HashSet::from_iter(["shuttle_train","ferry"]);
);

//returns if a car is allowed to drive on the given road (forward direction and backward direction)
pub fn is_car_allowed(tags: &TagsMap) -> (bool,bool) {
    if !tags.has_key("highway") {
        return (false,false)
    }

    if tags.tag_equals("area", "yes") || tags.has_key("parking") || tags.has_key("amenity") ||  tags.has_key("aeroway") || tags.has_key("barrier") || tags.has_key("railway") || tags.has_key("man_made") || tags.has_key("building") || tags.has_key("landuse") || tags.has_key("natural")  || tags.has_key("leisure") || tags.has_key("golf") || tags.has_key("waterway") || tags.has_key("boundary") {
        return (false,false)
    }

    if tags.tag_in_values("road", &FERRY_ROADS) {
        return (false,false)
    }

    if tags.tag_equals("impassable", "yes") || tags.tag_equals("status", "impassable") {
        return (false,false)
    }

    let (car, explicit) = tags.has_tag_ordered(&["motorcar","motor_vehicle","vehicle","access"], &DEFAULT_VALUES);
    if !car && explicit { // this must mean theres an explicit tag saying cars aren't allowed
        return (false,false);
    } else if !explicit { //there is no explicit tag saying anything about car access
        if !tags.tag_in_values("highway", &NORMAL_ROADS) { //SPECIAL_ROADS and normal roads are disjunct this is good enough
            return (false,false);
        }
    }

    //now we can assume that either there is an explicit car tag
    //or there is no explicit car tag but there are also no explicit foot and bike tags
    //so this is assumed to be a car way. We still have to check if it is a oneway or not

    let mut forward = true;
    let mut backward = true;
    if tags.tag_in_values("oneway", &ONEWAY_FORWARD_VALUES) ||tags.tag_in_values("oneway:vehicle", &ONEWAY_FORWARD_VALUES) || tags.tag_in_values("oneway:motor_vehicle", &ONEWAY_FORWARD_VALUES) {
        backward = false;
    } else if tags.tag_in_values("oneway", &ONEWAY_BACKWARD_VALUES) ||tags.tag_in_values("oneway:vehicle", &ONEWAY_BACKWARD_VALUES) || tags.tag_in_values("oneway:motor_vehicle", &ONEWAY_BACKWARD_VALUES) {
        forward = false;
    }

    (forward,backward)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use super::super::tags_map::TagsMap;

    #[test]
    fn test_is_car_allowed() {
        let mut map : HashMap<&str, &str> = HashMap::new();
        map.insert("vehicle","yes");
        let tags_map=TagsMap(map);
        let (fwd,bwd) = is_car_allowed(&tags_map);
        assert!(fwd&&bwd);

        //test if extra unrelated field causes different behavior
        let mut map : HashMap<&str, &str> = HashMap::new();
        map.insert("motorcar","delivery");
        map.insert("apple","pear");
        let tags_map=TagsMap(map);
        let (fwd,bwd) = is_car_allowed(&tags_map);
        assert!(fwd&&bwd);

        //test if bike roads get rejected
        let mut map:HashMap<&str, &str> = HashMap::new();
        map.insert("bike","yes");
        let tags_map = TagsMap(map);
        let (fwd,bwd) = is_car_allowed(&tags_map);
        assert!(!fwd&&!bwd);

        //test if reverse oneways work
        let mut map : HashMap<&str, &str> = HashMap::new();
        map.insert("oneway:vehicle","-1");
        let tags_map=TagsMap(map);
        let (fwd,bwd) = is_car_allowed(&tags_map);
        assert!(!fwd&&bwd);

        //test if oneways work
        let mut map : HashMap<&str, &str> = HashMap::new();
        map.insert("oneway","yes");
        let tags_map=TagsMap(map);
        let (fwd,bwd) = is_car_allowed(&tags_map);
        assert!(fwd&&!bwd);
    }
}
