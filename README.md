# pixelate
Basic Steganograpghy approach using LSB(least Significant Bit) Manipulation 

# How to Use 
-  create a cargo package 
 
 ```bash
  $ cargo new package_name
  ```
- If everything goes well, it will generate a ```Cargo.toml``` file at the root folder.

- Add the library to your ```Cargo.toml```

```toml
[dependencies]
pixelate ={ git="https://github.com/AnonComrade993/pixelate" } 
```
- Then in ```src/main.rs```. import the crate like this:

```rust
extern crate pixelate;
///both methods returns a Result 
use pixelate::Algorithms::lsb::LSB::{encode,decode};
```
# LSB struct
```rust
/*****
 @{pixels:Vec<u8>} one dimensional vector of image pixels
@{binary:String} binary string of information to be hidden
@{channel:i32} from RGB, pick a channel to hide the information in it's lsb
*****/
 
fn encode(pixels:&mut Vec<u8>, binary:String, channel:i32) -> Result<Vec<u8>,&'static str>

/*****
 @{pixels:Vec<u8>} one dimensional vector of image pixels
@{channel:i32} from RGB,the channel information is hidden 
*****/
fn decode(pixels:&mut Vec<u8>, channel:i32) -> Result<String,&'static str>
```


# Security
- try encryption of messages,E.g using
[AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) or any other secured encryption Algorithms to encrypt the message with a very secured key
- pipe the encrypted text to base64 or hex
- convert to binary string 

# Note
. You cannot store an information past the capacity of image 
