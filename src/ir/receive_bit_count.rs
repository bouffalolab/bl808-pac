#[doc = "Register `receive_bit_count` reader"]
pub struct R(crate::R<RECEIVE_BIT_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_BIT_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_BIT_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_BIT_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `receive_bit_count` writer"]
pub struct W(crate::W<RECEIVE_BIT_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_BIT_COUNT_SPEC>;
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
impl From<crate::W<RECEIVE_BIT_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_BIT_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_bit_count](index.html) module"]
pub struct RECEIVE_BIT_COUNT_SPEC;
impl crate::RegisterSpec for RECEIVE_BIT_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_bit_count::R](R) reader structure"]
impl crate::Readable for RECEIVE_BIT_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_bit_count::W](W) writer structure"]
impl crate::Writable for RECEIVE_BIT_COUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_bit_count to value 0"]
impl crate::Resettable for RECEIVE_BIT_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
