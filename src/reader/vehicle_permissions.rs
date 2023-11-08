
use std::collections::{HashMap, HashSet};

use osmpbf::TagIter;


pub struct TagsMap<'a>(HashMap<&'a str, &'a str>);

pub fn convert_tags_to_map(tags: TagIter) -> TagsMap {
    let mut map = HashMap::new();
    for tag in tags {
        map.insert(tag.0, tag.1);
    }

    TagsMap(map)
}

impl<'a> TagsMap<'a> {
    pub fn has_tag(&self, key: &str, values: HashSet<&str>) -> bool  {
        if let Some(tag_value) = self.0.get(key) {
            return values.contains(tag_value);
        }

        false
    }
}

//returns if a car is allowed to drive on the given road (forward direction and backward direction)
pub fn is_car_allowed(tags: &TagsMap) -> (bool,bool) {
    //TODO
    return (true,true);
}