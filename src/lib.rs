#[macro_use]
extern crate diesel;
extern crate dotenv;


use wasm_bindgen::prelude::*;

pub mod apis;
pub mod database_stuff;
pub mod schema;
pub mod models;
pub mod calculations;

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
	a + b
 }

 #[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
   if n == 0 || n == 1 {
      return n;
   }

   fib(n - 1) + fib(n - 2)
}

