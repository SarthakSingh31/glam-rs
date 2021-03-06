use crate::core::traits::vector::*;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
use crate::BVec3A;
use crate::{BVec3, DVec2, DVec4, IVec2, IVec4, UVec2, UVec4, Vec2, Vec4, XYZ};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{cmp::Ordering, f32, ops::*};

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

macro_rules! impl_vec3_common_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// `[1, 0, 0]`: a unit-length vector pointing along the positive X axis.
        pub const X: Self = Self(Vector3Const::UNIT_X);

        /// `[0, 1, 0]`: a unit-length vector pointing along the positive Y axis.
        pub const Y: Self = Self(Vector3Const::UNIT_Y);

        /// `[0, 0, 1]`: a unit-length vector pointing along the positive Z axis.
        pub const Z: Self = Self(Vector3Const::UNIT_Z);

        /// Creates a new 3D vector.
        #[inline(always)]
        pub fn new(x: $t, y: $t, z: $t) -> Self {
            Self(Vector3::new(x, y, z))
        }

        /// Creates a vector with values `[x: 1.0, y: 0.0, z: 0.0]`.
        #[inline(always)]
        pub const fn unit_x() -> Self {
            Self(Vector3Const::UNIT_X)
        }

        /// Creates a vector with values `[x: 0.0, y: 1.0, z: 0.0]`.
        #[inline(always)]
        pub const fn unit_y() -> Self {
            Self(Vector3Const::UNIT_Y)
        }

        /// Creates a vector with values `[x: 0.0, y: 0.0, z: 1.0]`.
        #[inline(always)]
        pub const fn unit_z() -> Self {
            Self(Vector3Const::UNIT_Z)
        }

        /// Creates a 4D vector from `self` and the given `w` value.
        #[inline(always)]
        pub fn extend(self, w: $t) -> $vec4 {
            // TODO: Optimize?
            $vec4(Vector4::new(self.x, self.y, self.z, w))
        }

        /// Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
        ///
        /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
        #[inline(always)]
        pub fn truncate(self) -> $vec2 {
            $vec2(Vector3::into_xy(self.0))
        }

        /// Returns the dot product result in all elements of the vector
        #[inline(always)]
        #[allow(dead_code)]
        pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
            Self(Vector3::dot_into_vec(self.0, other.0))
        }

        /// Computes the cross product of `self` and `other`.
        #[inline(always)]
        pub fn cross(self, other: Self) -> Self {
            Self(self.0.cross(other.0))
        }

        impl_vecn_common_methods!($t, $vec3, $mask, $inner, Vector3);
    };
}

macro_rules! impl_vec3_common_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// Creates a 3-dimensional vector.
        #[inline(always)]
        pub fn $new(x: $t, y: $t, z: $t) -> $vec3 {
            $vec3::new(x, y, z)
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec3 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let a = self.as_ref();
                fmt.debug_tuple(stringify!($vec3))
                    .field(&a[0])
                    .field(&a[1])
                    .field(&a[2])
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec3 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
            }
        }

        impl From<($vec2, $t)> for $vec3 {
            #[inline(always)]
            fn from((v, z): ($vec2, $t)) -> Self {
                Self::new(v.x, v.y, z)
            }
        }

        impl From<($t, $t, $t)> for $vec3 {
            #[inline(always)]
            fn from(t: ($t, $t, $t)) -> Self {
                Self(Vector3::from_tuple(t))
            }
        }

        impl From<$vec3> for ($t, $t, $t) {
            #[inline(always)]
            fn from(v: $vec3) -> Self {
                v.into_tuple()
            }
        }

        impl From<$vec3> for $vec2 {
            /// Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
            #[inline(always)]
            fn from(v: $vec3) -> Self {
                Self(v.into_xy())
            }
        }

        impl Deref for $vec3 {
            type Target = XYZ<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyz()
            }
        }

        impl DerefMut for $vec3 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xyz()
            }
        }

        impl_vecn_common_traits!($t, 3, $vec3, $inner, Vector3);
    };
}

macro_rules! impl_vec3_signed_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec3_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_methods!($t, $vec3, $mask, $inner, SignedVector3);
    };
}

macro_rules! impl_vec3_float_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec3_signed_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_float_methods!($t, $vec3, $mask, $inner, FloatVector3);

        /// Returns the angle between two vectors, in radians.
        ///
        /// The vectors do not need to be unit length, but this function does
        /// perform a `sqrt`.
        #[inline(always)]
        pub fn angle_between(self, other: Self) -> $t {
            self.0.angle_between(other.0)
        }
    };
}

// implements traits that are common between `Vec3`, `Vec3A` and `Vec4` types.
macro_rules! impl_vec3_float_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl_vec3_common_traits!($t, $new, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_traits!($t, 3, $vec3, $inner, SignedVector3);
    };
}

// implements f32 functionality common between `Vec3` and `Vec3A` types.
macro_rules! impl_f32_vec3 {
    ($new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec3 {
            impl_vec3_float_methods!(f32, $vec2, $vec3, $vec4, $mask, $inner);
            impl_vecn_as_f64!(DVec3, x, y, z);
            impl_vecn_as_i32!(IVec3, x, y, z);
            impl_vecn_as_u32!(UVec3, x, y, z);
        }
        impl_vec3_float_traits!(f32, $new, $vec2, $vec3, $vec4, $mask, $inner);
    };
}

type XYZF32 = XYZ<f32>;

/// A 3-dimensional vector without SIMD support.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3(pub(crate) XYZF32);
impl_f32_vec3!(vec3, Vec2, Vec3, Vec4, BVec3, XYZF32);

/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A(pub(crate) __m128);

/// A 3-dimensional vector.
///
/// This type is 16 byte aligned.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), repr(align(16), C))]
#[cfg_attr(target_arch = "spirv", repr(transparent))]
pub struct Vec3A(pub(crate) XYZF32);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_f32_vec3!(vec3a, Vec2, Vec3A, Vec4, BVec3A, __m128);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_f32_vec3!(vec3a, Vec2, Vec3A, Vec4, BVec3, XYZF32);

impl From<Vec3> for Vec3A {
    #[inline(always)]
    fn from(v: Vec3) -> Self {
        Self(v.0.into())
    }
}

impl From<Vec3A> for Vec3 {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        Self(v.0.into())
    }
}

type XYZF64 = XYZ<f64>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec3(pub(crate) XYZF64);

impl DVec3 {
    impl_vec3_float_methods!(f64, DVec2, DVec3, DVec4, BVec3, XYZF64);
    impl_vecn_as_f32!(Vec3, x, y, z);
    impl_vecn_as_i32!(IVec3, x, y, z);
    impl_vecn_as_u32!(UVec3, x, y, z);
}
impl_vec3_float_traits!(f64, dvec3, DVec2, DVec3, DVec4, BVec3, XYZF64);

type XYZI32 = XYZ<i32>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec3(pub(crate) XYZI32);

impl IVec3 {
    impl_vec3_common_methods!(i32, IVec2, IVec3, IVec4, BVec3, XYZI32);
    impl_vecn_signed_methods!(i32, IVec3, BVec3, XYZI32, SignedVector3);
    impl_vecn_as_f32!(Vec3, x, y, z);
    impl_vecn_as_f64!(DVec3, x, y, z);
    impl_vecn_as_u32!(UVec3, x, y, z);
}
impl_vec3_common_traits!(i32, ivec3, IVec2, IVec3, IVec4, BVec3, XYZI32);
impl_vecn_signed_traits!(i32, 3, IVec3, XYZI32, SignedVector3);

type XYZU32 = XYZ<u32>;

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec3(pub(crate) XYZU32);

impl UVec3 {
    impl_vec3_common_methods!(u32, UVec2, UVec3, UVec4, BVec3, XYZU32);
    impl_vecn_as_f32!(Vec3, x, y, z);
    impl_vecn_as_f64!(DVec3, x, y, z);
    impl_vecn_as_i32!(IVec3, x, y, z);
}
impl_vec3_common_traits!(u32, uvec3, UVec2, UVec3, UVec4, BVec3, XYZU32);

#[test]
fn test_vec3_private() {
    assert_eq!(
        vec3a(1.0, 1.0, 1.0).mul_add(vec3a(0.5, 2.0, -4.0), vec3a(-1.0, -1.0, -1.0)),
        vec3a(-0.5, 1.0, -5.0)
    );
}
