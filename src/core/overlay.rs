use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub trait Overlay {
    fn overlay_by(&self) -> String;
    fn overlay(&self, overlay: &Self) -> Self;
}

pub fn overlay_all<T>(template: &Vec<T>, overlay: &Vec<T>) -> Vec<T>
where
    T: Debug + Clone + Overlay,
{
    let mut templates_by_name = HashMap::new();
    for item in template.iter() {
        templates_by_name.insert(item.overlay_by(), item);
    }

    let mut overlays_by_name = HashMap::new();
    for item in overlay {
        overlays_by_name.insert(item.overlay_by(), item);
    }

    let mut overlay_keys = overlays_by_name.keys().collect::<HashSet<&String>>();
    overlay_keys.extend(templates_by_name.keys().collect::<HashSet<&String>>());

    let mut result = vec![];

    for key in overlay_keys {
        match (templates_by_name.get(key), overlays_by_name.get(key)) {
            (Some(&template), Some(&overlay)) => result.push(template.overlay(overlay)),
            (Some(&item), None) => {
                result.push(item.clone());
            }
            (None, Some(&item)) => {
                result.push(item.clone());
            }
            (None, None) => {}
        }
    }

    result
}
