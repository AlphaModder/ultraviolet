/// A bivector is an *oriented area*, and is equivalent
/// to the result of the exterior (wedge) product of two vectors, i.e.
/// `u ∧ v`. This means it is the *oriented area* of the parallelogram
/// created by attaching two vectors and then extending them into a parallelogram.
///
/// A normalized bivector can also be thought of as representing a plane and the *direction of rotation*
/// about that plane such that a *positive* rotation follows the orientation of the bivector. When
/// you obtain a bivector by taking the exterior product of two vectors, the positive direction of rotation
/// is defined as the one that *brings the first vector closer to the second*. For example, a bivector
/// created by taking the exterior product `x ∧ y` of the x and y basis vectors will create a unit
/// bivector that represents the xy plane, with orientation such that a positive rotation of `x` inside
/// the plane would bring `x` closer to `y`. This is why positive rotation is generally defined as
/// "counter clockwise" in 2d, since such a rotation brings `x` to `y`.
use wide::f32x4;

use crate::util::*;

use std::ops::*;

macro_rules! bivec2s {
    ($(($bn:ident) => $t:ident),+) => {
        $(
        /// A bivector in 2d space.
        ///
        /// Since in 2d there is only one plane in the whole of 2d space, a 2d bivector
        /// has only one component.
        #[derive(Clone, Copy, Debug)]
        pub struct $bn {
            pub xy: $t
        }

        impl $bn {
            #[inline]
            pub fn new(xy: $t) -> Self {
                Self {
                    xy
                }
            }

            #[inline]
            pub fn unit_xy() -> Self {
                Self::new($t::from(1.0))
            }
        }

        impl EqualsEps for $bn {
            fn eq_eps(self, other: Self) -> bool {
                self.xy.eq_eps(other.xy)
            }
        }

        impl Add for $bn {
            type Output = Self;
            #[inline]
            fn add(mut self, rhs: $bn) -> Self {
                self += rhs;
                self
            }
        }

        impl AddAssign for $bn {
            #[inline]
            fn add_assign(&mut self, rhs: $bn) {
                self.xy += rhs.xy;
            }
        }

        impl Sub for $bn {
            type Output = Self;
            #[inline]
            fn sub(mut self, rhs: $bn) -> Self {
                self -= rhs;
                self
            }
        }

        impl SubAssign for $bn {
            #[inline]
            fn sub_assign(&mut self, rhs: $bn) {
                self.xy -= rhs.xy;
            }
        }

        impl Mul for $bn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $bn) -> Self {
                self *= rhs;
                self
            }
        }

        impl Mul<$bn> for $t {
            type Output = $bn;
            #[inline]
            fn mul(self, mut rhs: $bn) -> $bn {
                rhs *= self;
                rhs
            }
        }

        impl Mul<$t> for $bn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $t) -> Self {
                self *= rhs;
                self
            }
        }

        impl MulAssign for $bn {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.xy *= rhs.xy;
            }
        }

        impl MulAssign<$t> for $bn {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.xy *= rhs;
            }
        }

        impl Div for $bn {
            type Output = Self;
            #[inline]
            fn div(mut self, rhs: $bn) -> Self {
                self /= rhs;
                self
            }
        }

        impl Div<$t> for $bn {
            type Output = $bn;
            #[inline]
            fn div(mut self, rhs: $t) -> $bn {
                self.xy /= rhs;
                self
            }
        }

        impl DivAssign for $bn {
            #[inline]
            fn div_assign(&mut self, rhs: $bn) {
                self.xy /= rhs.xy;
            }
        }

        impl DivAssign<$t> for $bn {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.xy /= rhs;
            }
        }

        impl Neg for $bn {
            type Output = Self;
            #[inline]
            fn neg(mut self) -> Self {
                self.xy = -self.xy;
                self
            }
        }
        )+
    }
}

bivec2s!((Bivec2) => f32, (WBivec2) => f32x4);

macro_rules! bivec3s {
    ($(($bn:ident) => $t:ident),+) => {
        $(
        /// A bivector in 3d space.
        ///
        /// In 3d, a bivector has 3 components, each one representing the signed *projected area* of the bivector
        /// onto one of the 3 *basis bivectors*, which can be thought of as corresponding to each of hte
        /// three basis planes. This is analogous to the components of a 3d vector, which correspond to the
        /// *projected length* of the vector onto the three basis *vectors. Since in 3d, there are three
        /// components for both vectors and bivectors, 3d bivectors have been historically confused with
        /// 3d vectors quite a lot.
        #[derive(Clone, Copy, Debug)]
        pub struct $bn {
            pub xy: $t,
            pub xz: $t,
            pub yz: $t,
        }

        impl EqualsEps for $bn {
            fn eq_eps(self, other: Self) -> bool {
                self.xy.eq_eps(other.xy) && self.xz.eq_eps(other.xz) && self.yz.eq_eps(other.yz)
            }
        }

        impl $bn {
            #[inline]
            pub fn new(xy: $t, xz: $t, yz: $t) -> Self {
                Self {
                    xy, xz, yz
                }
            }

            #[inline]
            pub fn unit_xy() -> Self {
                Self::new($t::from(1.0), $t::from(0.0), $t::from(0.0))
            }

            #[inline]
            pub fn unit_xz() -> Self {
                Self::new($t::from(0.0), $t::from(1.0), $t::from(0.0))
            }

            #[inline]
            pub fn unit_yz() -> Self {
                Self::new($t::from(0.0), $t::from(0.0), $t::from(1.0))
            }
        }

        impl Add for $bn {
            type Output = Self;
            #[inline]
            fn add(mut self, rhs: $bn) -> Self {
                self += rhs;
                self
            }
        }

        impl AddAssign for $bn {
            #[inline]
            fn add_assign(&mut self, rhs: $bn) {
                self.xy += rhs.xy;
                self.xz += rhs.xz;
                self.yz += rhs.yz;
            }
        }

        impl Sub for $bn {
            type Output = Self;
            #[inline]
            fn sub(mut self, rhs: $bn) -> Self {
                self -= rhs;
                self
            }
        }

        impl SubAssign for $bn {
            #[inline]
            fn sub_assign(&mut self, rhs: $bn) {
                self.xy -= rhs.xy;
                self.xz -= rhs.xz;
                self.yz -= rhs.yz;
            }
        }

        impl Mul for $bn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $bn) -> Self {
                self *= rhs;
                self
            }
        }

        impl Mul<$bn> for $t {
            type Output = $bn;
            #[inline]
            fn mul(self, mut rhs: $bn) -> $bn {
                rhs *= self;
                rhs
            }
        }

        impl Mul<$t> for $bn {
            type Output = Self;
            #[inline]
            fn mul(mut self, rhs: $t) -> Self {
                self *= rhs;
                self
            }
        }

        impl MulAssign for $bn {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.xy *= rhs.xy;
                self.xz *= rhs.xz;
                self.yz *= rhs.yz;
            }
        }

        impl MulAssign<$t> for $bn {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.xy *= rhs;
                self.xz *= rhs;
                self.yz *= rhs;
            }
        }

        impl Div for $bn {
            type Output = Self;
            #[inline]
            fn div(mut self, rhs: $bn) -> Self {
                self /= rhs;
                self
            }
        }

        impl Div<$t> for $bn {
            type Output = $bn;
            #[inline]
            fn div(mut self, rhs: $t) -> $bn {
                self.xy /= rhs;
                self
            }
        }

        impl DivAssign for $bn {
            #[inline]
            fn div_assign(&mut self, rhs: $bn) {
                self.xy /= rhs.xy;
                self.xz /= rhs.xz;
                self.yz /= rhs.yz;
            }
        }

        impl DivAssign<$t> for $bn {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.xy /= rhs;
                self.xz /= rhs;
                self.yz /= rhs;
            }
        }

        impl Neg for $bn {
            type Output = Self;
            #[inline]
            fn neg(mut self) -> Self {
                self.xy = -self.xy;
                self.xz = -self.xz;
                self.yz = -self.yz;
                self
            }
        }
        )+
    }
}

bivec3s!((Bivec3) => f32, (WBivec3) => f32x4);
