

use syn::{parse::{Parse, ParseStream},Result, punctuated::Punctuated, token::Token};



pub trait PunctuatedExtensions<T: Parse, P: Token + Parse> {
    fn parse_separated_nonempty_until(input: ParseStream, stop_predicate: fn(ParseStream) -> bool)-> Result<Self> where Self:Sized;
}

impl<T: Parse, P: Token+Parse> PunctuatedExtensions<T, P> for Punctuated<T, P> {
    fn parse_separated_nonempty_until(input: ParseStream, stop_predicate: fn(ParseStream) -> bool)-> Result<Self> {
        let mut punctuated = Punctuated::new();
        loop {
            if stop_predicate(input){
                break;
            }
            let value = T::parse(input)?;
            punctuated.push_value(value);
            if !P::peek(input.cursor()) {
                break;
            }
            let punctuation = input.parse()?;
            punctuated.push_punct(punctuation);
        }
        Ok(punctuated)
    }
}
