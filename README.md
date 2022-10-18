# huffman_compression

Compression using Huffman coding in Rust

## Usage

```sh
cargo run [--release] <compress|decompress> <input file> [output_file]
```

For compression the output file defaults to `out.hfm` and for decompression the output file defaults to whatever the original name of the file was.  
Kind of a spinoff of [https://github.com/ndm767/huffman](https://github.com/ndm767/huffman).  

Test data gotten from [The Canterbury Corpus](https://corpus.canterbury.ac.nz/descriptions/).  

[enwik8](http://www.mattmahoney.net/dc/text.html) benchmark results (using release build profile):  
compress 96M -> 61M in 2.63s  
decompress 61M -> 96M in 50.79s
