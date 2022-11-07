#[doc = "Register `receive_position` reader"]
pub struct R(crate::R<RECEIVE_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `receive_position` writer"]
pub struct W(crate::W<RECEIVE_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_POSITION_SPEC>;
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
impl From<crate::W<RECEIVE_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `start` reader - Start position of received pulse recovered from IR signal"]
pub type START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `start` writer - Start position of received pulse recovered from IR signal"]
pub type START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEIVE_POSITION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Start position of received pulse recovered from IR signal"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start position of received pulse recovered from IR signal"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR-mode receive position control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_position](index.html) module"]
pub struct RECEIVE_POSITION_SPEC;
impl crate::RegisterSpec for RECEIVE_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_position::R](R) reader structure"]
impl crate::Readable for RECEIVE_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_position::W](W) writer structure"]
impl crate::Writable for RECEIVE_POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_position to value 0x6f"]
impl crate::Resettable for RECEIVE_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x6f;
}
