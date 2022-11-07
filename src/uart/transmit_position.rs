#[doc = "Register `transmit_position` reader"]
pub struct R(crate::R<TRANSMIT_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `transmit_position` writer"]
pub struct W(crate::W<TRANSMIT_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_POSITION_SPEC>;
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
impl From<crate::W<TRANSMIT_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `start` reader - Start position of transmit IR pulse"]
pub type START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `start` writer - Start position of transmit IR pulse"]
pub type START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSMIT_POSITION_SPEC, u16, u16, 16, O>;
#[doc = "Field `stop` reader - Stop position of transmit IR pulse"]
pub type STOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `stop` writer - Stop position of transmit IR pulse"]
pub type STOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSMIT_POSITION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<16> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR-mode transmit position control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_position](index.html) module"]
pub struct TRANSMIT_POSITION_SPEC;
impl crate::RegisterSpec for TRANSMIT_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_position::R](R) reader structure"]
impl crate::Readable for TRANSMIT_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_position::W](W) writer structure"]
impl crate::Writable for TRANSMIT_POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets transmit_position to value 0x009f_0070"]
impl crate::Resettable for TRANSMIT_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x009f_0070;
}
