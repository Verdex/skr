
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
    None,
}

static mut TOP_RULE : Option<Rc<Rule<char, CSharp>>> = None;

fn rule() -> Rc<Rule<char, CSharp>> {

    unsafe { 
        if TOP_RULE.is_none() {
            let slash : Match<char, CSharp> = Match::pred(|c, _| *c == '/');
            let star : Match<char, CSharp> = Match::pred(|c, _| *c == '*');

            let end_line : Rc<Rule<char, CSharp>> = Rule::new
            ( "end_line"
            , vec![Match::pred(|c, _| *c == '\r' || *c == '\n')]
            , |_| Ok(CSharp::None)
            );
            
            let anything : Rc<Rule<char, CSharp>> = Rule::new
            ( "anything"
            , vec![ Match::pred(|c, _| true) ]
            , |_| Ok(CSharp::None) 
            );

            let line_comment : Rc<Rule<char, CSharp>> = Rule::new
            ( "line_comment"
            , vec![ slash.clone()
                  , slash.clone()
                  , Match::until(&anything, &end_line)
                  ]
            , |_| Ok(CSharp::None)
            );

            TOP_RULE = Some(Rule::new("top", vec![], |_| Ok(CSharp::None)));
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

    Ok(dealize::jerboa::parse(&input, r)?)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_c_sharp() {

    }

}
