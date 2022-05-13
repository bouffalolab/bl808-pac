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
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Write data to FIFO"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
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
}
#[doc = "`reset()` method sets data_write to value 0"]
impl crate::Resettable for DATA_WRITE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
