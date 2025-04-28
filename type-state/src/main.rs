use std::{fmt::Display, marker::PhantomData};

pub struct Empty;
pub struct Ready;
pub struct Flying;
pub trait State {}
impl State for Empty {}
impl State for Ready {}
impl State for Flying {}

pub struct Sleigh<S: State> {
    _state: PhantomData<S>,
}

impl<S: State> Sleigh<S> {
    fn transition() -> Sleigh<S> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Default for Sleigh<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Sleigh<Empty> {
        Sleigh::transition()
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh::transition()
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh::transition()
    }
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh::transition()
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh::transition()
    }
}

impl Display for Sleigh<Empty> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sleigh is empty")
    }
}

impl Display for Sleigh<Ready> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sleigh is ready")
    }
}

impl Display for Sleigh<Flying> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sleigh is flying")
    }
}

fn main() {
    let s = Sleigh::new();
    println!("State {}", s);
    let s = s.load();
    println!("State {}", s);
    let s = s.take_off();
    println!("State {}", s);
    let s = s.land();
    println!("State {}", s);
    let s = s.unload();
    println!("State {}", s);
}
