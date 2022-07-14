use std::collections::HashMap;


pub struct  KvStore{
    pub data :  HashMap<String,String>,
}

impl KvStore {
    pub fn new() ->Self{
        KvStore { data: HashMap::new() }
    }

    pub fn set(&mut self,key: String,val: String) {
        unimplemented!()
    }

    pub fn get(& mut self, key: String) ->Option<String> {
        unimplemented!()    
    }

    pub fn remove(& mut self,key:String)
    {
        unimplemented!()
    }
    
}
