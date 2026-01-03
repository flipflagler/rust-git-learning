use std:: collections::HashMap;

fn group_values_by_key (vec: Vec<(String,i32)>) -> HashMap<String,i32>{
    let mut hm = HashMap:: new();
    for (key ,value) in vec{
        hm.insert(key,value);
    }
    return hm;
}

fn main(){
    let input_vec = vec![(String::from("harsh"),20),(String::from("raman"),30)];
    let hm =  group_values_by_key(input_vec);
    print!("{:?}",hm);
}