use crate::command::Rotation;

pub struct Dial {
    value: i32,
    num_zeros_method_1: i32,
    num_zeros_method_2: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            num_zeros_method_1: 0,
            num_zeros_method_2: 0,
        }
    }
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left { amount } => {
                for _ in 0..amount.get() {
                    self.value = (self.value - 1).rem_euclid(100);
                    if self.value == 0 {
                        self.num_zeros_method_2 += 1;
                    }
                }
            }

            Rotation::Right { amount } => {
                for _ in 0..amount.get() {
                    self.value = (self.value + 1).rem_euclid(100);
                    if self.value == 0 {
                        self.num_zeros_method_2 += 1;
                    }
                }
            }
        }

        if self.value == 0 {
            self.num_zeros_method_1 += 1;
        }
    }

    pub fn num_zeros_method_1(&self) -> i32 {
        self.num_zeros_method_1
    }

    pub fn num_zeros_method_2(&self) -> i32 {
        self.num_zeros_method_2
    }
}
