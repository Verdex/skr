
use std::rc::Rc;
use dealize::jerboa::*;

pub enum CSharp {
    Class,
    Struct,
    Enum,
    Interface,
    Record,
}

static mut TOP_RULE : Option<Rc<Rule<char, CSharp>>> = None;

static mut TOP_COMMENT : Option<Rc<Rule<char, Option<char>>>> = None;

fn remove_comments(input : &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {

    let r = unsafe { 
        if TOP_COMMENT.is_none() {
            let slash : Match<char, Option<char>> = Match::pred(|c, _| *c == '/');
            let star : Match<char, Option<char>> = Match::pred(|c, _| *c == '*');
            let end_line : Match<char, Option<char>> = Match::pred(|c, _| *c == '\r' || *c == '\n');

            let anything : Rc<Rule<char, Option<char>>> = Rule::new
            ( "anything"
            , vec![ Match::pred(|c, _| true) ]
            , |_| Ok(None)
            );

            let line_comment : Rc<Rule<char, Option<char>>> = Rule::new
            ( "line_comment"
            , vec![ slash.clone()
                  , slash.clone()
                  , Match::list(&anything)
                  ]
            , |_| Ok(None)
            );

            TOP_COMMENT = Some(Rule::new("top", vec![], |_| Ok(None)));
            Rc::clone(TOP_COMMENT.as_ref().unwrap())
        }
        else {
            Rc::clone(TOP_COMMENT.as_ref().unwrap())
        }
    };
    todo!()
}

pub fn parse(input : &str) -> Result<Vec<CSharp>, Box<dyn std::error::Error>> {
    let top_rule = unsafe { 
        if TOP_RULE.is_none() {
            TOP_RULE = Some(Rule::new("blarg", vec![], |_| Ok(CSharp::Class)));
            Rc::clone(TOP_RULE.as_ref().unwrap())
        }
        else {
            Rc::clone(TOP_RULE.as_ref().unwrap())
        }
    };


    todo!()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_c_sharp() {

    }

}
