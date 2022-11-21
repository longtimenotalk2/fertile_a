use super::Resource;

impl Resource {
    pub fn str(&self) -> String {
        match self {
            Resource::Food => "Food".to_string(),
            Resource::Wood => "Wood".to_string(),
        }
    }
}