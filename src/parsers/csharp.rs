
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

pub fn parse(input : &str) -> Result<Vec<CSharp>, Box<dyn std::error::Error>> {

    let ls = lexer::lex(input)?;
    let bs = bracketer::bracket(ls.into_iter())?;
    let output = bracket::process(&[], bs.into_iter())?;

    Ok(output)
}

fn blarg(input : &mut Vec<Lexeme>) {
}

#[cfg(test)]
mod test {
    use super::*;

}
