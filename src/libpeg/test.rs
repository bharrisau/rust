#[feature(phase,trace_macros)];

#[phase(syntax)]
extern mod peg;

parse_peg! calc(
    match start {
        additive => ()
    }

    match additive {
        $left:multiplicative "+" $right:additive => left + right,
        multiplicative                           => ()
    }

    match multiplicative {
        $left:primary "*" $right:multiplicative => left * right,
        primary => ()
    }

    match primary {
        intger => ()
        "(" $additive:additive ")" => additive
    }

    match integer {
        $digits:[0-9]+ => int::from_str(digits).unwrap()
    } 
)

pub fn main() {
    // Create new parser
    let parser = calc::Parser::new();

    // Make it parse "2*(3+4)" and expect 14
    let ans = parser.parse(~"2*(3+4)");
}
