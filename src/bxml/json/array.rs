use crate::bxml::json::JSonPrimitive;
use std::slice::Iter;
use crate::bxml::JSonObject;
use crate::bxml::JSon;
use crate::bxml::JSonItem;

pub struct JSonArray{ data:Vec<Box<dyn JSonItem>> }

impl JSonArray{
    pub fn new()->Self{ JSonArray{ data:Vec::new() } }

    pub fn add(&mut self, item:Box<dyn JSonItem>){
        self.data.push(item);
    }

    pub fn get_itr(&self)->Iter<Box<dyn JSonItem>>{ self.data.iter() }
}

impl JSonItem for JSonArray{
    fn is_primitive(&self)->bool{ false }
    fn is_object(&self)->bool{ false }
    fn is_array(&self)->bool{ true }

    fn as_primitive(&self)->Option<&JSonPrimitive>{ None }
    fn as_object(&self)->Option<&JSonObject>{ None }
    fn as_array(&self)->Option<&JSonArray>{ Some(self) }
    fn to_string(&self)->String{
        let mut builder = String::from("[");
        let mut i = 0;
        for item in self.data.iter(){
            builder.push_str(format!("{}", item.to_string()).as_ref());
            if i < self.data.len() - 1{ builder.push_str(", "); i+= 1;}
        }
        builder.push(']');
		return builder;
    }
}

impl JSon<usize> for JSonArray{
    fn get(&self, key:usize)->Option<&dyn JSonItem>{
        let init = self.data.get(key);
        if let Option::Some(item) = init {
            return Some(item.as_ref());
        }
        return None;
    }

    fn contains(&self, key:usize)->bool{ key < self.data.len() }
    fn remove(&mut self, key:usize){ self.data.remove(key); }
    fn length(&self)->usize{ self.data.len() }
}