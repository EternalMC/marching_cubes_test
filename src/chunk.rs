use isosurface::linear_hashed_marching_cubes::LinearHashedMarchingCubes;
use isosurface::source::CentralDifference;
use isosurface::source::Source;

const LOD: usize = 6;

pub fn gen_mbox() -> (Vec<f32>, Vec<u32>) {
    let mut pos_normal = Vec::new();
    let mut index = Vec::new();

    let source = CentralDifference::new(Box::new(Mandelbox {}));
    let mut linear_hashed_marching_cubes = LinearHashedMarchingCubes::new(LOD);
    linear_hashed_marching_cubes.extract_with_normals(&source, &mut pos_normal, &mut index);

    return (pos_normal, index);
}
