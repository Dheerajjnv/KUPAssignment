use std::collections::HashMap;
/// _sum_conditional give the sum of age with matching string.
///
///  #Argument
/// map-A referenced hashmap type value contain string and number i32 .
/// string-A string to search pattern
///
///  #Return
///  total_age-The sum of age.
pub fn sum_conditional(map: &HashMap<String, i32>, string: String) -> i32 {
    let mut total_age = 0;
    let mut keys: Vec<String> = Vec::new();
    for key in map.keys() {
        keys.push(key.to_string());
    }
    let mut index = 0;
    while index < keys.len() {
        if keys[index].contains(&string.to_string()) {
            let age = map.get(&keys[index]);
            match age {
                Some(age) => {
                    total_age += age;
                }
                None => log::error!("Data not found "),
            }
        }
        index += 1;
    }
    log::info!("Total age is returned");
    total_age
}
