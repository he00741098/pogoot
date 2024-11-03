mod lib;
use lib::run;
fn main() {
// std::env::set_var(r"WINIT\_UNIX\_BACKEND", "x11");
    pollster::block_on(run());
}
