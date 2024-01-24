mod chmod;

use chmod::Chmod;

fn main() {
    let a = Chmod{};
    a.convert_octal_to_symbolic(777);
}
