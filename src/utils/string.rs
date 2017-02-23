#[derive(Clone, Default, Eq, Hash, PartialOrd, Ord, PartialEq)]
struct MutableString {
    vec: Vec<char>
}

impl MutableString {
    // All the documentation in the official source https://doc.rust-lang.org/src/collections/string.rs.html
    #[inline]
    pub fn new() -> MutableString {
        MutableString { vec: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> MutableString {
        MutableString { vec: Vec::with_capacity(capacity) }
    }

    #[inline]
    pub fn from_vec(vec: Vec<char>) -> MutableString {
        MutableString { vec: vec }
    }

    #[inline]
    pub fn into_chars(self) -> Vec<char> {
        self.vec
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.vec.push(ch)
    }

    #[inline]
    pub fn as_chars(&self) -> &[char] {
        &self.vec
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            self.vec.truncate(new_len)
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.vec.pop()
    }

    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<char> {
        &mut self.vec
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn into_boxed_str(self) -> Box<str> {
        let slice = self.vec.into_boxed_slice();
        unsafe { std::mem::transmute::<Box<[char]>, Box<str>>(slice) }
    }
}

impl FromStr for MutableString {
    type Err = ParseError;
    #[inline]
    fn from_str(s: &str) -> Result<MutableString, ParseError> {
        Ok( MutableString{ vec: s.chars().collect() } )
    }
}

impl Index<usize> for MutableString {
    type Output = char;
    #[inline]
    fn index(&self, idx: usize) -> &char {
        &self.vec[idx]
    }
}
impl Index<Range<usize>> for MutableString{
    type Output = [char];
    #[inline]
    fn index(&self, index: Range<usize>) -> &[char] {
        &self.vec[index]
    }
}
impl IndexMut<usize> for MutableString {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut char {
        self.vec.index_mut(idx)
    }
}
impl IndexMut<Range<usize>> for MutableString {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [char] {
        self.vec.index_mut(index)
    }
}

impl Display for MutableString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.pad(&self.vec.iter().cloned().collect::<String>())
    }
}
impl Debug for MutableString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", &self.vec.iter().cloned().collect::<String>())
    }
}

/*
#[stable(feature = "rust1", since = "1.0.0")]
impl FromIterator<char> for String {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> String {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> FromIterator<&'a str> for String {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> String {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

#[stable(feature = "extend_string", since = "1.4.0")]
impl FromIterator<String> for String {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> String {
        let mut buf = String::new();
        buf.extend(iter);
        buf
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Extend<char> for String {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        for ch in iterator {
            self.push(ch)
        }
    }
}

#[stable(feature = "extend_ref", since = "1.2.0")]
impl<'a> Extend<&'a char> for String {
    fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> Extend<&'a str> for String {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        for s in iter {
            self.push_str(s)
        }
    }
}

#[stable(feature = "extend_string", since = "1.4.0")]
impl Extend<String> for String {
    fn extend<I: IntoIterator<Item = String>>(&mut self, iter: I) {
        for s in iter {
            self.push_str(&s)
        }
    }
}

/// A convenience impl that delegates to the impl for `&str`
#[unstable(feature = "pattern",
           reason = "API not fully fleshed out and ready to be stabilized",
           issue = "27721")]
impl<'a, 'b> Pattern<'a> for &'b String {
    type Searcher = <&'b str as Pattern<'a>>::Searcher;

    fn into_searcher(self, haystack: &'a str) -> <&'b str as Pattern<'a>>::Searcher {
        self[..].into_searcher(haystack)
    }

    #[inline]
    fn is_contained_in(self, haystack: &'a str) -> bool {
        self[..].is_contained_in(haystack)
    }

    #[inline]
    fn is_prefix_of(self, haystack: &'a str) -> bool {
        self[..].is_prefix_of(haystack)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(&self[..], &other[..])
    }
    #[inline]
    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(&self[..], &other[..])
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl hash::Hash for String {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        (**self).hash(hasher)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> Add<&'a str> for String {
    type Output = String;

    #[inline]
    fn add(mut self, other: &str) -> String {
        self.push_str(other);
        self
    }
}

#[stable(feature = "stringaddassign", since = "1.12.0")]
impl<'a> AddAssign<&'a str> for String {
    #[inline]
    fn add_assign(&mut self, other: &str) {
        self.push_str(other);
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Index<ops::Range<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &str {
        &self[..][index]
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Index<ops::RangeTo<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &str {
        &self[..][index]
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Index<ops::RangeFrom<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &str {
        &self[..][index]
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Index<ops::RangeFull> for String {
    type Output = str;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}
#[unstable(feature = "inclusive_range", reason = "recently added, follows RFC", issue = "28237")]
impl ops::Index<ops::RangeInclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeInclusive<usize>) -> &str {
        Index::index(&**self, index)
    }
}
#[unstable(feature = "inclusive_range", reason = "recently added, follows RFC", issue = "28237")]
impl ops::Index<ops::RangeToInclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeToInclusive<usize>) -> &str {
        Index::index(&**self, index)
    }
}

#[stable(feature = "derefmut_for_string", since = "1.2.0")]
impl ops::IndexMut<ops::Range<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut str {
        &mut self[..][index]
    }
}
#[stable(feature = "derefmut_for_string", since = "1.2.0")]
impl ops::IndexMut<ops::RangeTo<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut str {
        &mut self[..][index]
    }
}
#[stable(feature = "derefmut_for_string", since = "1.2.0")]
impl ops::IndexMut<ops::RangeFrom<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut str {
        &mut self[..][index]
    }
}
#[stable(feature = "derefmut_for_string", since = "1.2.0")]
impl ops::IndexMut<ops::RangeFull> for String {
    #[inline]
    fn index_mut(&mut self, _index: ops::RangeFull) -> &mut str {
        unsafe { mem::transmute(&mut *self.vec) }
    }
}
#[unstable(feature = "inclusive_range", reason = "recently added, follows RFC", issue = "28237")]
impl ops::IndexMut<ops::RangeInclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeInclusive<usize>) -> &mut str {
        IndexMut::index_mut(&mut **self, index)
    }
}
#[unstable(feature = "inclusive_range", reason = "recently added, follows RFC", issue = "28237")]
impl ops::IndexMut<ops::RangeToInclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeToInclusive<usize>) -> &mut str {
        IndexMut::index_mut(&mut **self, index)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}

#[stable(feature = "derefmut_for_string", since = "1.2.0")]
impl ops::DerefMut for String {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        unsafe { mem::transmute(&mut *self.vec) }
    }
}

/// An error when parsing a `String`.
///
/// This `enum` is slightly awkward: it will never actually exist. This error is
/// part of the type signature of the implementation of [`FromStr`] on
/// [`String`]. The return type of [`from_str()`], requires that an error be
/// defined, but, given that a [`String`] can always be made into a new
/// [`String`] without error, this type will never actually be returned. As
/// such, it is only here to satisfy said signature, and is useless otherwise.
///
/// [`FromStr`]: ../../std/str/trait.FromStr.html
/// [`String`]: struct.String.html
/// [`from_str()`]: ../../std/str/trait.FromStr.html#tymethod.from_str
#[stable(feature = "str_parse_error", since = "1.5.0")]
#[derive(Copy)]
pub enum ParseError {}

#[stable(feature = "rust1", since = "1.0.0")]
impl FromStr for String {
    type Err = ParseError;
    #[inline]
    fn from_str(s: &str) -> Result<String, ParseError> {
        Ok(String::from(s))
    }
}

#[stable(feature = "str_parse_error", since = "1.5.0")]
impl Clone for ParseError {
    fn clone(&self) -> ParseError {
        match *self {}
    }
}

#[stable(feature = "str_parse_error", since = "1.5.0")]
impl fmt::Debug for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

#[stable(feature = "str_parse_error2", since = "1.8.0")]
impl fmt::Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

#[stable(feature = "str_parse_error", since = "1.5.0")]
impl PartialEq for ParseError {
    fn eq(&self, _: &ParseError) -> bool {
        match *self {}
    }
}

#[stable(feature = "str_parse_error", since = "1.5.0")]
impl Eq for ParseError {}

/// A trait for converting a value to a `String`.
///
/// This trait is automatically implemented for any type which implements the
/// [`Display`] trait. As such, `ToString` shouldn't be implemented directly:
/// [`Display`] should be implemented instead, and you get the `ToString`
/// implementation for free.
///
/// [`Display`]: ../../std/fmt/trait.Display.html
#[stable(feature = "rust1", since = "1.0.0")]
pub trait ToString {
    /// Converts the given value to a `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let i = 5;
    /// let five = String::from("5");
    ///
    /// assert_eq!(five, i.to_string());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn to_string(&self) -> String;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Display + ?Sized> ToString for T {
    #[inline]
    default fn to_string(&self) -> String {
        use core::fmt::Write;
        let mut buf = String::new();
        let _ = buf.write_fmt(format_args!("{}", self));
        buf.shrink_to_fit();
        buf
    }
}

#[stable(feature = "str_to_string_specialization", since = "1.9.0")]
impl ToString for str {
    #[inline]
    fn to_string(&self) -> String {
        String::from(self)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<str> for String {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<[char]> for String {
    #[inline]
    fn as_ref(&self) -> &[char] {
        self.as_bytes()
    }
}

impl From<MutableString> for Vec<char> {
    fn from(string : MutableString) -> Vec<char> {
        string.into_bytes()
    }
}

impl fmt::Write for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        Ok(())
    }
}*/
