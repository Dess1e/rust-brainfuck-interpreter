use Token::*;
use crate::util::variant_eq;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    MoveFwd(u64),
    MoveBack(u64),
    Inc(u64),
    Dec(u64),
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
            try_pack_simple_token(token, inner);
        } else {
            panic!("Fatal error: wrong token in loop stack.");
        }
    } else {
        try_pack_simple_token(token, result_vec);
    }
}

fn try_pack_simple_token(token: Token, instr_arr: &mut Vec<Token>) {
    let token = token;
    let arr_token = match instr_arr.last() {
        Some(val) => val,
        None => {
            instr_arr.push(token);
            return;
        }
    };
    if !variant_eq(&token, arr_token) {
        instr_arr.push(token);
        return;
    }
    let new_token = match token {
        MoveFwd(n) => {
            if let MoveFwd(n2) = arr_token {
                MoveFwd(n + n2)
            } else { panic!() }
        },
        MoveBack(n) => {
            if let MoveBack(n2) = arr_token {
                MoveBack(n + n2)
            } else { panic!() }
        },
        Inc(n) => {
            if let Inc(n2) = arr_token {
                Inc(n + n2)
            } else { panic!() }
        },
        Dec(n) => {
            if let Dec(n2) = arr_token {
                Dec(n + n2)
            } else { panic!() }
        }
        _ => {
            instr_arr.push(token);
            return;
        }
    };
    instr_arr.pop().unwrap();
    instr_arr.push(new_token);
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
                    '>' => MoveFwd(1),
                    '<' => MoveBack(1),
                    '+' => Inc(1),
                    '-' => Dec(1),
                    '.' => PutCh,
                    ',' => GetCh,
                    _ => panic!("Couldn't tokenize instruction {}. Syntax checker error?", chr)
                };
                try_push_token_to_loop_stack(instr, &mut loop_stack, &mut res);
            }
        }
    }
    res
}

pub fn preprocess(raw_code: &String) -> Vec<Token> {
    tokenize(
        &syntax_check(raw_code)
    )
}
