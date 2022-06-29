mod hashmaps;
mod strings;
mod vectors;

use hashmaps::run as hashmapsrun;
use strings::run as stringsrun;
use vectors::run as vectorsrun;

fn main() {
    hashmapsrun();
    // vectorsrun();
    // stringsrun();
}
