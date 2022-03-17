use Token::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    MoveFwd,
    MoveBack,
    Inc,
    Dec,
    PutCh,
    GetCh,
    LoopL,
    LoopR,
    Loop(Vec<Token>),
}

fn syntax_check_parse_punctuation(code: &String) -> String {
    const ALLOWED_CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];
    let mut code_result = String::new();
    for char in code.chars() {
        if ALLOWED_CHARS.contains(&char) {
            code_result.push(char);
        }
    }
    code_result
}

fn syntax_check_lexical(code: &String) {
    let mut opened_blocks = 0;
    for (indx, char) in code.chars().enumerate() {
        opened_blocks += match char {
            '[' => 1,
            ']' => -1,
            _ => 0
        };
        if opened_blocks < 0 {
            panic!("Unmatched loop end token `]` at pos {}", indx);
        }
    }
    if opened_blocks > 0 {
        panic!("Unmatched loop start token `[`");
    }
}

fn syntax_check(code: &String) -> String {
    let parsed_code = syntax_check_parse_punctuation(code);
    syntax_check_lexical(&parsed_code);
    parsed_code
}

fn try_push_token_to_loop_stack(
    token: Token,
    loop_stack: &mut Vec<Token>,
    result_vec: &mut Vec<Token>
) {
    if loop_stack.len() > 0 {
        let last_loop = loop_stack.last_mut().unwrap();
        if let Loop(inner) = last_loop {
            inner.push(token);
        } else {
            panic!("Fatal error: wrong token in loop stack.");
        }
    } else {
        result_vec.push(token);
    }
}


fn tokenize(parsed_code: &String) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    let mut loop_stack: Vec<Token> = Vec::new();
    for chr in parsed_code.chars() {
        match chr {
            '[' => {
                let loop_ = Loop(vec![LoopL]);
                loop_stack.push(loop_);
            },
            ']' => {
                try_push_token_to_loop_stack(LoopR, &mut loop_stack, &mut res);
                let last_loop = loop_stack.pop().unwrap();
                try_push_token_to_loop_stack(last_loop, &mut loop_stack, &mut res);
            },
            _ => {
                let instr = match chr {
                    '>' => MoveFwd,
                    '<' => MoveBack,
                    '+' => Inc,
                    '-' => Dec,
                    '.' => PutCh,
                    ',' => GetCh,
                    _ => panic!("Couldn't tokenize instruction {}. Syntax checker error?", chr)
                };
                try_push_token_to_loop_stack(instr, &mut loop_stack, &mut res);
            }
        }
    }
    println!("{:?}", res);
    res
}

pub fn preprocess(raw_code: &String) -> Vec<Token> {
    tokenize(
        &syntax_check(raw_code)
    )
}
