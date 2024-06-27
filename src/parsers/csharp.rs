
use std::rc::Rc;
use dealize::jerboa::*;

#[derive(Clone)]
pub enum CSharp {
    Class,
    Struct,
    Enum,
    Interface,
    Record,
    Comment(Box<str>),
}

#[derive(Clone)]
enum IR {
    C(char),
    Comment(Box<str>),
    None,
}

macro_rules! iproj {
    ($input:expr) => {
        match $input {
            Capture::Item(x) => { Ok(IR::C(*x)) },
            _ => { unreachable!() },
        }
    }
}

macro_rules! rproj {
    ($input:expr) => {
        match &$input {
            Capture::Result(x) => { Ok(x.clone()) },
            _ => { unreachable!(); },
        }
    }
}

macro_rules! lproj {
    ($input:expr, $p:pat, $x:expr) => {
        match &$input {
            Capture::List($p) => { Ok($x) },
            _ => { unreachable!(); },
        }
    }
}

macro_rules! proj {
    ($input:expr, $p:pat, $x:expr) => {
        match $input {
            $p => { $x },
            _ => { unreachable!(); },
        }
    }
}

static mut TOP_RULE : Option<Rc<Rule<char, IR>>> = None;

fn rule() -> Rc<Rule<char, IR>> {

    unsafe { 
        if TOP_RULE.is_none() {
            let slash : Match<char, IR> = Match::pred(|c, _| *c == '/');
            let star : Match<char, IR> = Match::pred(|c, _| *c == '*');

            let end_line : Rc<Rule<char, IR>> = Rule::new
            ( "end_line"
            , vec![Match::pred(|c, _| *c == '\r' || *c == '\n')]
            , |_| Ok(IR::None)
            );
            
            let anything : Rc<Rule<char, IR>> = Rule::new
            ( "anything"
            , vec![ Match::pred(|c, _| true) ]
            , |cs| iproj!(cs[0])
            );

            let line_comment : Rc<Rule<char, IR>> = Rule::new
            ( "line_comment"
            , vec![ slash.clone()
                  , slash.clone()
                  , Match::until(&anything, &end_line)
                  ]
            , |cs| lproj!(cs[2], x, IR::Comment(x.iter().map(|y| proj!(y, IR::C(z), z)).collect::<String>().into()) )
            );

            TOP_RULE = Some(Rule::new("top", vec![], |_| Ok(IR::None)));
            Rc::clone(TOP_RULE.as_ref().unwrap())
        }
        else {
            Rc::clone(TOP_RULE.as_ref().unwrap())
        }
    }
}

pub fn parse(input : &str) -> Result<Vec<CSharp>, Box<dyn std::error::Error>> {
    let r = rule();
    let input = input.chars().collect::<Vec<_>>();

    //Ok(dealize::jerboa::parse(&input, r)?)
    todo!()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_c_sharp() {

    }

}
