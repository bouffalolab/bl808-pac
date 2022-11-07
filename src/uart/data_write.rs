#[doc = "Register `data_write` writer"]
pub struct W(crate::W<DATA_WRITE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_WRITE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DATA_WRITE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_WRITE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` writer - Write data to FIFO"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_WRITE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Write data to FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO write data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_write](index.html) module"]
pub struct DATA_WRITE_SPEC;
impl crate::RegisterSpec for DATA_WRITE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_write::W](W) writer structure"]
impl crate::Writable for DATA_WRITE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets data_write to value 0"]
impl crate::Resettable for DATA_WRITE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
