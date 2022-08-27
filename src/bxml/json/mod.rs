mod array;
mod object;
mod primitive;

pub use array::JSonArray;
pub use object::JSonObject;
pub use primitive::JSonPrimitive;

pub enum JSonItemType{ Primitive, Object, Array, None }

pub trait JSonItem{
    fn is_type(&self)->JSonItemType;

    fn as_primitive(&self)->Option<&JSonPrimitive>;
    fn as_object(&self)->Option<&JSonObject>;
    fn as_array(&self)->Option<&JSonArray>;
    fn to_string(&self)->String;
}

pub trait JSon<T> : JSonItem{
    fn get(&self, key: T)->Option<&dyn JSonItem>;
    fn contains(&self, key: T)->bool;
    fn remove(&mut self, key: T);
    fn length(&self)->usize;

    fn get_number(&self, key: T) -> Option<f32> {
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Primitive =  init.is_type(){
                return init.as_primitive().unwrap().as_number();
            }
        }
        return None;
    }

    fn get_string(&self, key: T) -> std::option::Option<&str> { 
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Primitive =  init.is_type(){
                return Some(init.as_primitive().unwrap().as_string());
            }
        }
        return None;
    }

    fn get_boolean(&self, key:T) -> std::option::Option<bool> { 
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Primitive =  init.is_type(){
                return Some(init.as_primitive().unwrap().as_boolean());
            }
        }
        return None;
    }

    fn get_primitive(&self, key:T) -> std::option::Option<&JSonPrimitive> { 
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Primitive =  init.is_type(){
                return init.as_primitive();
            }
        }
        return None;
    }
    
    fn get_object(&self, key:T) -> Option<&JSonObject> { 
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Object =  init.is_type(){
                return init.as_object();
            }
        }
        return None;
    }
    
    fn get_array(&self, key:T) -> Option<&JSonArray> {
        if let Option::Some(init) = self.get(key){
            if let JSonItemType::Array =  init.is_type(){
                return init.as_array();
            }
        }
        return None;
    }
}

pub struct JSonNone;
impl JSonItem for JSonNone{
    fn is_type(&self)->JSonItemType{ JSonItemType::None }

    fn as_primitive(&self)->Option<&JSonPrimitive>{ None}
    fn as_object(&self)->Option<&JSonObject>{ None }
    fn as_array(&self)->Option<&JSonArray>{ None}
    fn to_string(&self)->String{ String::new() }
}