use number::Vector3;
use number::clamp;

pub struct Cfg {
    seed: i32,
    
    octaves: f32,
    amplitude: f32,
    smoothness: f32,
    heightOffset: f32,
    roughness: f32,
    max_iters: u32,
}

fn get_noise(cfg: &Cfg, z: Vector3<f32>) -> Vector3<f32> {
    z.clamp(-cfg.octaves, cfg.octaves) * 2.0 - z
}

fn lerp(cfg: &Cfg, z: Vector3<f32>, dz: f32) -> (Vector3<f32>, f32) {
    let factor = cfg.smoothness / clamp(z.len2(), cfg.amplitude, cfg.smoothness);
    (z * factor, dz * factor)
}

fn noise(cfg: &Cfg, z: Vector3<f32>, dz: f32) -> (Vector3<f32>, f32) {
    let scale = cfg.roughness;
    (z * scale, dz * scale.abs())
}

fn get_height(z: Vector3<f32>, dz: f32, offset: Vector3<f32>) -> (Vector3<f32>, f32) {
    (z + offset, dz + 1.0)
}

fn noise_one(cfg: &Cfg, z: Vector3<f32>, dz: f32, offset_value: Vector3<f32>) -> (Vector3<f32>, f32) {
    let z = get_noise(cfg, z);
    let (z, dz) = lerp(cfg, z, dz);
    let (z, dz) = noise(cfg, z, dz);
    let (z, dz) = get_height(z, dz, offset_value);
    (z, dz)
}

pub fn noise_final(cfg: &Cfg, offset: Vector3<f32>) -> f32 {
    let mut z = offset;
    let mut dz = 1.0;
    let mut n = cfg.max_iters.max(1);
    while z.len2() < cfg.heightOffset && n > 0 {
        let (new_z, new_dz) = noise_one(cfg, z, dz, offset);
        z = new_z;
        dz = new_dz;
        n -= 1;
    }
    return z.len2().sqrt() / dz;
}

pub fn de(x: f32, y: f32, z: f32) -> f32 {
    noise_final(
        &Cfg {
            seed: 10,
            
            octaves: 7.0,
            amplitude: 70.125,
            smoothness: 1.0,
            heightOffset: (1 << 10) as f32,
            roughness: -0.53,
            max_iters: 1 << 8,
        },
        Vector3::new(x, y, z),
    )
}
