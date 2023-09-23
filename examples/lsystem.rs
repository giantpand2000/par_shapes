use par_shapes::Mesh;

fn main() {
    // export a tree-like shape
    let program = "\
      sx 2 sy 2 \
      ry 90 rx 90 \
      shape tube rx 15  call rlimb rx -15 \
      shape tube rx -15 call llimb rx 15 \
      shape tube ry 15  call rlimb ry -15 \
      shape tube ry 15  call llimb ry -15 \
      rule rlimb \
          sx 0.925 sy 0.925 tz 1 rx 1.2 \
          call rlimb2 \
      rule rlimb2.1 \
          shape connect \
          call rlimb \
      rule rlimb2.1 \
          rx 15  shape tube call rlimb rx -15 \
          rx -15 shape tube call llimb rx 15 \
      rule rlimb.1 \
          call llimb \
      rule llimb.1 \
          call rlimb \
      rule llimb.10 \
          sx 0.925 sy 0.925 \
          tz 1 \
          rx -1.2 \
          shape connect \
          call llimb";
    let o = [0., 0., 0.];
    let j = [0., 1., 0.];
    let mut mesh = unsafe { Mesh::create_lsystem(program, 5, 60) };
    let disk = Mesh::create_disk(10., 30, o, j);
    mesh.merge_from(disk);
    mesh.export("build/lsystem.obj");
}
