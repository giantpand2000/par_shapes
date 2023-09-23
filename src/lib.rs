pub mod sys;

#[repr(transparent)]
pub struct Mesh {
    handle: *mut sys::par_shapes_mesh,
}

impl Drop for Mesh {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            sys::par_shapes_free_mesh(self.handle);
        }
    }
}

impl core::ops::Deref for Mesh {
    type Target = sys::par_shapes_mesh;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.handle }
    }
}

impl core::ops::DerefMut for Mesh {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.handle }
    }
}

impl AsRef<sys::par_shapes_mesh> for Mesh {
    #[inline]
    fn as_ref(&self) -> &sys::par_shapes_mesh {
        unsafe { &*self.handle }
    }
}

impl AsMut<sys::par_shapes_mesh> for Mesh {
    #[inline]
    fn as_mut(&mut self) -> &mut sys::par_shapes_mesh {
        unsafe { &mut *self.handle }
    }
}

impl Clone for Mesh {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let new_handle = sys::par_shapes_clone(self.handle, core::ptr::null_mut());
            Self::from_raw(new_handle)
        }
    }
}

impl Mesh {
    #[inline]
    pub unsafe fn from_raw(handle: *mut sys::par_shapes_mesh) -> Self {
        Self { handle }
    }

    #[inline]
    pub fn into_raw(self) -> *mut sys::par_shapes_mesh {
        let handle = self.handle;
        core::mem::forget(self);
        handle
    }

    #[inline]
    pub fn clone_to(&self, target: &mut Self) {
        unsafe {
            target.handle = sys::par_shapes_clone(self.handle, target.handle);
        }
    }

    #[inline]
    pub fn points_len(&self) -> usize {
        (**self).npoints as usize
    }

    #[inline]
    pub fn triangles_len(&self) -> usize {
        (**self).ntriangles as usize
    }

    #[inline]
    #[doc(alias = "vertices")]
    pub fn points(&self) -> &[f32] {
        unsafe {
            core::slice::from_raw_parts((*self.handle).points, 3 * (*self.handle).npoints as usize)
        }
    }

    #[inline]
    #[doc(alias = "indices")]
    pub fn triangles(&self) -> &[u16] {
        unsafe {
            core::slice::from_raw_parts(
                (*self.handle).triangles,
                3 * (*self.handle).ntriangles as usize,
            )
        }
    }

    #[inline]
    pub fn normals(&self) -> Option<&[f32]> {
        unsafe {
            if (*self.handle).normals.is_null() {
                None
            } else {
                Some(core::slice::from_raw_parts(
                    (*self.handle).normals,
                    3 * (*self.handle).npoints as usize,
                ))
            }
        }
    }

    #[inline]
    #[doc(alias = "uv")]
    pub fn tcoords(&self) -> Option<&[f32]> {
        unsafe {
            if (*self.handle).tcoords.is_null() {
                None
            } else {
                Some(core::slice::from_raw_parts(
                    (*self.handle).tcoords,
                    2 * (*self.handle).npoints as usize,
                ))
            }
        }
    }
}

/// Generators
impl Mesh {
    /// Instance a cylinder that sits on the Z=0 plane using the given tessellation
    /// levels across the UV domain.  Think of `slices` like a number of pizza
    /// slices, and `stacks` like a number of stacked rings.  Height and radius are
    /// both 1.0, but they can easily be changed with par_shapes_scale.
    #[inline]
    pub fn create_cylinder(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_cylinder(slices, stacks)) }
    }

    /// Cone is similar to cylinder but the radius diminishes to zero as Z increases.
    /// Again, height and radius are 1.0, but can be changed with par_shapes_scale.
    #[inline]
    pub fn create_cone(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_cone(slices, stacks)) }
    }

    /// Create a disk of radius 1.0 with texture coordinates and normals by squashing
    /// a cone flat on the Z=0 plane.
    #[inline]
    pub fn create_parametric_disk(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_parametric_disk(slices, stacks)) }
    }

    /// Create a donut that sits on the Z=0 plane with the specified inner radius.
    /// The outer radius can be controlled with par_shapes_scale.
    #[inline]
    pub fn create_torus(slices: i32, stacks: i32, radius: f32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_torus(slices, stacks, radius)) }
    }

    /// Create a sphere with texture coordinates and small triangles near the poles.
    #[inline]
    pub fn create_parametric_sphere(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_parametric_sphere(slices, stacks)) }
    }

    /// Approximate a sphere with a subdivided icosahedron, which produces a nice
    /// distribution of triangles, but no texture coordinates.  Each subdivision
    /// level scales the number of triangles by four, so use a very low number.
    #[inline]
    pub fn create_subdivided_sphere(nsubdivisions: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_subdivided_sphere(nsubdivisions)) }
    }

    /// More parametric surfaces.
    #[inline]
    pub fn create_klein_bottle(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_klein_bottle(slices, stacks)) }
    }

    #[inline]
    pub fn create_trefoil_knot(slices: i32, stacks: i32, radius: f32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_trefoil_knot(slices, stacks, radius)) }
    }

    #[inline]
    pub fn create_hemisphere(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_hemisphere(slices, stacks)) }
    }

    #[inline]
    pub fn create_plane(slices: i32, stacks: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_plane(slices, stacks)) }
    }

    /// Create a parametric surface from a callback function that consumes a 2D
    /// point in `[0., 1.]` and produces a 3D point.
    #[inline]
    pub fn create_parametric<F: FnMut(&[f32; 2], &mut [f32; 3])>(
        slices: i32,
        stacks: i32,
        mut f: F,
    ) -> Self {
        unsafe extern "C" fn func_raw<F: FnMut(&[f32; 2], &mut [f32; 3])>(
            input: *const f32,
            output: *mut f32,
            f_ptr: *mut core::ffi::c_void,
        ) {
            use core::slice;

            let f = &mut *(f_ptr as *mut F);
            let input = TryFrom::try_from(slice::from_raw_parts(input, 2)).unwrap();
            let output = TryFrom::try_from(slice::from_raw_parts_mut(output, 3)).unwrap();
            (f)(input, output);
        }
        unsafe {
            Self::from_raw(sys::par_shapes_create_parametric(
                Some(func_raw::<F>),
                slices,
                stacks,
                &mut f as *mut _ as *mut _,
            ))
        }
    }

    /// Generate points for a 20-sided polyhedron that fits in the unit sphere.
    /// Texture coordinates and normals are not generated.
    #[inline]
    pub fn create_icosahedron() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_icosahedron()) }
    }

    /// Generate points for a 12-sided polyhedron that fits in the unit sphere.
    /// Again, texture coordinates and normals are not generated.
    #[inline]
    pub fn create_dodecahedron() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_dodecahedron()) }
    }

    /// More platonic solids.
    #[inline]
    pub fn create_octahedron() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_octahedron()) }
    }
    #[inline]
    pub fn create_tetrahedron() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_tetrahedron()) }
    }
    #[inline]
    pub fn create_cube() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_cube()) }
    }

    /// Generate an orientable disk shape in 3-space.  Does not include normals or
    /// texture coordinates.
    #[inline]
    pub fn create_disk(
        radius: f32,
        slices: i32,
        center: impl Into<[f32; 3]>,
        normal: impl Into<[f32; 3]>,
    ) -> Self {
        let center: [f32; 3] = center.into();
        let normal: [f32; 3] = normal.into();
        unsafe {
            Self::from_raw(sys::par_shapes_create_disk(
                radius,
                slices,
                center.as_ptr(),
                normal.as_ptr(),
            ))
        }
    }

    /// Create an empty shape.  Useful for building scenes with merge_and_free.
    #[inline]
    pub fn create_empty() -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_empty()) }
    }

    /// Generate a rock shape that sits on the Y=0 plane, and sinks into it a bit.
    /// This includes smooth normals but no texture coordinates.  Each subdivision
    /// level scales the number of triangles by four, so use a very low number.
    #[inline]
    pub fn create_rock(seed: i32, nsubdivisions: i32) -> Self {
        unsafe { Self::from_raw(sys::par_shapes_create_rock(seed, nsubdivisions)) }
    }

    /// Create trees or vegetation by executing a recursive turtle graphics program.
    /// The program is a list of command-argument pairs.  See the unit test for
    /// an example.  Texture coordinates and normals are not generated. <br/>
    ///
    /// **Warning**: Only for people who has read the C source code! Program is hard to get to be correct.
    /// Bad program leads panic without any warning, or cannot stop until memory exhausted.
    ///
    /// # Examples
    ///
    /// ```
    /// // export a tree-like shape
    /// let program = "\
    ///      sx 2 sy 2 \
    ///      ry 90 rx 90 \
    ///      shape tube rx 15  call rlimb rx -15 \
    ///      shape tube rx -15 call llimb rx 15 \
    ///      shape tube ry 15  call rlimb ry -15 \
    ///      shape tube ry 15  call llimb ry -15 \
    ///      rule rlimb \
    ///          sx 0.925 sy 0.925 tz 1 rx 1.2 \
    ///          call rlimb2 \
    ///      rule rlimb2.1 \
    ///          shape connect \
    ///          call rlimb \
    ///      rule rlimb2.1 \
    ///          rx 15  shape tube call rlimb rx -15 \
    ///          rx -15 shape tube call llimb rx 15 \
    ///      rule rlimb.1 \
    ///          call llimb \
    ///      rule llimb.1 \
    ///          call rlimb \
    ///      rule llimb.10 \
    ///          sx 0.925 sy 0.925 \
    ///          tz 1 \
    ///          rx -1.2 \
    ///          shape connect \
    ///          call llimb";
    /// let o = [0., 0., 0.];
    /// let j = [0., 1., 0.];
    /// let mut mesh = unsafe { Mesh::create_lsystem(program, 5, 60) };
    /// let disk = Mesh::create_disk(10., 30, o, j);
    /// mesh.merge_from(disk);
    /// mesh.export("build/lsystem.obj");
    /// ```
    #[inline]
    pub unsafe fn create_lsystem(program: &str, slices: i32, maxdepth: i32) -> Self {
        let program = std::ffi::CString::new(program).unwrap();
        unsafe {
            Self::from_raw(sys::par_shapes_create_lsystem(
                program.as_ptr(),
                slices,
                maxdepth,
            ))
        }
    }
}

/// Queries
impl Mesh {
    /// Dump out a text file conforming to the venerable OBJ format.
    #[inline]
    pub fn export<P: AsRef<std::path::Path> + ?Sized>(&self, objfile: &P) {
        let objfile = std::ffi::CString::new(objfile.as_ref().to_str().unwrap()).unwrap();
        unsafe { sys::par_shapes_export(self.handle, objfile.as_ptr()) }
    }

    /// Take a pointer to 6 floats and set them to min xyz, max xyz.
    #[doc(alias = "bounds")]
    #[inline]
    pub fn compute_aabb(&self, aabb: &mut [f32; 6]) {
        unsafe { sys::par_shapes_compute_aabb(self.handle, aabb.as_mut_ptr()) }
    }
}

/// Transformations
impl Mesh {
    #[inline]
    pub fn merge(&mut self, src: &Self) {
        unsafe { sys::par_shapes_merge(self.handle, src.handle) }
    }
    #[inline]
    pub fn translate(&mut self, vector: impl Into<[f32; 3]>) {
        let [x, y, z] = vector.into();
        unsafe { sys::par_shapes_translate(self.handle, x, y, z) }
    }
    #[inline]
    pub fn rotate(&mut self, radians: f32, axis: impl Into<[f32; 3]>) {
        let axis: [f32; 3] = axis.into();
        unsafe { sys::par_shapes_rotate(self.handle, radians, axis.as_ptr()) }
    }
    #[inline]
    pub fn scale(&mut self, vector: impl Into<[f32; 3]>) {
        let [x, y, z] = vector.into();
        unsafe { sys::par_shapes_scale(self.handle, x, y, z) }
    }
    #[inline]
    pub fn merge_from(&mut self, src: Self) {
        unsafe {
            let src = src.into_raw();
            sys::par_shapes_merge_and_free(self.handle, src)
        }
    }

    /// Reverse the winding of a run of faces.  Useful when drawing the inside of
    /// a Cornell Box.  Pass 0 for nfaces to reverse every face in the mesh.
    #[inline]
    pub fn invert(&mut self, startface: i32, nfaces: i32) {
        unsafe { sys::par_shapes_invert(self.handle, startface, nfaces) }
    }

    /// Remove all triangles whose area is less than minarea.
    #[inline]
    pub fn remove_degenerate(&mut self, minarea: f32) {
        unsafe { sys::par_shapes_remove_degenerate(self.handle, minarea) }
    }

    /// Dereference the entire index buffer and replace the point list.
    /// This creates an inefficient structure, but is useful for drawing facets.
    /// If `create_indices` is `true`, a trivial "0 1 2 3..." index buffer is generated.
    #[inline]
    pub fn unweld(&mut self, create_indices: bool) {
        unsafe { sys::par_shapes_unweld(self.handle, create_indices) }
    }

    /// Merge colocated verts, build a new index buffer, and return the
    /// optimized mesh.  Epsilon is the maximum distance to consider when
    /// welding vertices. The mapping argument can be null, or a pointer to
    /// npoints integers, which gets filled with the mapping from old vertex
    /// indices to new indices.
    #[inline]
    pub fn weld(&self, epsilon: f32, mapping: Option<&mut [u16]>) -> Self {
        unsafe {
            if let Some(mapping) = mapping {
                assert!(mapping.len() >= (*self.handle).npoints as usize);
                Self::from_raw(sys::par_shapes_weld(
                    self.handle,
                    epsilon,
                    mapping.as_mut_ptr(),
                ))
            } else {
                Self::from_raw(sys::par_shapes_weld(
                    self.handle,
                    epsilon,
                    core::ptr::null_mut(),
                ))
            }
        }
    }

    /// Compute smooth normals by averaging adjacent facet normals.
    #[inline]
    pub fn compute_normals(&mut self) {
        unsafe { sys::par_shapes_compute_normals(self.handle) }
    }
}

/// Global Config
impl Mesh {
    #[inline]
    pub unsafe fn set_epsilon_welded_normals(epsilon: f32) {
        sys::par_shapes_set_epsilon_welded_normals(epsilon)
    }
    #[inline]
    pub unsafe fn set_epsilon_degenerate_sphere(epsilon: f32) {
        sys::par_shapes_set_epsilon_degenerate_sphere(epsilon)
    }
}

/// Advanced
impl Mesh {
    #[inline]
    pub fn compute_welded_normals(&mut self) {
        unsafe { sys::par_shapes__compute_welded_normals(self.handle) }
    }
    #[inline]
    pub fn connect(scene: &mut Self, cylinder: &mut Self, slices: i32) {
        unsafe { sys::par_shapes__connect(scene.handle, cylinder.handle, slices) }
    }
}
