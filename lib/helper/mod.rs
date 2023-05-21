use core::{fmt, mem::MaybeUninit, ptr::addr_of_mut};

/// Case-insensitive chars and strs
pub mod case_insensitive;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Errors for [`try_collect_into_array`]
pub enum CollectIntoArrayError {
    /// The array to collect into was not large enough
    ArrayNotLargeEnough,
}

impl fmt::Display for CollectIntoArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArrayNotLargeEnough => {
                write!(f, "The array to collect into was not large enough!")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CollectIntoArrayError {}

/// Try to collect the iterator into an array
///
/// # Errors
/// See [`CollectIntoArrayError`]
pub fn try_collect_into_array<const N: usize, T>(
    iterator: impl Iterator<Item = T>,
) -> Result<[Option<T>; N], CollectIntoArrayError> {
    // Create an uninitialised array
    let mut array: MaybeUninit<[Option<T>; N]> = MaybeUninit::uninit();
    let array_ptr = array.as_mut_ptr();

    // Write each item to the array, returning if full
    let mut index = 0;
    for item in iterator {
        unsafe {
            ((*array_ptr)
                .get_mut(index)
                .ok_or(CollectIntoArrayError::ArrayNotLargeEnough)? as *mut Option<T>)
                .write(Some(item));
        };
        index += 1;
    }

    // Fill the remainder of the array with None
    (index..N).for_each(|i| unsafe { addr_of_mut!((*array_ptr)[i]).write(None) });

    Ok(unsafe { array.assume_init() })
}
