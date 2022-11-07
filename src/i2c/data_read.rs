#[doc = "Register `data_read` reader"]
pub struct R(crate::R<DATA_READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_READ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `value` reader - Read data from FIFO"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Read data from FIFO"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FIFO read data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_read](index.html) module"]
pub struct DATA_READ_SPEC;
impl crate::RegisterSpec for DATA_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_read::R](R) reader structure"]
impl crate::Readable for DATA_READ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_read to value 0"]
impl crate::Resettable for DATA_READ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
