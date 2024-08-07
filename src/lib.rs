mod core;
use crate::core::lsb::{encode,decode};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn encode_pixels(pixels:&[u8],data:&str, channel:u32) -> Vec<u8>{
  let res =  encode(&mut pixels.to_vec(),data, channel as u8).unwrap();
   res
}


#[wasm_bindgen]
pub fn decode_pixels(pixels:&[u8],channel:u32) -> String{
  let res =  match decode(pixels.to_vec(),channel as u8){
     Ok(encoded) => encoded.to_string(),
     Err(_) => return format!("error")
   };
   res
}
