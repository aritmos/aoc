#[derive(Debug, Default)]
pub struct Hand {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Hand {
    pub fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}
