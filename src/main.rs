use wgpu_tilemap::runner::run;

fn main() {
    pollster::block_on(run());
}
