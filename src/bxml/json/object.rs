use std::collections::hash_map::Iter;
use crate::bxml::JSonArray;
use crate::bxml::json::JSonPrimitive;
use crate::bxml::JSon;
use crate::bxml::JSonItem;
use std::collections::HashMap;

pub struct JSonObject{ data:HashMap<String, Box<dyn JSonItem>> }

impl JSonObject{
    pub fn new()->Self{ JSonObject{ data:HashMap::new() } }

    pub fn add(&mut self, key:&str, item:Box<dyn JSonItem>){
        self.data.insert(String::from(key), item);
    }

    pub fn get_itr(&self)->Iter<String, Box<dyn JSonItem>>{ self.data.iter() }
}

impl JSonItem for JSonObject{
    fn is_primitive(&self)->bool{ false }
    fn is_object(&self)->bool{ true }
    fn is_array(&self)->bool{ false }

    fn as_primitive(&self)->Option<&JSonPrimitive>{ None }
    fn as_object(&self)->Option<&JSonObject>{ Some(self) }
    fn as_array(&self)->Option<&JSonArray>{ None }
    fn to_string(&self)->String{
        let mut builder = String::from("{");
        let mut i = 0;
        for (key, item) in self.data.iter(){
            builder.push_str(format!("\"{}\" : {}", key, item.to_string()).as_ref());
            if i < self.data.len() - 1{ builder.push_str(", "); i+= 1;}
        }
        builder.push('}');
		return builder;
    }
}

impl JSon<&str> for JSonObject{
    fn get(&self, key:&str)->Option<&dyn JSonItem>{
        let init = self.data.get(key);
        if let Option::Some(item) = init {
            return Some(item.as_ref());
        }
        return None;
    }

    fn contains(&self, key:&str)->bool{ self.data.contains_key(key)}
    fn remove(&mut self, key:&str){ self.data.remove(key); }
    fn length(&self)->usize{ self.data.len() }
}