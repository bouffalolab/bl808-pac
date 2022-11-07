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
#[doc = "Field `transmit_busy` reader - Indicates that UART transmit bus is busy"]
pub use RECEIVE_BUSY_R as TRANSMIT_BUSY_R;
#[doc = "Field `receive_busy` reader - Indicates that UART receive bus is busy"]
pub type RECEIVE_BUSY_R = crate::BitReader<BUS_BUSY_A>;
#[doc = "Indicates that UART receive bus is busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUS_BUSY_A {
    #[doc = "1: Bus is busy"]
    BUSY = 1,
    #[doc = "0: Bus is not busy"]
    IDLE = 0,
}
impl From<BUS_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_BUSY_A {
        match self.bits {
            true => BUS_BUSY_A::BUSY,
            false => BUS_BUSY_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUS_BUSY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUS_BUSY_A::IDLE
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that UART transmit bus is busy"]
    #[inline(always)]
    pub fn transmit_busy(&self) -> TRANSMIT_BUSY_R {
        TRANSMIT_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that UART receive bus is busy"]
    #[inline(always)]
    pub fn receive_busy(&self) -> RECEIVE_BUSY_R {
        RECEIVE_BUSY_R::new(((self.bits >> 1) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
