pub struct Sleigh {
    color: String,
    engine: String,
    gift_capacity: u32,
    magical_enhancements: bool,
}

pub struct SleighBuilder {
    color: String,
    engine: String,
    gift_capacity: u32,
    magical_enhancements: bool,
}

impl SleighBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn color(&mut self, color: &str) -> &mut Self {
        self.color = color.to_string();
        self
    }

    fn engine(&mut self, engine: &str) -> &mut Self {
        self.engine = engine.to_string();
        self
    }

    fn gift_capacity(&mut self, capacity: u32) -> &mut Self {
        self.gift_capacity = capacity;
        self
    }

    fn magical_enhancements(&mut self) -> &mut Self {
        self.magical_enhancements = true;
        self
    }

    fn build(&self) -> Sleigh {
        Sleigh {
            color: self.color.clone(),
            engine: self.engine.clone(),
            gift_capacity: self.gift_capacity,
            magical_enhancements: self.magical_enhancements,
        }
    }
}

impl Default for SleighBuilder {
    fn default() -> Self {
        Self {
            color: "red".to_string(),
            engine: "reindeer-powered".to_string(),
            gift_capacity: Default::default(),
            magical_enhancements: Default::default(),
        }
    }
}

impl Sleigh {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn engine(&self) -> &str {
        &self.engine
    }

    pub fn gift_capacity(&self) -> u32 {
        self.gift_capacity
    }

    pub fn magical_enhancements(&self) -> bool {
        self.magical_enhancements
    }
}

fn main() {
    let sleigh = SleighBuilder::new()
        .color("gold")
        .engine("magic")
        .gift_capacity(350)
        .magical_enhancements()
        .build();

    assert_eq!(sleigh.color(), "gold");
    assert_eq!(sleigh.engine(), "magic");
    assert_eq!(sleigh.gift_capacity(), 350);
    assert!(sleigh.magical_enhancements());
}
