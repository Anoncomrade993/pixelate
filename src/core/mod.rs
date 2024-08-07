pub mod lsb {
    /// Perform LSB replacement to encode data into image pixels
    pub fn encode(pixels: &mut Vec<u8>, data: &str, channel: u8) -> Result<Vec<u8>, &'static str> {
        let mut tracker = 0;
        let data_len = data.len();

        if pixels.is_empty() || data.trim().is_empty() {
            return Err("Inputs can't be null or empty");
        }

        if is_large(pixels.len(), data_len) {
            return Err("Data too big for pixels capacity");
        }

        for q in (0..pixels.len()).step_by(4) {
            if tracker < data_len {
                if let Some(pixel) = pixels.get_mut(q + channel as usize) {
                    *pixel &= 0xFE;
                    *pixel |= (data.chars().nth(tracker).unwrap() as u8 - b'0') & 1;
                    tracker += 1;
                } else {
                    break; // Exit loop if pixel is None
                }
            } else {
                break;
            }
        }
        Ok(pixels.to_vec())
    }

    /// Check image capacity
    fn is_large(arr_len: usize, bin_len: usize) -> bool {
        bin_len > (arr_len * 8) / 4
    }

    /// Reverse the LSB replacement to decode data from image pixels
    pub fn decode(pixels: Vec<u8>, channel: u8) -> Result<String, &'static str> {
        let mut binary_string = String::new();

        if pixels.is_empty() {
            return Err("Pixel array is empty");
        }

        for q in (0..pixels.len()).step_by(4) {
            let lsb = pixels[q + channel as usize] & 1;
            binary_string.push_str(&lsb.to_string());
        }

        Ok(binary_string)
    }
}
