use std::collections::HashMap;
pub fn sum_conditional(map: &HashMap<String, i32>, str: String) -> i32 {
    let mut sum_age = 0;
    let mut key_set: Vec<String> = Vec::new();
    for key in map.keys() {
        key_set.push(key.to_string());
    }
    let mut counter = 0;
    while counter < key_set.len() {
        if key_set[counter].contains(&str.to_string()) {
            let age = map.get(&key_set[counter]);
            match age {
                Some(age) => {
                    sum_age += age;
                }
                None => log::error!("Data not found "),
            }
        }
        counter += 1;
    }
    log::info!("{}",sum_age);
    sum_age
}