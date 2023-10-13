use {
    put_account_decoder::parse_token::real_number_string_trimmed,
    put_sdk::native_token::lamports_to_put,
    std::{
        fmt::{Debug, Display, Formatter, Result},
        ops::Add,
    },
};

const PUT_SYMBOL: &str = "◎";

#[derive(PartialEq)]
pub enum TokenType {
    PUT,
    PplToken,
}

pub struct Token {
    amount: u128,
    decimals: u8,
    token_type: TokenType,
}

impl Token {
    fn write_with_symbol(&self, f: &mut Formatter) -> Result {
        match &self.token_type {
            TokenType::PUT => {
                let amount = lamports_to_put(self.amount );
                write!(f, "{PUT_SYMBOL}{amount}")
            }
            TokenType::PplToken => {
                let amount = real_number_string_trimmed(self.amount, self.decimals);
                write!(f, "{amount} tokens")
            }
        }
    }

    pub fn put(amount: u128) -> Self {
        Self {
            amount : amount,
            decimals: 9,
            token_type: TokenType::PUT,
        }
    }

    pub fn ppl_token(amount: u128, decimals: u8) -> Self {
        Self {
            amount,
            decimals,
            token_type: TokenType::PplToken,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Self) -> Self {
        if self.token_type == other.token_type {
            Self {
                amount: self.amount + other.amount,
                decimals: self.decimals,
                token_type: self.token_type,
            }
        } else {
            self
        }
    }
}
