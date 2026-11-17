//! Integration tests for SigmaOS-arch-shader-adaptability-kernel.

use sigmaos_arch_shader_adaptability_kernel_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
