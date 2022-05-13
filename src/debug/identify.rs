#[doc = "Register `identify[%s]` reader"]
pub struct R(crate::R<IDENTIFY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDENTIFY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDENTIFY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDENTIFY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `word` reader - Read identifier part in word"]
pub type WORD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read identifier part in word"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(self.bits)
    }
}
#[doc = "Unique module identifier\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [identify](index.html) module"]
pub struct IDENTIFY_SPEC;
impl crate::RegisterSpec for IDENTIFY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [identify::R](R) reader structure"]
impl crate::Readable for IDENTIFY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets identify[%s]
to value 0"]
impl crate::Resettable for IDENTIFY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
