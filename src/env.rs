use std::collections::HashMap;
use mushroom::MushRoom;

pub struct Env {
    vars: HashMap<String, MushRoom>,
    parent:Vec<Env>
}

impl Env {

    pub fn new() -> Env {
        Env{vars:HashMap::new(), parent:Vec::with_capacity(1)}
    }
    
    pub fn create_child(self) -> Env {
        let mut newchild = Env::new();
        newchild.parent.push(self);
        newchild
    }
    
    pub fn has_parent(&self) -> bool {
        !self.parent.is_empty()
    }
    
    pub fn get(&self, key:String) -> MushRoom {
        match self.vars.get(&key) {
            Some(i) => i.clone(),
            None => return if self.has_parent() {self.parent[0].get(key)} else {MushRoom::Bool(false)}
        }
    }
    
    pub fn set(&mut self, key:String, val:MushRoom) {
        self.vars.insert(key, val);
    }
    
    pub fn has(&self, key:String) -> bool {
        self.vars.contains_key(&key)
    }
    
    pub fn del(&mut self, key:String) {
        self.vars.remove(&key);
    }
}