#[doc = "Register `bus_state` reader"]
pub struct R(crate::R<BUS_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `receive_busy` reader - Indicates that UART receive bus is busy"]
pub struct RECEIVE_BUSY_R(crate::FieldReader<bool>);
impl RECEIVE_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECEIVE_BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `transmit_busy` reader - Indicates that UART transmit bus is busy"]
pub struct TRANSMIT_BUSY_R(crate::FieldReader<bool>);
impl TRANSMIT_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANSMIT_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSMIT_BUSY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Indicates that UART receive bus is busy"]
    #[inline(always)]
    pub fn receive_busy(&self) -> RECEIVE_BUSY_R {
        RECEIVE_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates that UART transmit bus is busy"]
    #[inline(always)]
    pub fn transmit_busy(&self) -> TRANSMIT_BUSY_R {
        TRANSMIT_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Bus state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_state](index.html) module"]
pub struct BUS_STATE_SPEC;
impl crate::RegisterSpec for BUS_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_state::R](R) reader structure"]
impl crate::Readable for BUS_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bus_state to value 0"]
impl crate::Resettable for BUS_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
