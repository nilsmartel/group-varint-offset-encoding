# Group Varint Offset Encoding

- Compressed integers in blocks of 3.
- Has good compression rate, even with outliers.
- Utilized offset encoding to store less bytes.
- Appends zeros to your data, if it can't be grouped into 3. Manually keep track of the exact amount if needed.


## Usage

```rust
let data: Vec<u32> = ...;

use group_varint_offset_encoding::{ compress, decompress };

// anything that can be iteratored into u32s works fine.
let compressed_data = compress(&data);

let decompressed_data = decompress(&compressed_data).to_vec();
```
