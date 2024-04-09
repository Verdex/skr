
pub enum CSharp {
    Class,
    Struct,
    Enum,
    Interface,
    Record,
}

pub fn parse(input : &str) -> Result<Vec<CSharp>, Box<dyn std::error::Error>> {


    todo!()
}

enum Lex {
    Bracket(char),
    Symbol(Box<str>),
    String(Box<str>),
}

fn lex(mut input : &[char]) -> Result<Vec<Lex>, String> {
    fn block_comment<'a>(mut input : &'a [char]) -> Result<&'a [char], Box<str>> {
        loop {
            match input { 
                ['*', '/', rest @ ..] => { return Ok(rest); },
                [_, rest @ ..] => { input = rest; },
                [] => { return Err("Encountered end of line in block comment".into()); }
            }
        }
    }

    let mut ret = vec![];
    loop {
        match input {
            ['/', '*', rest @ ..] => { input = block_comment(rest)?; },
            
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
