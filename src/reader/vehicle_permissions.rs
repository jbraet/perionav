
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

use osmpbf::TagIter;


pub struct TagsMap<'a>(HashMap<&'a str, &'a str>);

lazy_static! (
    static ref DEFAULT_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes", "permissive", "designated", "open", "destination", "delivery"].iter().cloned());
    static ref ONEWAY_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1","-1","reverse"]);
    static ref ONEWAY_FORWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1"]);
    static ref ONEWAY_BACKWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["-1","reverse"]);

    static ref SPECIAL_ROADS: HashSet<&'static str> = HashSet::from_iter(["pedestrian", "footway", "path", "bridleway", "cycleway", "steps", "platform", "bus_stop", "busway", "bus_guideway", "emergency_access_point", "no", "proposed", "construction", "abandoned"]);
);

pub fn convert_tags_to_map(tags: TagIter) -> TagsMap {
    let mut map = HashMap::new();
    for tag in tags {
        map.insert(tag.0, tag.1);
    }

    TagsMap(map)
}

impl<'a> TagsMap<'a> {
    pub fn has_tag(&self, key: &str, values: &HashSet<&str>) -> bool  {
        if let Some(tag_value) = self.0.get(key) {
            return values.contains(tag_value);
        }

        false
    }

    //returns if one of the keys has a value in values (in the right order)
    //return false if theres one key that does have a value but the value is not inside values
    //second bool is to indicate if there was an explicit tag or not
    pub fn has_tag_ordered(&self, keys: &[&str], values: &HashSet<&str>) -> (bool,bool)  {
        for key in keys.iter() {
            if let Some(tag_value) = self.0.get(key) {
                return (values.contains(tag_value),true) ;
            } //else its None and we can try the next key value
        }
        
        (true, false)
    }
}

//returns if a car is allowed to drive on the given road (forward direction and backward direction)
pub fn is_car_allowed(tags: &TagsMap) -> (bool,bool) {
    //return (true,true); //TODO temp

    let (car, explicit) = tags.has_tag_ordered(&["motorcar","motor_vehicle","vehicle","access"], &DEFAULT_VALUES);
    if !car && explicit { // this must mean theres an explicit tag saying cars aren't allowed
        return (false,false);
    } else if !explicit{
        if let Some(highway_tag) = tags.0.get("highway") {
            if SPECIAL_ROADS.contains(highway_tag) {
                return (false,false);
            }
        }

        //there was no explicit tag allowing or disallowing cars, so check if it is a foot or bike road.
        //if it isn't then we assume it is accessible by car by default

        let foot = tags.has_tag("foot", &DEFAULT_VALUES);
        if foot {
            return (false, false)
        }
        let bike = tags.has_tag("bike", &DEFAULT_VALUES) || tags.has_tag("bicycle", &DEFAULT_VALUES);
        if  bike {
            return (false, false)
        }
    }

    //now we can assume that either there is an explicit car tag
    //or there is no explicit car tag but there are also no explicit foot and bike tags
    //so this is assumed to be a car way. We still have to check if it is a oneway or not

    let mut forward = true;
    let mut backward = true;
    if tags.has_tag("oneway", &ONEWAY_FORWARD_VALUES) ||tags.has_tag("oneway:vehicle", &ONEWAY_FORWARD_VALUES) || tags.has_tag("oneway:motor_vehicle", &ONEWAY_FORWARD_VALUES) {
        backward = false;
    } else if tags.has_tag("oneway", &ONEWAY_BACKWARD_VALUES) ||tags.has_tag("oneway:vehicle", &ONEWAY_BACKWARD_VALUES) || tags.has_tag("oneway:motor_vehicle", &ONEWAY_BACKWARD_VALUES) {
        forward = false;
    }

    (forward,backward)
}

#[cfg(test)]
mod tests {
    use super::*;

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
