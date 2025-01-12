extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
pub mod parser;

pub fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let mut p = parser::Parser::new(input);
        println!("{}",input);
    let ast = try!(p.parse());
    match ast.eval(env) {
    Some(result) => Ok(result),
        None => Err("No value for that expression!".to_string())
}
}
#[wasm_bindgen]
pub fn eval_math(input: &str) -> f64 {
    use std::f64;
    let mut env = HashMap::new();
    env.insert("wow".to_string(), 35.0f64);
    env.insert("pi".to_string(), f64::consts::PI);

    let res = evaluate(input, &mut env);
    match res {
            Err(why) => panic!("{:?}", why),
            Ok(value) => value
    }
}

#[wasm_bindgen]
pub fn add(x:f64 , y:f64 ) -> f64 {
            x+y
        }

#[wasm_bindgen]
pub fn div(x:f64 , y:f64 ) -> f64 {
            x/y
        }

pub fn main() {
    let result = eval_math("sin(30)");
    println!("=> {}", result);

    let result1 = eval_math("pi/2");
     println!("=> {}", result1);

    let result2 = eval_math("283.869575/0.886558");
    println!("=> {}", result2);

}
