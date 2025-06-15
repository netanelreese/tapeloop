// TODO 
// led struct that holds current: bool and last_on: u64 as timestamp 
// Led function 

// Led struct
pub struct Led {
    current: bool,
    last_on: u64, // timestamp
}

impl Led{
    pub fn new() -> Self {
        Self {current: false }
    }

    pub fn turn_on(&mut self) {
        self.current = true;
    }
}


#[cfg(test)]
mod tests {
    // TODO: write some tests here!
}


