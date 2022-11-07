#[doc = "Register `receive_timeout` reader"]
pub struct R(crate::R<RECEIVE_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `receive_timeout` writer"]
pub struct W(crate::W<RECEIVE_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_TIMEOUT_SPEC>;
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
impl From<crate::W<RECEIVE_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` reader - Timeout interrupt triggering value by bits received"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `value` writer - Timeout interrupt triggering value by bits received"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RECEIVE_TIMEOUT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timeout interrupt triggering value by bits received"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout interrupt triggering value by bits received"]
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
#[doc = "Receive Time-Out interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_timeout](index.html) module"]
pub struct RECEIVE_TIMEOUT_SPEC;
impl crate::RegisterSpec for RECEIVE_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_timeout::R](R) reader structure"]
impl crate::Readable for RECEIVE_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_timeout::W](W) writer structure"]
impl crate::Writable for RECEIVE_TIMEOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_timeout to value 0x0f"]
impl crate::Resettable for RECEIVE_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
