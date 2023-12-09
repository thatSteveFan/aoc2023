use std::io;
use std::io::prelude::*;


struct StateMachine<const len: usize> {
    idx: i32,
    val: i32,
    s: [u8; len],
}

impl<const len:usize> StateMachine<len> {
    fn advance(&mut self, chr: u8) -> i32 {
        if self.idx > len as i32 || self.idx < 0 {
            unreachable!();
        }
        if chr == self.s[self.idx as usize] {
            self.idx += 1;
            if self.idx == len as i32 {
                self.idx = 0;
                return self.val;
            }
        }
        else if chr == self.s[0] {
            self.idx = 1;
        }
        else {
            self.idx = 0;
        }
        return 0;
    }
    fn reset(&mut self) {
        self.idx = 0;
    }
}

fn main() {
    let mut sum: i32 = 0;
    let mut reg1: i32 = 0;
    let mut reg2: i32 = 0;
    let mut sm1: StateMachine<3> = StateMachine{idx:0, val:1, s: *b"one"};
    let mut sm2: StateMachine<3> = StateMachine{idx:0, val:2, s: *b"two"};
    let mut sm3: StateMachine<5> = StateMachine{idx:0, val:3, s: *b"three"};
    let mut sm4: StateMachine<4> = StateMachine{idx:0, val:4, s: *b"four"};
    let mut sm5: StateMachine<4> = StateMachine{idx:0, val:5, s: *b"five"};
    let mut sm6: StateMachine<3> = StateMachine{idx:0, val:6, s: *b"six"};
    let mut sm7: StateMachine<5> = StateMachine{idx:0, val:7, s: *b"seven"};
    let mut sm8: StateMachine<5> = StateMachine{idx:0, val:8, s: *b"eight"};
    let mut sm9: StateMachine<4> = StateMachine{idx:0, val:9, s: *b"nine"};
    for i in io::stdin().bytes() {
        match i {
            Ok(byte) => {
                match byte {
                    b'1'..=b'9' => {
                        reg2 = (byte - b'0') as i32;
                        if reg1 == 0 {
                            reg1 = reg2;
                        }
                    },
                    b'\n' => {
                        //print!("line: {}, {}{}\n",line,  reg1, reg2);
                        //print!("{}{}\n",reg1, reg2);
                        sum += reg1 * 10 + reg2;
                        reg1 = 0;
                        reg2 = 0;
                        line += 1;
                        sm1.reset();
                        sm2.reset();
                        sm3.reset();
                        sm4.reset();
                        sm5.reset();
                        sm6.reset();
                        sm7.reset();
                        sm8.reset();
                        sm9.reset();
                    }
                    _ => {
                        let mut res = 0;
                        res |= sm1.advance(byte);
                        res |= sm2.advance(byte);
                        res |= sm3.advance(byte);
                        res |= sm4.advance(byte);
                        res |= sm5.advance(byte);
                        res |= sm6.advance(byte);
                        res |= sm7.advance(byte);
                        res |= sm8.advance(byte);
                        res |= sm9.advance(byte);
                        if res != 0 {
                            reg2 = res;
                            if reg1 == 0 {
                                reg1 = reg2;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("error {}", e);
                return;
            }
        }
    }
    println!("Sum: {}", sum);

}
