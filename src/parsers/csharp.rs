
use yoke::data::*;
use yoke::parsing::*;
use yoke::matching::*;

pub enum CSharp {
    Class,
    Struct,
    Enum,
    Interface,
    Record,
}

pub fn parse(input : &str) -> Result<CSharp, Box<dyn std::error::Error>> {

    let w = lexer::lex(input)?;


    Ok(CSharp::Record)
}

fn lex(input : Vec<Lexeme>) -> impl Iterator<Item = Lexeme> {
    
}

#[cfg(test)]
mod test {
    use super::*;


}
