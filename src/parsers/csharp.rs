
use dealize::jerboa::*;

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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_c_sharp() {

    }

}
