use std::io;
use std::io::prelude::*;
use std::cmp::*;


fn main() {
    let totalReds = 12;
    let totalGreens = 13;
    let totalBlues = 14;

    // state machine
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let mut numReds: i32 = 0;
    let mut numBlues: i32 = 0;
    let mut numGreens: i32 = 0;
    let mut reg1: i32 = 0;
    let mut line = 1;
    for i in io::stdin().bytes() {
        match i {
            Ok(byte) => {
                match byte {
                    b'0'..=b'9' => {
                        reg1 = 10 * reg1 + (byte - b'0') as i32;
                        /*
                        if reg1 > 10 {
                            println!("multi-digit number, got {}", reg1);
                        }
                        */
                    },
                    b'\n' => {
                        // println!("{} {} {} {}", line, numReds, numGreens, numBlues);
                        if (numReds <= totalReds && numBlues <= totalBlues && numGreens <= totalGreens) {
                            // println!("possible");
                            sum += line;
                        }
                        let power = numReds * numGreens * numBlues;
                        sum2 += power;
                        // println!("{}, power: {}", line, power);

                        line += 1;
                        numReds = 0;
                        numBlues = 0;
                        numGreens = 0;
                        reg1 = 0;
                    },
                    b'r' => {
                        numReds = max(reg1, numReds);
                        reg1 = 0;
                    },
                    b'g' => {
                        numGreens = max(reg1, numGreens);
                        reg1 = 0;
                    },
                    b'b' => {
                        numBlues = max(reg1, numBlues);
                        reg1 = 0;
                    },
                    b':' => {
                        reg1=0;
                    },
                    _ => {
                    }
                }
            }
            Err(e) => {
                println!("error {}", e);
                return;
            }
        }
    }
    println!("Sum: {}, Sum2: {}", sum, sum2);

}
