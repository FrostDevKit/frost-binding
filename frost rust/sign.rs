use std::collections::HashMap;
use std::fs::File;

use crate::Sign;

pub fn load_signs() -> HashMap<String, Sign> {

    let mut signs = HashMap::new();
    let file = File::open("signs.json").unwrap();
    let signs_json: Vec<Sign> = serde_json::from_reader(file).unwrap();
    for sign in signs_json {
        signs.insert(sign.name.clone(), sign);
    }
    
