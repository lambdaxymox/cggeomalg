/*
 * Generate a view into a multivector type that accesses the components
 * of the multivector type by name.
 *
 * The component names are used in conjunction with a `Deref` implementation
 * such that one can access the components of a multivector by name instead
 * of only by index. For example, the components names for the two-dimensional
 * Euclidean multivector are `scalar`, `e1`, `e2` and `e12`. The underlying data
 * structure for the two-dimensional multivector is an array of length four, and
 * each component name corresponds to an element of the array:
 * ```text
 * multivector.scalar <--> multivector[0]
 * multivector.e1     <--> multivector[1]
 * multivector.e2     <--> multivector[2]
 * multivector.e12    <--> multivector[3]
 * ```
 */
#[macro_export]
macro_rules! impl_coords {
    ($T:ident, { $($comps: ident),* }) => {
        #[repr(C)]
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $T<S: Copy> {
            $(pub $comps: S),*
        }
    }
}

/*
 *
 * Generate the component accessors for a multivector type.
 *
 */
#[macro_export]
macro_rules! impl_coords_deref {
    ($Source:ident, $Target:ident) => {
        impl<S> core::ops::Deref for $Source<S>
        where
            S: Copy,
        {
            type Target = $Target<S>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.as_ptr() as *const $Target<S>) }
            }
        }

        impl<S> core::ops::DerefMut for $Source<S>
        where
            S: Copy,
        {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self.as_mut_ptr() as *mut $Target<S>) }
            }
        }
    };
}
