use osmpbf::TagIter;
use std::collections::{HashMap, HashSet};

pub struct TagsMap<'a>(pub HashMap<&'a str, &'a str>);

pub fn convert_tags_to_map(tags: TagIter) -> TagsMap {
    let mut map = HashMap::new();
    for tag in tags {
        map.insert(tag.0, tag.1);
    }

    TagsMap(map)
}

impl<'a> TagsMap<'a> {
    pub fn tag_equals(&self, key: &str, value: &str) -> bool  {
        if let Some(tag_value) = self.0.get(key) {
            return *tag_value==value;
        }

        false
    }

    pub fn tag_in_values(&self, key: &str, values: &HashSet<&str>) -> bool  {
        if let Some(tag_value) = self.0.get(key) {
            return values.contains(tag_value);
        }

        false
    }

    pub fn has_key(&self, key: &str) -> bool  { //doesn't matter which value
        self.0.get(key).is_some()
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