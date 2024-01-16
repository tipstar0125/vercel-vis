use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as usize).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String,mut _output:String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    if _output.is_empty() {
        _output.push('0');
    }
    let output = util::parse_output(&_output);
    let (score, err, svg) = util::vis(&input, &output, turn);
    Ret {
        score: score as i64,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let output = util::parse_output(&_output);
    output.L
}
