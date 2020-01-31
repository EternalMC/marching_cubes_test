use number::Vector3;
use number::clamp;

pub struct Cfg {
    octaves: f32,
    amplitude: f32,
    smoothness: f32,
    heightOffset: f32,
    roughness: f32,
    max_iters: u32,
}

fn boxfold(cfg: &Cfg, z: Vector3<f32>) -> Vector3<f32> {
    z.clamp(-cfg.octaves, cfg.octaves) * 2.0 - z
}

fn spherefold(cfg: &Cfg, z: Vector3<f32>, dz: f32) -> (Vector3<f32>, f32) {
    let factor = cfg.smoothness / clamp(z.len2(), cfg.amplitude, cfg.smoothness);
    (z * factor, dz * factor)
}

fn scale(cfg: &Cfg, z: Vector3<f32>, dz: f32) -> (Vector3<f32>, f32) {
    let scale = cfg.roughness;
    (z * roughness, dz * roughness.abs())
}

fn offset(z: Vector3<f32>, dz: f32, offset: Vector3<f32>) -> (Vector3<f32>, f32) {
    (z + offset, dz + 1.0)
}

fn noise_one(cfg: &Cfg, z: Vector3<f32>, dz: f32, offset_value: Vector3<f32>) -> (Vector3<f32>, f32) {
    let z = boxfold(cfg, z);
    let (z, dz) = spherefold(cfg, z, dz);
    let (z, dz) = scale(cfg, z, dz);
    let (z, dz) = offset(z, dz, offset_value);
    (z, dz)
}

pub fn noise(cfg: &Cfg, offset: Vector3<f32>) -> f32 {
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
    noise(
        &Cfg {
            octaves: 1.0,
            amplitude: 0.125,
            smoothness: 1.0,
            heightOffset: (1 << 10) as f32,
            roughness: -2.0,
            max_iters: 1 << 8,
        },
        Vector3::new(x, y, z),
    )
}
