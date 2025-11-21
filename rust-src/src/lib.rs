use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn caesar_encrypt(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8 - base) as i32;
                let shifted = (offset + shift).rem_euclid(26) as u8;
                (base + shifted) as char
            } else {
                c
            }
        })
        .collect()
}

#[wasm_bindgen]
pub fn caesar_decrypt(text: &str, shift: i32) -> String {
    caesar_encrypt(text, -shift)
}

#[wasm_bindgen]
pub fn rot13(text: &str) -> String {
    caesar_encrypt(text, 13)
}

#[wasm_bindgen]
pub fn xor_cipher(text: &str, key: &str) -> String {
    if key.is_empty() {
        return text.to_string();
    }
    
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let key_char = key.chars().nth(i % key.len()).unwrap();
            ((c as u8) ^ (key_char as u8)) as char
        })
        .collect()
}

#[wasm_bindgen]
pub fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

#[wasm_bindgen]
pub fn glitch_text(text: &str) -> String {
    let glitch_chars = ['҉', '̴', '̵', '̶', '̷', '̸', '̡', '̢', '̧', '̨', '̛', '̖', '̗', '̘', '̙', '̜', '̝', '̞', '̟'];
    
    text.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 && !c.is_whitespace() {
                vec![c, glitch_chars[i % glitch_chars.len()]]
            } else {
                vec![c]
            }
        })
        .collect()
}
#[wasm_bindgen]
pub fn base64_encode(text: &str) -> String {
    use std::collections::HashMap;
    
    let b64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = text.as_bytes();
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = buf[0] >> 2;
        let b2 = ((buf[0] & 0x03) << 4) | (buf[1] >> 4);
        let b3 = ((buf[1] & 0x0F) << 2) | (buf[2] >> 6);
        let b4 = buf[2] & 0x3F;
        
        result.push(b64_chars.chars().nth(b1 as usize).unwrap());
        result.push(b64_chars.chars().nth(b2 as usize).unwrap());
        
        if chunk.len() > 1 {
            result.push(b64_chars.chars().nth(b3 as usize).unwrap());
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(b64_chars.chars().nth(b4 as usize).unwrap());
        } else {
            result.push('=');
        }
    }
    
    result
}

#[wasm_bindgen]
pub fn base64_decode(text: &str) -> String {
    let b64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let clean: String = text.chars().filter(|&c| c != '=' && c != '\n' && c != '\r').collect();
    
    for chunk in clean.as_bytes().chunks(4) {
        if chunk.len() < 2 { break; }
        
        let b1 = b64_chars.find(chunk[0] as char).unwrap_or(0);
        let b2 = b64_chars.find(chunk[1] as char).unwrap_or(0);
        let b3 = if chunk.len() > 2 { b64_chars.find(chunk[2] as char).unwrap_or(0) } else { 0 };
        let b4 = if chunk.len() > 3 { b64_chars.find(chunk[3] as char).unwrap_or(0) } else { 0 };
        
        result.push(((b1 << 2) | (b2 >> 4)) as u8);
        if chunk.len() > 2 {
            result.push((((b2 & 0x0F) << 4) | (b3 >> 2)) as u8);
        }
        if chunk.len() > 3 {
            result.push((((b3 & 0x03) << 6) | b4) as u8);
        }
    }
    
    String::from_utf8_lossy(&result).to_string()
}
