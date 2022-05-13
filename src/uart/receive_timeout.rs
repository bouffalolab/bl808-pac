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
pub struct VALUE_R(crate::FieldReader<u8>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `value` writer - Timeout interrupt triggering value by bits received"]
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
}
#[doc = "`reset()` method sets receive_timeout to value 0x0f"]
impl crate::Resettable for RECEIVE_TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
