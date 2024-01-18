mod ir;
mod lexer;
use std::process::ExitCode;

use lexer::Lexer;

const TESTSOURCE_ROT13: &[u8] = br#"
-,+[-[>>++++[>++++++++<-]<+<-[>+>+>-[>>>]<[[>+<-]>>+>]<<<<<-]]>>>[-]+
>--[-[<->+++[-]]]<[++++++++++++<[>-[>+>>]>[+[<+>-]>+>>]<<<<<-
]>>[<+>-]>[-[-<<[-]>>]<<[<<->>-]>>]<<[<<+>>-]]<[-]<.[-]<-,+]
"#;
const TESTSOURCE_HELLOWORLD: &[u8] = br#"
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.
>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
"#;

fn main() -> ExitCode {
    let lexer = Lexer::new(TESTSOURCE_ROT13);
    match ir::IR::new(lexer) {
        Ok(ires) => match ires.interprete() {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("ERROR: {err}");
                ExitCode::FAILURE
            }
        },
        Err(err) => {
            eprintln!("ERROR: {err}");

            ExitCode::FAILURE
        }
    }
}
