#[doc = "Register `interrupt_state` reader"]
pub struct R(crate::R<INTERRUPT_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receive FIFO overflow or underflow occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_FIFO_ERROR_A {
    #[doc = "1: Has interrupt"]
    HAS_INTERRUPT = 1,
    #[doc = "0: No interrupt occurred"]
    NO_INTERRUPT = 0,
}
impl From<RECEIVE_FIFO_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_FIFO_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_fifo_error` reader - Receive FIFO overflow or underflow occurred"]
pub struct RECEIVE_FIFO_ERROR_R(crate::FieldReader<bool>);
impl RECEIVE_FIFO_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE_FIFO_ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE_FIFO_ERROR_A {
        match self.bits {
            true => RECEIVE_FIFO_ERROR_A::HAS_INTERRUPT,
            false => RECEIVE_FIFO_ERROR_A::NO_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT`"]
    #[inline(always)]
    pub fn is_has_interrupt(&self) -> bool {
        **self == RECEIVE_FIFO_ERROR_A::HAS_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == RECEIVE_FIFO_ERROR_A::NO_INTERRUPT
    }
}
impl core::ops::Deref for RECEIVE_FIFO_ERROR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO overflow or underflow occurred"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_FIFO_ERROR_A;
#[doc = "Receive parity check failure occurred"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_PARITY_A;
#[doc = "Receive timed-out interrupt occurred"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_TIMEOUT_A;
#[doc = "Receive FIFO ready signal raised"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_FIFO_READY_A;
#[doc = "Transmit FIFO ready signal raised"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_FIFO_READY_A;
#[doc = "Receive transfer finish signal raised"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_TRANSFER_A;
#[doc = "Transmit transfer finish signal raised"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_TRANSFER_A;
#[doc = "Field `transmit_fifo_error` reader - Transmit FIFO overflow or underflow occurred"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_FIFO_ERROR_R;
#[doc = "Field `receive_parity` reader - Receive parity check failure occurred"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_PARITY_R;
#[doc = "Field `receive_timeout` reader - Receive timed-out interrupt occurred"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_TIMEOUT_R;
#[doc = "Field `receive_fifo_ready` reader - Receive FIFO ready signal raised"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_FIFO_READY_R;
#[doc = "Field `transmit_fifo_ready` reader - Transmit FIFO ready signal raised"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_FIFO_READY_R;
#[doc = "Field `receive_transfer` reader - Receive transfer finish signal raised"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_TRANSFER_R;
#[doc = "Field `transmit_transfer` reader - Transmit transfer finish signal raised"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_TRANSFER_R;
impl R {
    #[doc = "Bit 7 - Receive FIFO overflow or underflow occurred"]
    #[inline(always)]
    pub fn receive_fifo_error(&self) -> RECEIVE_FIFO_ERROR_R {
        RECEIVE_FIFO_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow occurred"]
    #[inline(always)]
    pub fn transmit_fifo_error(&self) -> TRANSMIT_FIFO_ERROR_R {
        TRANSMIT_FIFO_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive parity check failure occurred"]
    #[inline(always)]
    pub fn receive_parity(&self) -> RECEIVE_PARITY_R {
        RECEIVE_PARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt occurred"]
    #[inline(always)]
    pub fn receive_timeout(&self) -> RECEIVE_TIMEOUT_R {
        RECEIVE_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal raised"]
    #[inline(always)]
    pub fn receive_fifo_ready(&self) -> RECEIVE_FIFO_READY_R {
        RECEIVE_FIFO_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal raised"]
    #[inline(always)]
    pub fn transmit_fifo_ready(&self) -> TRANSMIT_FIFO_READY_R {
        TRANSMIT_FIFO_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive transfer finish signal raised"]
    #[inline(always)]
    pub fn receive_transfer(&self) -> RECEIVE_TRANSFER_R {
        RECEIVE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit transfer finish signal raised"]
    #[inline(always)]
    pub fn transmit_transfer(&self) -> TRANSMIT_TRANSFER_R {
        TRANSMIT_TRANSFER_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_state](index.html) module"]
pub struct INTERRUPT_STATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_state::R](R) reader structure"]
impl crate::Readable for INTERRUPT_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets interrupt_state to value 0"]
impl crate::Resettable for INTERRUPT_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
