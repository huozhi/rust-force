use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//extern {
//   pub fn addTwo(x: i32, y: i32) -> i32;
//}

#[wasm_bindgen]
pub fn add_two(x: i32, y: i32) -> i32 {
    x + y
}
