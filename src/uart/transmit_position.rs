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
#[doc = "Field `stop` reader - Stop position of transmit IR pulse"]
pub struct STOP_R(crate::FieldReader<u16>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `stop` writer - Stop position of transmit IR pulse"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `start` reader - Start position of transmit IR pulse"]
pub struct START_R(crate::FieldReader<u16>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start` writer - Start position of transmit IR pulse"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Stop position of transmit IR pulse"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bits 0:15 - Start position of transmit IR pulse"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
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
}
#[doc = "`reset()` method sets transmit_position to value 0x009f_0070"]
impl crate::Resettable for TRANSMIT_POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x009f_0070
    }
}
