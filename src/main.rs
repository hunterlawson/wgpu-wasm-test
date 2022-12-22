use wgpu_webasm::run;

fn main() {
    pollster::block_on(run());
}
