pub struct VirtualMachine {
    regs: [u8; 4],
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self { regs: [0; 4] }
    }
    pub fn execute(&mut self, code: u8) {
        // [op 3bit] [dst 2bit] [prefix 1bit] [src 2bit]
        let op = (code >> 5) & 0b111; // 상위 3비트
        let dst = ((code >> 3) & 0b11) as usize; // 그 다음 2비트
        let prefix = (code >> 2) & 0b1; // 그 다음 1비트
        let src = code & 0b11; // 하위 2비트

        let src_val = if prefix == 0 {
            self.regs[src as usize]
        } else {
            src
        };

        match op {
            0b000 => { /* do nothing */ }
            0b001 => {
                // mov
                self.regs[dst] = src_val;
            }
            0b010 => {
                // add
                self.regs[dst] = self.regs[dst].wrapping_add(src_val);
            }
            0b011 => {
                // sub
                self.regs[dst] = self.regs[dst].wrapping_sub(src_val);
            }
            0b100 => {
                if src_val == 0 {
                    println!("Error: Division by zero!");
                } else {
                    self.regs[dst] = self.regs[dst].wrapping_div(src_val);
                }
            }
            0b101 => {
                self.regs[dst] = self.regs[dst].wrapping_mul(src_val);
            }
            0b110 => {
                self.regs[dst] = !(self.regs[dst] & src_val);
            }
            _ => println!("unknown opcode"),
        }
        println!(
            "register:\na: {}, b: {}, c: {}, d: {}",
            self.regs[0], self.regs[1], self.regs[2], self.regs[3]
        );
    }
}
