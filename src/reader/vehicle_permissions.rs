
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

use osmpbf::TagIter;


pub struct TagsMap<'a>(HashMap<&'a str, &'a str>);

lazy_static! (
    static ref DEFAULT_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes", "permissive", "designated", "open", "destination", "delivery"].iter().cloned());
    static ref ONEWAY_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1","-1","reverse"]);
    static ref ONEWAY_FORWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["yes","true","1"]);
    static ref ONEWAY_BACKWARD_VALUES: HashSet<&'static str> = HashSet::from_iter(["-1","reverse"]);
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
    pub fn has_tag_ordered(&self, keys: &[&str], values: &HashSet<&str>) -> bool  {
        for key in keys.iter() {
            if let Some(tag_value) = self.0.get(key) {
                return values.contains(tag_value);
            } //else its None and we can try the next key value
        }
        
        true
    }
}

//returns if a car is allowed to drive on the given road (forward direction and backward direction)
pub fn is_car_allowed(tags: &TagsMap) -> (bool,bool) {
    let car = tags.has_tag_ordered(&["motorcar","motor_vehicle","vehicle","access"], &DEFAULT_VALUES);
    if !car {
        return (false,false);
    }

    let mut forward = true;
    let mut backward = true;
    if tags.has_tag("oneway", &ONEWAY_FORWARD_VALUES) ||tags.has_tag("oneway:vehicle", &ONEWAY_FORWARD_VALUES) || tags.has_tag("oneway:motor_vehicle", &ONEWAY_FORWARD_VALUES) {
        backward = false;
    } else if tags.has_tag("oneway", &ONEWAY_BACKWARD_VALUES) ||tags.has_tag("oneway:vehicle", &ONEWAY_BACKWARD_VALUES) || tags.has_tag("oneway:motor_vehicle", &ONEWAY_BACKWARD_VALUES) {
        forward = false;
    }

    (forward,backward)
}