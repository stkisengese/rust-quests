#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let swap_byte = |byte| {
            match byte {
                b if b == first => second,
                b if b == second => first,
                _ => byte,
            }
        };

       Color {
            r: swap_byte(self.r),
            g: swap_byte(self.g),
            b: swap_byte(self.b),
            a: swap_byte(self.a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        let result = c.swap(c.r, c.b);
        assert_eq!(result, Color {r: 10, g: 200, b: 255, a: 30});
    }
}
