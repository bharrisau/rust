#[feature(phase,trace_macros)];

#[phase(syntax)]
extern mod peg;

parse_peg!()
