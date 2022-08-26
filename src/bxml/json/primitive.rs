use crate::bxml::JSonArray;
use crate::bxml::JSonObject;
use crate::bxml::JSonItem;

#[derive(Debug)]
pub struct JSonPrimitive{ data:String }

#[allow(dead_code)]
impl JSonPrimitive{
    pub fn new()->Self{ JSonPrimitive{ data:String::new() } }
    pub fn from(data:&str)->Self{ JSonPrimitive{ data: String::from(data) } }

    pub fn as_number(&self)->Option<f32>{
        if let Ok(value) = self.data.parse::<f32>(){
            return Some(value);
        }
        return None;
    }

    pub fn as_string(&self)->&str{
        return self.data.as_ref();
    }

    pub fn as_boolean(&self)->bool{
        return self.data.eq("true") || !(self.data.eq("0"));
    }
}

impl JSonItem for JSonPrimitive{
    fn is_primitive(&self)->bool{ true }
    fn is_object(&self)->bool{ false }
    fn is_array(&self)->bool{ false }

    fn as_primitive(&self)->Option<&JSonPrimitive>{ Some(self) }
    fn as_object(&self)->Option<&JSonObject>{ None }
    fn as_array(&self)->Option<&JSonArray>{ None }
    fn to_string(&self)->String{ format!("\"{}\"", self.data.clone()) }
}