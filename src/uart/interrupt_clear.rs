#[doc = "Register `interrupt_clear` writer"]
pub struct W(crate::W<INTERRUPT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_CLEAR_SPEC>;
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
impl From<crate::W<INTERRUPT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `transmit_transfer` writer - Write 1 to clear transmit transfer finish signal"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_TRANSFER_W;
#[doc = "Field `receive_transfer` writer - Write 1 to clear receive transfer finish signal"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TRANSFER_W;
#[doc = "Field `receive_timeout` writer - Write 1 to clear receive timed-out"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TIMEOUT_W;
#[doc = "Field `receive_parity` writer - Write 1 to clear receive parity check failure"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_PARITY_W;
#[doc = "Field `receive_sync_error` writer - Write 1 to clear receive LIN mode synchronization field error"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_SYNC_ERROR_W;
#[doc = "Field `receive_byte_count` writer - Write 1 to clear receive byte count reached"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_BYTE_COUNT_W;
#[doc = "Field `auto_baudrate_start_bit` writer - Write 1 to clear receive auto baudrate detection finished using start bit"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as AUTO_BAUDRATE_START_BIT_W;
#[doc = "Write 1 to clear receive auto baudrate detection finished using 0x55\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_CLEAR_AW {
    #[doc = "1: Write 1 to clear interrupt state"]
    CLEAR = 1,
}
impl From<INTERRUPT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `auto_baudrate_five_five` writer - Write 1 to clear receive auto baudrate detection finished using 0x55"]
pub type AUTO_BAUDRATE_FIVE_FIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPT_CLEAR_SPEC, INTERRUPT_CLEAR_AW, O>;
impl<'a, const O: u8> AUTO_BAUDRATE_FIVE_FIVE_W<'a, O> {
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INTERRUPT_CLEAR_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear transmit transfer finish signal"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_transfer(&mut self) -> TRANSMIT_TRANSFER_W<0> {
        TRANSMIT_TRANSFER_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to clear receive transfer finish signal"]
    #[inline(always)]
    #[must_use]
    pub fn receive_transfer(&mut self) -> RECEIVE_TRANSFER_W<1> {
        RECEIVE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 4 - Write 1 to clear receive timed-out"]
    #[inline(always)]
    #[must_use]
    pub fn receive_timeout(&mut self) -> RECEIVE_TIMEOUT_W<4> {
        RECEIVE_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 5 - Write 1 to clear receive parity check failure"]
    #[inline(always)]
    #[must_use]
    pub fn receive_parity(&mut self) -> RECEIVE_PARITY_W<5> {
        RECEIVE_PARITY_W::new(self)
    }
    #[doc = "Bit 8 - Write 1 to clear receive LIN mode synchronization field error"]
    #[inline(always)]
    #[must_use]
    pub fn receive_sync_error(&mut self) -> RECEIVE_SYNC_ERROR_W<8> {
        RECEIVE_SYNC_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 to clear receive byte count reached"]
    #[inline(always)]
    #[must_use]
    pub fn receive_byte_count(&mut self) -> RECEIVE_BYTE_COUNT_W<9> {
        RECEIVE_BYTE_COUNT_W::new(self)
    }
    #[doc = "Bit 10 - Write 1 to clear receive auto baudrate detection finished using start bit"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_start_bit(&mut self) -> AUTO_BAUDRATE_START_BIT_W<10> {
        AUTO_BAUDRATE_START_BIT_W::new(self)
    }
    #[doc = "Bit 11 - Write 1 to clear receive auto baudrate detection finished using 0x55"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_five_five(&mut self) -> AUTO_BAUDRATE_FIVE_FIVE_W<11> {
        AUTO_BAUDRATE_FIVE_FIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_clear](index.html) module"]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [interrupt_clear::W](W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
