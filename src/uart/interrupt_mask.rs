#[doc = "Register `interrupt_mask` reader"]
pub struct R(crate::R<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interrupt_mask` writer"]
pub struct W(crate::W<INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO overflow or underflow interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_FIFO_ERROR_A {
    #[doc = "1: Mask interrupt"]
    MASK = 1,
    #[doc = "0: Unmask interrupt"]
    UNMASK = 0,
}
impl From<RECEIVE_FIFO_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_FIFO_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `receive_fifo_error` reader - Receive FIFO overflow or underflow interrupt mask"]
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
            true => RECEIVE_FIFO_ERROR_A::MASK,
            false => RECEIVE_FIFO_ERROR_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == RECEIVE_FIFO_ERROR_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == RECEIVE_FIFO_ERROR_A::UNMASK
    }
}
impl core::ops::Deref for RECEIVE_FIFO_ERROR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `receive_fifo_error` writer - Receive FIFO overflow or underflow interrupt mask"]
pub struct RECEIVE_FIFO_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_FIFO_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_FIFO_ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RECEIVE_FIFO_ERROR_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RECEIVE_FIFO_ERROR_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Transmit FIFO overflow or underflow interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_FIFO_ERROR_A;
#[doc = "Field `transmit_fifo_error` reader - Transmit FIFO overflow or underflow interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_FIFO_ERROR_R;
#[doc = "Field `transmit_fifo_error` writer - Transmit FIFO overflow or underflow interrupt mask"]
pub struct TRANSMIT_FIFO_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_FIFO_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_FIFO_ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TRANSMIT_FIFO_ERROR_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TRANSMIT_FIFO_ERROR_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Receive parity check failure interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_PARITY_A;
#[doc = "Field `receive_parity` reader - Receive parity check failure interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_PARITY_R;
#[doc = "Field `receive_parity` writer - Receive parity check failure interrupt mask"]
pub struct RECEIVE_PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_PARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RECEIVE_PARITY_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RECEIVE_PARITY_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Receive timed-out interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_TIMEOUT_A;
#[doc = "Field `receive_timeout` reader - Receive timed-out interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_TIMEOUT_R;
#[doc = "Field `receive_timeout` writer - Receive timed-out interrupt mask"]
pub struct RECEIVE_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_TIMEOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RECEIVE_TIMEOUT_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RECEIVE_TIMEOUT_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Receive FIFO ready signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_FIFO_READY_A;
#[doc = "Field `receive_fifo_ready` reader - Receive FIFO ready signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_FIFO_READY_R;
#[doc = "Field `receive_fifo_ready` writer - Receive FIFO ready signal interrupt mask"]
pub struct RECEIVE_FIFO_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_FIFO_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_FIFO_READY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RECEIVE_FIFO_READY_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RECEIVE_FIFO_READY_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Transmit FIFO ready signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_FIFO_READY_A;
#[doc = "Field `transmit_fifo_ready` reader - Transmit FIFO ready signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_FIFO_READY_R;
#[doc = "Field `transmit_fifo_ready` writer - Transmit FIFO ready signal interrupt mask"]
pub struct TRANSMIT_FIFO_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_FIFO_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_FIFO_READY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TRANSMIT_FIFO_READY_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TRANSMIT_FIFO_READY_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Receive transfer finish signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as RECEIVE_TRANSFER_A;
#[doc = "Field `receive_transfer` reader - Receive transfer finish signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as RECEIVE_TRANSFER_R;
#[doc = "Field `receive_transfer` writer - Receive transfer finish signal interrupt mask"]
pub struct RECEIVE_TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_TRANSFER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_TRANSFER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RECEIVE_TRANSFER_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RECEIVE_TRANSFER_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Transmit transfer finish signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_A as TRANSMIT_TRANSFER_A;
#[doc = "Field `transmit_transfer` reader - Transmit transfer finish signal interrupt mask"]
pub use RECEIVE_FIFO_ERROR_R as TRANSMIT_TRANSFER_R;
#[doc = "Field `transmit_transfer` writer - Transmit transfer finish signal interrupt mask"]
pub struct TRANSMIT_TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_TRANSFER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_TRANSFER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TRANSMIT_TRANSFER_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TRANSMIT_TRANSFER_A::UNMASK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_error(&self) -> RECEIVE_FIFO_ERROR_R {
        RECEIVE_FIFO_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_error(&self) -> TRANSMIT_FIFO_ERROR_R {
        TRANSMIT_FIFO_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt mask"]
    #[inline(always)]
    pub fn receive_parity(&self) -> RECEIVE_PARITY_R {
        RECEIVE_PARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt mask"]
    #[inline(always)]
    pub fn receive_timeout(&self) -> RECEIVE_TIMEOUT_R {
        RECEIVE_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_ready(&self) -> RECEIVE_FIFO_READY_R {
        RECEIVE_FIFO_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_ready(&self) -> TRANSMIT_FIFO_READY_R {
        TRANSMIT_FIFO_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive transfer finish signal interrupt mask"]
    #[inline(always)]
    pub fn receive_transfer(&self) -> RECEIVE_TRANSFER_R {
        RECEIVE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit transfer finish signal interrupt mask"]
    #[inline(always)]
    pub fn transmit_transfer(&self) -> TRANSMIT_TRANSFER_R {
        TRANSMIT_TRANSFER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_error(&mut self) -> RECEIVE_FIFO_ERROR_W {
        RECEIVE_FIFO_ERROR_W { w: self }
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_error(&mut self) -> TRANSMIT_FIFO_ERROR_W {
        TRANSMIT_FIFO_ERROR_W { w: self }
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt mask"]
    #[inline(always)]
    pub fn receive_parity(&mut self) -> RECEIVE_PARITY_W {
        RECEIVE_PARITY_W { w: self }
    }
    #[doc = "Bit 4 - Receive timed-out interrupt mask"]
    #[inline(always)]
    pub fn receive_timeout(&mut self) -> RECEIVE_TIMEOUT_W {
        RECEIVE_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_ready(&mut self) -> RECEIVE_FIFO_READY_W {
        RECEIVE_FIFO_READY_W { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_ready(&mut self) -> TRANSMIT_FIFO_READY_W {
        TRANSMIT_FIFO_READY_W { w: self }
    }
    #[doc = "Bit 1 - Receive transfer finish signal interrupt mask"]
    #[inline(always)]
    pub fn receive_transfer(&mut self) -> RECEIVE_TRANSFER_W {
        RECEIVE_TRANSFER_W { w: self }
    }
    #[doc = "Bit 0 - Transmit transfer finish signal interrupt mask"]
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
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_mask](index.html) module"]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_mask::R](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets interrupt_mask to value 0xff"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
