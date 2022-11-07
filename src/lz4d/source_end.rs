#[doc = "Register `source_end` reader"]
pub struct R(crate::R<SOURCE_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCE_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCE_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCE_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `end` reader - End of address"]
pub type END_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:25 - End of address"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "Reads the end address of source after decompression\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [source_end](index.html) module"]
pub struct SOURCE_END_SPEC;
impl crate::RegisterSpec for SOURCE_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [source_end::R](R) reader structure"]
impl crate::Readable for SOURCE_END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets source_end to value 0"]
impl crate::Resettable for SOURCE_END_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
