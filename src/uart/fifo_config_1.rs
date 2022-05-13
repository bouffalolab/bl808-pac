#[doc = "Register `fifo_config_1` reader"]
pub struct R(crate::R<FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONFIG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONFIG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONFIG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fifo_config_1` writer"]
pub struct W(crate::W<FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONFIG_1_SPEC>;
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
impl From<crate::W<FIFO_CONFIG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONFIG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `receive_threshold` reader - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub struct RECEIVE_THRESHOLD_R(crate::FieldReader<u8>);
impl RECEIVE_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECEIVE_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECEIVE_THRESHOLD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `receive_threshold` writer - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub struct RECEIVE_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `transmit_threshold` reader - Transmit FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub struct TRANSMIT_THRESHOLD_R(crate::FieldReader<u8>);
impl TRANSMIT_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANSMIT_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSMIT_THRESHOLD_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `transmit_threshold` writer - Transmit FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub struct TRANSMIT_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `receive_available` reader - Count of available data in receive FIFO"]
pub struct RECEIVE_AVAILABLE_R(crate::FieldReader<u8>);
impl RECEIVE_AVAILABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECEIVE_AVAILABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECEIVE_AVAILABLE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `transmit_available` reader - Count of available data in transmit FIFO"]
pub struct TRANSMIT_AVAILABLE_R(crate::FieldReader<u8>);
impl TRANSMIT_AVAILABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRANSMIT_AVAILABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSMIT_AVAILABLE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:28 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn receive_threshold(&self) -> RECEIVE_THRESHOLD_R {
        RECEIVE_THRESHOLD_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Transmit FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn transmit_threshold(&self) -> TRANSMIT_THRESHOLD_R {
        TRANSMIT_THRESHOLD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Count of available data in receive FIFO"]
    #[inline(always)]
    pub fn receive_available(&self) -> RECEIVE_AVAILABLE_R {
        RECEIVE_AVAILABLE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Count of available data in transmit FIFO"]
    #[inline(always)]
    pub fn transmit_available(&self) -> TRANSMIT_AVAILABLE_R {
        TRANSMIT_AVAILABLE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn receive_threshold(&mut self) -> RECEIVE_THRESHOLD_W {
        RECEIVE_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 16:20 - Transmit FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn transmit_threshold(&mut self) -> TRANSMIT_THRESHOLD_W {
        TRANSMIT_THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_config_1](index.html) module"]
pub struct FIFO_CONFIG_1_SPEC;
impl crate::RegisterSpec for FIFO_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_config_1::R](R) reader structure"]
impl crate::Readable for FIFO_CONFIG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_config_1::W](W) writer structure"]
impl crate::Writable for FIFO_CONFIG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fifo_config_1 to value 0x20"]
impl crate::Resettable for FIFO_CONFIG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
