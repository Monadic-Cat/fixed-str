//! Fixed width Unicode strings!

// Basically, copying stuff from std to make `str` but with a fixed sized array behind it.
// Copying comments where appropriate.
// Functions declared `unsafe` without any internal unsafe code
// are declared such to match the `std` interface.
// In particular, this will have to do with `str` being *guaranteed* to be valid UTF-8.

// TODO: consider adding indexing methods that use const generic parameters for indices
// TODO: consider adding a `const` way to construct fixed size strings?
// TODO: actually design a useful interface lmao
// TODO: don't forget this exists when you find yourself wanting fixed size strings

//! The [`Str`] type, a fixed sized version of [`str`].
//! See the [`str`] docs for most methods defined here.
#[repr(transparent)]
pub struct Str<const SIZE: usize> {
    buf: [u8; SIZE],
}
// Btw, the only real reason to copy these implementations
// manually instead of just Deref-ing to `str` is that we want
// things to be usable from const contexts like they are on `str`.
impl<const SIZE: usize> Str<SIZE> {
    pub const fn len(&self) -> usize {
        // Note: Could use the SIZE type parameter.
        // I don't think it matters, though.
        self.buf.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_char_boundary(&self, index: usize) -> bool {
        // 0 and len are always ok.
        // Test for 0 explicitly so that it can optimize out the check
        // easily and skip reading string data for that case.
        if index == 0 || index == self.len() {
            true
        } else {
            // TODO: could probably make this function const by
            // just indexing ourselves instead of doing this like std
            match self.as_bytes().get(index) {
                None => false,
                // This is bit magic equivalent to: b < 128 || b >= 192
                Some(&b) => (b as i8) >= -0x40,
            }
        }
    }

    pub const fn as_bytes(&self) -> &[u8; SIZE] {
        // lmao since we're not even using `str` this doesn't require a transmute
        &self.buf
    }

    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8; SIZE] {
        &mut self.buf
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    // TODO: SliceIndex based methods, to which `slice_unchecked` and co are deprecated in favor of
    pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str {
        #[allow(unused_unsafe)]
        unsafe { ::core::str::from_utf8_unchecked(&self.buf[begin..end]) }
    }

    pub unsafe fn slice_mut_unchecked(&mut self, begin: usize, end: usize) -> &mut str {
        #[allow(unused_unsafe)]
        unsafe { ::core::str::from_utf8_unchecked_mut(&mut self.buf[begin..end]) }
    }

    pub fn split_at(&self, mid: usize) -> (&str, &str) {
        // is_char_boundary checks that the index is in [0, .len()]
        if self.is_char_boundary(mid) {
            // SAFETY: just checked that `mid` is on a char boundary.
            unsafe {
                // TODO: use the SliceIndex based methods to match the std impl
                let (front, back) = self.buf.split_at(mid);
                (::core::str::from_utf8_unchecked(front), ::core::str::from_utf8_unchecked(back))
            }
        } else {
            slice_error_fail(self, 0, mid)
        }
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) {
        // is_char_boundary checks that the index is in [0, .len()]
        if self.is_char_boundary(mid) {
            unsafe {
                // SAFETY: just checked that `mid` is on a char boundary.
                let (front, back) = self.buf.split_at_mut(mid);
                (::core::str::from_utf8_unchecked_mut(front),
                 ::core::str::from_utf8_unchecked_mut(back))
            }
        } else {
            slice_error_fail(self, 0, mid)
        }
    }

    pub fn chars(&self) -> Chars<'_, SIZE> {
        todo!("chars iteration")
    }

    pub fn char_indices(&self) -> CharIndices<'_, SIZE> {
        todo!("char indices iteration")
    }

    // TODO: use a real return type for this
    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.as_bytes().iter().copied()
    }

    // TODO: resume copying at split_whitespace, probably
}

pub struct Chars<'a, const SIZE: usize> {
    _hm: ::core::marker::PhantomData<&'a Str<SIZE>>,
}
pub struct CharIndices<'a, const SIZE: usize> {
    _hm: ::core::marker::PhantomData<&'a Str<SIZE>>,
}

fn slice_error_fail<const SIZE: usize>(_s: &Str<SIZE>, _begin: usize, _end: usize) -> ! {
    todo!("copying std failure to slice error abort thingy")
}

// Implementations I just want, lol.
impl<const N: usize> Str<N> {
    // These two method names are copied from the array type.
    pub fn as_slice(&self) -> &str {
        unsafe { ::core::str::from_utf8_unchecked(&self.buf) }
    }
    pub fn as_slice_mut(&mut self) -> &mut str {
        unsafe { ::core::str::from_utf8_unchecked_mut(&mut self.buf) }
    }
    // this one i just want
    pub fn zeroed() -> Self {
        // Safety: repeated 0 bytes is valid UTF-8 lol
        Self { buf: [0; N] }
    }
}
