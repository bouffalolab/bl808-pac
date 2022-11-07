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
#[doc = "Field `transmit_count` reader - Count of available data in transmit FIFO"]
pub type TRANSMIT_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `receive_count` reader - Count of available data in receive FIFO"]
pub type RECEIVE_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `transmit_threshold` reader - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
pub type TRANSMIT_THRESHOLD_R = crate::BitReader<bool>;
#[doc = "Field `transmit_threshold` writer - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
pub type TRANSMIT_THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_1_SPEC, bool, O>;
#[doc = "Field `receive_threshold` reader - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub type RECEIVE_THRESHOLD_R = crate::BitReader<bool>;
#[doc = "Field `receive_threshold` writer - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
pub type RECEIVE_THRESHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONFIG_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Count of available data in transmit FIFO"]
    #[inline(always)]
    pub fn transmit_count(&self) -> TRANSMIT_COUNT_R {
        TRANSMIT_COUNT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Count of available data in receive FIFO"]
    #[inline(always)]
    pub fn receive_count(&self) -> RECEIVE_COUNT_R {
        RECEIVE_COUNT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
    #[inline(always)]
    pub fn transmit_threshold(&self) -> TRANSMIT_THRESHOLD_R {
        TRANSMIT_THRESHOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    pub fn receive_threshold(&self) -> RECEIVE_THRESHOLD_R {
        RECEIVE_THRESHOLD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Transmit FIFO threshold\n\n DMA request will not be asserted if `transmit_available` is less than this value"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_threshold(&mut self) -> TRANSMIT_THRESHOLD_W<16> {
        TRANSMIT_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 24 - Receive FIFO threshold\n\n DMA request will not be asserted if `receive_available` is less than this value"]
    #[inline(always)]
    #[must_use]
    pub fn receive_threshold(&mut self) -> RECEIVE_THRESHOLD_W<24> {
        RECEIVE_THRESHOLD_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fifo_config_1 to value 0x02"]
impl crate::Resettable for FIFO_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
