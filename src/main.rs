mod arg_handler;
mod binary_tree;
mod compress;
mod decompress;

use arg_handler::get_args;
use compress::compress;
use decompress::decompress;

fn main() {
    let args = get_args();
    if args.should_compress {
        compress(args);
    } else {
        decompress(args);
    }
}
