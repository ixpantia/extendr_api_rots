pub struct RoStrings(extendr_api::Strings);

unsafe impl Sync for RoStrings {}
unsafe impl Send for RoStrings {}

impl RoStrings {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&str> {
        self.0.get(i).map(AsRef::as_ref)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize) -> &str {
        self.0.get_unchecked(i)
    }
}

impl std::ops::Index<usize> for RoStrings {
    type Output = str;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl TryFrom<extendr_api::Robj> for RoStrings {
    type Error = extendr_api::Error;
    fn try_from(value: extendr_api::Robj) -> Result<Self, Self::Error> {
        extendr_api::Strings::try_from(value).map(RoStrings)
    }
}
