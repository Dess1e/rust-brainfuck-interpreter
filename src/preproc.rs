use Token::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    MoveFwd,
    MoveBack,
    Inc,
    Dec,
    PutCh,
    GetCh,
    LoopStart,
    LoopEnd
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

fn tokenize(parsed_code: &String) -> Vec<Token> {
    let mut res = Vec::new();
    for chr in parsed_code.chars() {
        let instr = match chr {
            '>' => MoveFwd,
            '<' => MoveBack,
            '+' => Inc,
            '-' => Dec,
            '.' => PutCh,
            ',' => GetCh,
            '[' => LoopStart,
            ']' => LoopEnd,
            _ => panic!("Couldn't tokenize instruction {}. Syntax checker error?", chr)
        };
        res.push(instr);
    }
    res
}

pub fn preprocess(raw_code: &String) -> Vec<Token> {
    tokenize(
        &syntax_check(raw_code)
    )
}
