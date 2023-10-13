//! Definitions for the native PUT token and its fractional lamports.

#![allow(clippy::integer_arithmetic)]
/// There are 10^9 lamports in one PUT
pub const LAMPORTS_PER_PUT: u128 = 1_000_000_000;


pub fn parse_amount(amount:&str) -> (u128, f64){
    let num:Vec<&str> = amount.split('.').collect();
    if num.len() == 1 {
        return (num[0].parse::<u128>().unwrap(),0_f64)
    }else if num.len() == 2  {
        if num[1] != ""{
            let frac  = "0.".to_owned() + num[1];
            return (num[0].parse::<u128>().unwrap(),frac.parse::<f64>().unwrap())
        }else{
            return (num[0].parse::<u128>().unwrap(),0_f64)
        }
    }else {
        return (0_u128,0_f64)
    }
}

/// Approximately convert fractional native tokens (lamports) into native tokens (PUT)
pub fn lamports_to_put(lamports: u128) -> String {
    // Left-pad zeros to decimals + 1, so we at least have an integer zero
    let mut s = format!("{:01$}", lamports, 10);
    // Add the decimal point (Sorry, "," locales!)
    s.insert(s.len() - 9, '.');
    let zeros_trimmed = s.trim_end_matches('0');
    s = zeros_trimmed.trim_end_matches('.').to_string();
    s
}

/// Approximately convert native tokens (PUT) into fractional native tokens (lamports)
pub fn put_to_lamports(put: &str) -> u128 {
    let (inte,frac) = parse_amount(put);
    inte * LAMPORTS_PER_PUT + (frac * LAMPORTS_PER_PUT as f64) as u128
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct PUT(pub u128);

impl PUT {
    fn write_in_put(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_PUT,
            self.0 % LAMPORTS_PER_PUT
        )
    }
}

impl Display for PUT {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_put(f)
    }
}

impl Debug for PUT {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_put(f)
    }
}
