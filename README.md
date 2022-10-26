# huffman_compression

Compression using Huffman coding in Rust

## Usage

```sh
cargo run [--release] <compress|decompress> <input file> [-o output_file]
```

For compression the output file defaults to `out.hfm` and for decompression the output file defaults to whatever the original name of the file was.  
Kind of a spinoff of [https://github.com/ndm767/huffman](https://github.com/ndm767/huffman).  

Test data gotten from [The Canterbury Corpus](https://corpus.canterbury.ac.nz/descriptions/).  

[Large Text Compression Benchmark](http://www.mattmahoney.net/dc/text.html) benchmark results (using release build profile):  
enwik8 compress 96MiB -> 61MiB in 2.96s  
enwik8 decompress 61MiB -> 96MiB in 3.03s  
enwik9 compress 954MiB -> 619MiB in 24.42s  
enwik9 decompress 619MiB -> 954MiB in 30.11s
