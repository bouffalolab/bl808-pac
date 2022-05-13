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
#[doc = "Write 1 to clear receive parity check failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_PARITY_AW {
    #[doc = "1: Write 1 to clear interrupt state"]
    CLEAR = 1,
}
impl From<RECEIVE_PARITY_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_PARITY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_parity` writer - Write 1 to clear receive parity check failure"]
pub struct RECEIVE_PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_PARITY_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE_PARITY_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Write 1 to clear receive timed-out"]
pub use RECEIVE_PARITY_AW as RECEIVE_TIMEOUT_AW;
#[doc = "Field `receive_timeout` writer - Write 1 to clear receive timed-out"]
pub struct RECEIVE_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_TIMEOUT_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE_TIMEOUT_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Write 1 to clear receive transfer finish signal"]
pub use RECEIVE_PARITY_AW as RECEIVE_TRANSFER_AW;
#[doc = "Field `receive_transfer` writer - Write 1 to clear receive transfer finish signal"]
pub struct RECEIVE_TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_TRANSFER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_TRANSFER_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE_TRANSFER_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Write 1 to clear transmit transfer finish signal"]
pub use RECEIVE_PARITY_AW as TRANSMIT_TRANSFER_AW;
#[doc = "Field `transmit_transfer` writer - Write 1 to clear transmit transfer finish signal"]
pub struct TRANSMIT_TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_TRANSFER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_TRANSFER_AW) -> &'a mut W {
        unsafe { self.bit(variant.into()) }
    }
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TRANSMIT_TRANSFER_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 5 - Write 1 to clear receive parity check failure"]
    #[inline(always)]
    pub fn receive_parity(&mut self) -> RECEIVE_PARITY_W {
        RECEIVE_PARITY_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 to clear receive timed-out"]
    #[inline(always)]
    pub fn receive_timeout(&mut self) -> RECEIVE_TIMEOUT_W {
        RECEIVE_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 to clear receive transfer finish signal"]
    #[inline(always)]
    pub fn receive_transfer(&mut self) -> RECEIVE_TRANSFER_W {
        RECEIVE_TRANSFER_W { w: self }
    }
    #[doc = "Bit 0 - Write 1 to clear transmit transfer finish signal"]
    #[inline(always)]
    pub fn transmit_transfer(&mut self) -> TRANSMIT_TRANSFER_W {
        TRANSMIT_TRANSFER_W { w: self }
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
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
