mod chmod;

use chmod::Chmod;

fn main() {
    let a = Chmod{};
    a.convert_octal_to_symbolic(677);
    a.convert_symbolic_to_octal("rwx-wx-xw".to_string());
}
