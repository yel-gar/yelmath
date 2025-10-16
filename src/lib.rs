mod vectors;
mod errors;
mod matrices;
mod types;
mod util;

#[cfg(test)]
mod tests {
    use crate::vectors::*;
    use crate::matrices::*;

    fn f32_eq(f1: f32, f2: f32) -> bool {
        (f1 - f2).abs() <= f32::EPSILON
    }

    fn f64_eq(f1: f64, f2: f64) -> bool {
        (f1 - f2).abs() <= f64::EPSILON
    }

    #[test]
    fn vec_zero() {
        let vec2d = Vector2D::<f32>::zero();
        assert_eq!(vec2d.x, 0.0);
        assert_eq!(vec2d.y, 0.0);
        
        let vec3d = Vector3D::<f32>::zero();
        assert_eq!(vec3d.x, 0.0);
        assert_eq!(vec3d.y, 0.0);
        assert_eq!(vec3d.z, 0.0);
        
        let vec4d = Vector4D::<f32>::zero();
        assert_eq!(vec4d.x, 0.0);
        assert_eq!(vec4d.y, 0.0);
        assert_eq!(vec4d.z, 0.0);
        assert_eq!(vec4d.w, 0.0);
    }
    
    #[test]
    fn vec_from_slice_ok() -> Result<(), Box<dyn std::error::Error>> {
        let slice2d = [1, 2];
        let slice3d = [3, 4, 5];
        let slice4d = [6, 7, 8, 9];
        
        let v2d = Vector2D::from_slice(&slice2d)?;
        let v3d = Vector3D::from_slice(&slice3d)?;
        let v4d = Vector4D::from_slice(&slice4d)?;
        
        assert_eq!((v2d.x, v2d.y), (1, 2));
        assert_eq!((v3d.x, v3d.y, v3d.z), (3, 4, 5));
        assert_eq!((v4d.x, v4d.y, v4d.z, v4d.w), (6, 7, 8, 9));
        
        Ok(())
    }
    
    #[test]
    fn vec_from_slice_err() {
        let slice2 = [1, 2];
        let slice3 = [3, 4, 5];
        
        let v2dr = Vector2D::from_slice(&slice3);
        let v3dr = Vector3D::from_slice(&slice2);
        let v4dr = Vector4D::from_slice(&slice2);
        
        assert!(v2dr.is_err());
        assert!(v3dr.is_err());
        assert!(v4dr.is_err());
    }

    #[test]
    fn vec_from_vec_ok() -> Result<(), Box<dyn std::error::Error>> {
        let vec2d = vec![1, 2];
        let vec3d = vec![3, 4, 5];
        let vec4d = vec![6, 7, 8, 9];

        let v2d = Vector2D::from_vec(&vec2d)?;
        let v3d = Vector3D::from_vec(&vec3d)?;
        let v4d = Vector4D::from_vec(&vec4d)?;

        assert_eq!((v2d.x, v2d.y), (1, 2));
        assert_eq!((v3d.x, v3d.y, v3d.z), (3, 4, 5));
        assert_eq!((v4d.x, v4d.y, v4d.z, v4d.w), (6, 7, 8, 9));

        Ok(())
    }

    #[test]
    fn vec_from_vec_err() {
        let vec2 = vec![1, 2];
        let vec3 = vec![3, 4, 5];

        let v2dr = Vector2D::from_vec(&vec3);
        let v3dr = Vector3D::from_vec(&vec2);
        let v4dr = Vector4D::from_vec(&vec2);

        assert!(v2dr.is_err());
        assert!(v3dr.is_err());
        assert!(v4dr.is_err());
    }

    #[test]
    fn vec_invert() {
        let v2d = Vector2D::new(5, -2);
        let v2dr = Vector2D::new(-5, 2);

        let v3d = Vector3D::new(5, -2, 4);
        let v3dr = Vector3D::new(-5, 2, -4);

        let v4d = Vector4D::new(5, -2, 4, 900);
        let v4dr = Vector4D::new(-5, 2, -4, -900);

        assert_eq!(v2d.invert(), v2dr);
        assert_eq!(v3d.invert(), v3dr);
        assert_eq!(v4d.invert(), v4dr);
    }

    #[test]
    fn vec_normalize() {
        let v2d = Vector2D::new(5, -2);
        let v2drf32 = Vector2D::new(1.0f32, -0.4);
        let v2drf64 = Vector2D::new(1.0f64, -0.4);

        let v3d = Vector3D::new(5, -2, 4);
        let v3drf32 = Vector3D::new(1.0f32, -0.4, 0.8);
        let v3drf64 = Vector3D::new(1.0f64, -0.4, 0.8);

        let v4d = Vector4D::new(5, -2, 4, -1);
        let v4drf32 = Vector4D::new(1.0f32, -0.4, 0.8, -0.2);
        let v4drf64 = Vector4D::new(1.0f64, -0.4, 0.8, -0.2);

        assert_eq!(v2d.normalize_f32(), v2drf32);
        assert_eq!(v2d.normalize_f64(), v2drf64);
        assert_eq!(v3d.normalize_f32(), v3drf32);
        assert_eq!(v3d.normalize_f64(), v3drf64);
        assert_eq!(v4d.normalize_f32(), v4drf32);
        assert_eq!(v4d.normalize_f64(), v4drf64);
    }

    #[test]
    fn vec_precision_eq() {
        let v2_1 = Vector2D::new(1.0, 0.0);
        let v2_2 = Vector2D::new(1.005, 0.0);
        assert!(v2_1.precision_eq(&v2_2, 0.01));
        assert!(!v2_1.precision_eq(&v2_2, 0.0001));

        let v3_1 = Vector3D::new(1.0, 0.0, 0.);
        let v3_2 = Vector3D::new(1.005, 0.0, 0.);
        assert!(v3_1.precision_eq(&v3_2, 0.01));
        assert!(!v3_1.precision_eq(&v3_2, 0.0001));

        let v4_1 = Vector4D::new(1.0, 0.0, 0., 5.);
        let v4_2 = Vector4D::new(1.005, 0.0, 0., 4.999);
        assert!(v4_1.precision_eq(&v4_2, 0.01));
        assert!(!v4_1.precision_eq(&v4_2, 0.0001))
    }
    
    #[test]
    fn vec_add() {
        let v2d1 = Vector2D::new(2.0, 4.0);
        let v2d2 = Vector2D::new(-7.0, 2.0);
        let v2dr = Vector2D::new(-5.0, 6.0);

        let v3d1 = Vector3D::new(2.0, 4.0, 5.2);
        let v3d2 = Vector3D::new(-7.0, 2.0, 0.6);
        let v3dr = Vector3D::new(-5.0, 6.0, 5.8);

        let v4d1 = Vector4D::new(2.0, 4.0, 5.2, 0.0);
        let v4d2 = Vector4D::new(-7.0, 2.0, 0.6, -15.2);
        let v4dr = Vector4D::new(-5.0, 6.0, 5.8, -15.2);
        
        assert_eq!(v2d1.add(&v2d2), v2dr);
        assert_eq!(v3d1.add(&v3d2), v3dr);
        assert_eq!(v4d1.add(&v4d2), v4dr);
    }
    
    #[test]
    fn vec_sub() {
        let v2d1 = Vector2D::new(2, 4);
        let v2d2 = Vector2D::new(-7, 2);
        let v2dr = Vector2D::new(9, 2);

        let v3d1 = Vector3D::new(2, 4, 5);
        let v3d2 = Vector3D::new(-7, 2, 0);
        let v3dr = Vector3D::new(9, 2, 5);

        let v4d1 = Vector4D::new(2, 4, 5, 0);
        let v4d2 = Vector4D::new(-7, 2, 0, -15);
        let v4dr = Vector4D::new(9, 2, 5, 15);

        assert_eq!(v2d1.sub(&v2d2), v2dr);
        assert_eq!(v3d1.sub(&v3d2), v3dr);
        assert_eq!(v4d1.sub(&v4d2), v4dr);
    }
    
    #[test]
    fn vec_dot() {
        let v2d1 = Vector2D::new(2, 1);
        let v2d2 = Vector2D::new(-7, 2);
        let v2dr = -12;

        let v3d1 = Vector3D::new(2, 1, 5);
        let v3d2 = Vector3D::new(-7, 2, 5);
        let v3dr = 13;

        let v4d1 = Vector4D::new(2, 1, 5, -2);
        let v4d2 = Vector4D::new(-7, 2, 5, 6);
        let v4dr = 1;

        assert_eq!(v2d1.dot(&v2d2), v2dr);
        assert_eq!(v3d1.dot(&v3d2), v3dr);
        assert_eq!(v4d1.dot(&v4d2), v4dr);
    }
    
    #[test]
    fn vec_scale() {
        let factor = 2;

        let v2d = Vector2D::new(2, 4);
        let v2dr = Vector2D::new(4, 8);

        let v3d = Vector3D::new(2, 4, 0);
        let v3dr = Vector3D::new(4, 8, 0);

        let v4d = Vector4D::new(2, 4, 0, -3);
        let v4dr = Vector4D::new(4, 8, 0, -6);

        assert_eq!(v2d.scale(factor), v2dr);
        assert_eq!(v3d.scale(factor), v3dr);
        assert_eq!(v4d.scale(factor), v4dr);
    }
    
    #[test]
    fn vec_magnitude() {
        let v2d = Vector2D::new(3, 4);
        let v2drf32 = 5f32;
        let v2drf64 = 5f64;

        let v3d = Vector3D::new(2, -3, 6);
        let v3drf32 = 7f32;
        let v3drf64 = 7f64;

        let v4d = Vector4D::new(2, -3, 4, 6);
        let v4drf32 = 65f32.sqrt();
        let v4drf64 = 65f64.sqrt();

        assert!(f32_eq(v2d.magnitude_f32(), v2drf32) && f64_eq(v2d.magnitude_f64(), v2drf64));
        assert!(f32_eq(v3d.magnitude_f32(), v3drf32) && f64_eq(v3d.magnitude_f64(), v3drf64));
        assert!(f32_eq(v4d.magnitude_f32(), v4drf32) && f64_eq(v4d.magnitude_f64(), v4drf64));
    }

    #[test]
    fn vec_cross() {
        let v3_1 = Vector3D::new(1, 2, 3);
        let v3_2 = Vector3D::new(2, 1, -2);
        let v3r = Vector3D::new(-7, 8, -3);

        assert_eq!(v3_1.cross(&v3_2), v3r);
    }
    
    #[test]
    fn mat_zero() {
        let m3 = Matrix3x3::new([[0; 3]; 3]);
        let m4 = Matrix3x3::new([[0; 3]; 3]);

        assert_eq!(m3, Matrix3x3::<i32>::zero());
        assert_eq!(m4, Matrix3x3::<i32>::zero());
    }
    
    #[test]
    fn mat_dimensions() {
        assert_eq!(Matrix3x3::<i32>::zero().dimensions(), (3, 3));
        assert_eq!(Matrix4x4::<i32>::zero().dimensions(), (4, 4));
    }
    
    #[test]
    fn mat_identity() {
        let m3 = Matrix3x3::new(
            [
                [1, 0, 0],
                [0, 1, 0],
                [0, 0, 1],
            ]
        );
        let m4 = Matrix4x4::new(
            [
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1]
            ]
        );

        assert_eq!(Matrix3x3::<i32>::identity(), m3);
        assert_eq!(Matrix4x4::<i32>::identity(), m4);
    }
    
    #[test]
    fn mat_from_func() {
        let m3 = Matrix3x3::from_func(|i, j| {
            (i * 3 + j) as i32
        });
        let m3e = Matrix3x3::new(
            [
                [0, 1, 2],
                [3, 4, 5],
                [6, 7, 8],
            ]
        );

        let m4 = Matrix4x4::from_func(|i, j| {
            (i * 4 + j) as i32
        });
        let m4e = Matrix4x4::new(
            [
                [0, 1, 2, 3],
                [4, 5, 6, 7],
                [8, 9, 10, 11],
                [12, 13, 14, 15]
            ]
        );

        assert_eq!(m3, m3e);
        assert_eq!(m4, m4e);
    }

    #[test]
    fn mat_precision_eq() {
        let m3_1 = Matrix3x3::new([
            [1., 2., 3.],
            [2., 8., -4.],
            [5., 10., 2.]
        ]);
        let m3_2 = Matrix3x3::new([
            [1., 2., 3.],
            [2., 8.1, -4.],
            [5., 10., 2.]
        ]);
        let m4_1 = Matrix4x4::new([
            [1., 2., 3., 2.],
            [2., 8., -4., 3.],
            [5., 10., 2., 0.],
            [8., 0., 2., 4.]
        ]);
        let m4_2 = Matrix4x4::new([
            [1., 2., 3., 2.],
            [2., 8.3, -4., 3.],
            [5., 10., 2., 0.],
            [8., 0., 2., 4.]
        ]);

        assert!(m3_1.precision_eq(&m3_2, 1.));
        assert!(!m3_1.precision_eq(&m3_2, 0.01));
        assert!(m4_1.precision_eq(&m4_2, 1.));
        assert!(!m4_1.precision_eq(&m4_2, 0.01));
    }
    
    #[test]
    fn mat_get_val() {
        let m3 = Matrix3x3::new(
            [
                [5, 2, 6],
                [7, 9, -1],
                [2, 5, 1]
            ]
        );
        let m4 = Matrix4x4::new([
            [0, 2, -1, 4],
            [7, 3, 4, 10],
            [5, 1, -3, -2],
            [-4, 1, 9, -9]
        ]);

        assert_eq!(m3.get_val(0, 0), 5);
        assert_eq!(m3.get_val(1, 1), 9);
        assert_eq!(m3.get_val(2, 2), 1);
        assert_eq!(m4.get_val(2, 3), -2);
        assert_eq!(m4.get_val(0, 0), 0);
        assert_eq!(m4.get_val(3, 1), 1);
    }
    
    #[test]
    fn mat_determinant() {
        assert_eq!(Matrix3x3::<i32>::identity().determinant(), 1);
        assert_eq!(Matrix4x4::<i32>::identity().determinant(), 1);

        let m3 = Matrix3x3::new([
            [2, -3, 1],
            [2, 0, -1],
            [1, 4, 5]
        ]);
        let m3_det = 49;

        let m4 = Matrix4x4::new([
            [1, 3, 1, 4],
            [3, 9, 5, 15],
            [0, 2, 1, 1],
            [0, 4, 2, 3]
        ]);
        let m4_det = -4;

        assert_eq!(m3.determinant(), m3_det);
        assert_eq!(m4.determinant(), m4_det);
    }
    
    #[test]
    fn mat_minor() {
        let m3 = Matrix3x3::new([
            [2, -3, 1],
            [2, 0, -1],
            [1, 4, 5]
        ]);
        assert_eq!(m3.minor(0, 0), 4);
        assert_eq!(m3.minor(1, 0), -19);
        assert_eq!(m3.minor(2, 1), -4);

        let m4 = Matrix4x4::new([
            [1, 2, -3, 1],
            [7, 2, 0, -1],
            [-5, 1, 4, 5],
            [10, 2, -4, 2]
        ]);
        assert_eq!(m4.minor(3, 0), 49);
    }
    
    #[test]
    fn mat_transposed() {
        assert_eq!(Matrix3x3::<i32>::identity().transposed(), Matrix3x3::<i32>::identity());
        assert_eq!(Matrix4x4::<i32>::identity().transposed(), Matrix4x4::<i32>::identity());

        let m3 = Matrix3x3::new([
            [2, -3, 1],
            [2, 0, -1],
            [1, 4, 5]
        ]);
        let m3e = Matrix3x3::new([
            [2, 2, 1],
            [-3, 0, 4],
            [1, -1, 5]
        ]);

        let m4 = Matrix4x4::new([
            [1, 2, -3, 1],
            [7, 2, 0, -1],
            [-5, 1, 4, 5],
            [10, 2, -4, 2]
        ]);
        let m4e = Matrix4x4::new([
            [1, 7, -5, 10],
            [2, 2, 1, 2],
            [-3, 0, 4, -4],
            [1, -1, 5, 2]
        ]);

        assert_eq!(m3.transposed(), m3e);
        assert_eq!(m4.transposed(), m4e);
    }
    
    #[test]
    fn mat_add() {
        let m3_1 = Matrix3x3::new([
            [1, 2, 3],
            [3, 2, 1],
            [8, 0, -5]
        ]);
        let m3_2 = Matrix3x3::new([
            [0, 2, 1],
            [1, 2, 3],
            [-5, 4, 8]
        ]);
        let m3r = Matrix3x3::new([
            [1, 4, 4],
            [4, 4, 4],
            [3, 4, 3]
        ]);

        let m4_1 = Matrix4x4::new([
            [0, 1, 2, 3],
            [4, 5, 6, 0],
            [1, 2, 3, 4],
            [5, 0, 1, 2]
        ]);
        let m4_2 = Matrix4x4::new([
            [9, 8, 7, 6],
            [5, 4, 3, 2],
            [1, 0, -1, -2],
            [-3, -4, -5, -6]
        ]);
        let m4r = Matrix4x4::new([
            [9, 9, 9, 9],
            [9, 9, 9, 2],
            [2, 2, 2, 2],
            [2, -4, -4, -4]
        ]);

        assert_eq!(m3_1.add(&m3_2), m3r);
        assert_eq!(m4_1.add(&m4_2), m4r);
    }
    
    #[test]
    fn mat_sub() {
        let m3_1 = Matrix3x3::new([
            [1, 2, 3],
            [3, 2, 1],
            [8, 0, -5]
        ]);
        let m3_2 = Matrix3x3::new([
            [0, 2, 1],
            [1, 2, 3],
            [-5, 4, 8]
        ]);
        let m3r = Matrix3x3::new([
            [1, 0, 2],
            [2, 0, -2],
            [13, -4, -13]
        ]);

        let m4_1 = Matrix4x4::new([
            [0, 1, 2, 3],
            [4, 5, 6, 0],
            [1, 2, 3, 4],
            [5, 0, 1, 2]
        ]);
        let m4_2 = Matrix4x4::new([
            [9, 8, 7, 6],
            [5, 4, 3, 2],
            [1, 0, -1, -2],
            [-3, -4, -5, -6]
        ]);
        let m4r = Matrix4x4::new([
            [-9, -7, -5, -3],
            [-1, 1, 3, -2],
            [0, 2, 4, 6],
            [8, 4, 6, 8]
        ]);

        assert_eq!(m3_1.sub(&m3_2), m3r);
        assert_eq!(m4_1.sub(&m4_2), m4r);
    }
    
    #[test]
    fn mat_cross() {
        let m3_1 = Matrix3x3::new([
            [1, 2, 3],
            [3, 2, 1],
            [8, 0, -5]
        ]);
        let m3_2 = Matrix3x3::new([
            [0, 2, 1],
            [1, 2, 3],
            [-5, 4, 8]
        ]);
        let m3r = Matrix3x3::new([
            [-13, 18, 31],
            [-3, 14, 17],
            [25, -4, -32]
        ]);

        let m4_1 = Matrix4x4::new([
            [0, 1, 2, 3],
            [4, 5, 6, 0],
            [1, 0, 3, 4],
            [5, 0, 1, 2]
        ]);
        let m4_2 = Matrix4x4::new([
            [3, 3, 0, 3],
            [-2, 1, 0, 2],
            [1, 0, -1, -2],
            [-3, -4, 10, 0]
        ]);
        let m4r = Matrix4x4::new([
            [-9, -11, 28, -2],
            [8, 17, -6, 10],
            [-6, -13, 37, -3],
            [10, 7, 19, 13]
        ]);

        assert_eq!(m3_1.cross(&m3_2), m3r);
        assert_eq!(m4_1.cross(&m4_2), m4r);
    }
    
    #[test]
    fn mat_inverse() {
        let i3 = Matrix3x3::<f32>::identity();
        let i4 = Matrix4x4::<f32>::identity();
        assert_eq!(Matrix3x3::<f32>::identity().inverse().unwrap(), i3);
        assert_eq!(Matrix4x4::<f32>::identity().inverse().unwrap(), i4);

        let m3 = Matrix3x3::new([
            [1., 2., 3.],
            [2., 8., -4.],
            [5., 10., 2.]
        ]);
        let m4 = Matrix4x4::new([
            [1., 2., 3., 2.],
            [2., 8., -4., 3.],
            [5., 10., 2., 0.],
            [8., 0., 2., 4.]
        ]);
        assert!(m3.cross(&m3.inverse().unwrap()).precision_eq(&i3, 0.001));
        assert!(m4.cross(&m4.inverse().unwrap()).precision_eq(&i4, 0.001));
    }
    
    #[test]
    fn mat_mul_vec() {
        let m3 = Matrix3x3::new([
            [1, 2, 0],
            [-2, 5, 1],
            [6, 4, 0]
        ]);
        let v3 = Vector3D::new(2, -3, 3);
        let r3 = Vector3D::new(-4, -16, 0);

        let m4 = Matrix4x4::new([
            [1, 0, 2, 0],
            [7, 1, -1, -3],
            [5, 0, 2, -1],
            [3, 3, 4, 2]
        ]);
        let v4 = Vector4D::new(0, 1, -2, 2);
        let r4 = Vector4D::new(-4, -3, -6, -1);

        assert_eq!(m3.mul_vec(&v3), r3);
        assert_eq!(m4.mul_vec(&v4), r4);
    }
}
