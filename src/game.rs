use std::string::String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    name: String,
    limit: i32,
}

impl Game {
    pub fn new(name: String, limit: i32) -> Game {
        Game {
            name: name,
            limit: limit,
        }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn limit(&self) -> &i32 {
        &self.limit
    }
}
