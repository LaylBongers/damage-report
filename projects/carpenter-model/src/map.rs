#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub brushes: Vec<Brush>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            brushes: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brush {
    data: String,
}

impl Brush {
    pub fn new() -> Self {
        Brush {
            data: "Placeholder Data".into()
        }
    }
}
