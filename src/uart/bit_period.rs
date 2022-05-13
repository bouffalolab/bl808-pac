#[doc = "Register `bit_period` reader"]
pub struct R(crate::R<BIT_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIT_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIT_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIT_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bit_period` writer"]
pub struct W(crate::W<BIT_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIT_PERIOD_SPEC>;
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
impl From<crate::W<BIT_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIT_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `receive` reader - Period of each receive bit\n\n This field relates to baudrate."]
pub struct RECEIVE_R(crate::FieldReader<u16>);
impl RECEIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RECEIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECEIVE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `receive` writer - Period of each receive bit\n\n This field relates to baudrate."]
pub struct RECEIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `transmit` reader - Period of each transmit bit\n\n This field relates to baudrate."]
pub struct TRANSMIT_R(crate::FieldReader<u16>);
impl TRANSMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TRANSMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSMIT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `transmit` writer - Period of each transmit bit\n\n This field relates to baudrate."]
pub struct TRANSMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Period of each receive bit\n\n This field relates to baudrate."]
    #[inline(always)]
    pub fn receive(&self) -> RECEIVE_R {
        RECEIVE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n This field relates to baudrate."]
    #[inline(always)]
    pub fn transmit(&self) -> TRANSMIT_R {
        TRANSMIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Period of each receive bit\n\n This field relates to baudrate."]
    #[inline(always)]
    pub fn receive(&mut self) -> RECEIVE_W {
        RECEIVE_W { w: self }
    }
    #[doc = "Bits 0:15 - Period of each transmit bit\n\n This field relates to baudrate."]
    #[inline(always)]
    pub fn transmit(&mut self) -> TRANSMIT_W {
        TRANSMIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit period control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bit_period](index.html) module"]
pub struct BIT_PERIOD_SPEC;
impl crate::RegisterSpec for BIT_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bit_period::R](R) reader structure"]
impl crate::Readable for BIT_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bit_period::W](W) writer structure"]
impl crate::Writable for BIT_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets bit_period to value 0x00ff_00ff"]
impl crate::Resettable for BIT_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_00ff
    }
}
