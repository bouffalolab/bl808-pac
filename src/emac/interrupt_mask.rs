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
#[doc = "Field `buffer_transmitted` reader - Buffer transmitted interrupt mask"]
pub use CONTROL_RECEIVE_R as BUFFER_TRANSMITTED_R;
#[doc = "Field `transmit_error` reader - Transmit error interrupt mask"]
pub use CONTROL_RECEIVE_R as TRANSMIT_ERROR_R;
#[doc = "Field `frame_received` reader - Frame received interrupt mask"]
pub use CONTROL_RECEIVE_R as FRAME_RECEIVED_R;
#[doc = "Field `receive_error` reader - Receive error interrupt mask"]
pub use CONTROL_RECEIVE_R as RECEIVE_ERROR_R;
#[doc = "Field `busy` reader - Lack of buffer interrupt mask"]
pub use CONTROL_RECEIVE_R as BUSY_R;
#[doc = "Field `control_transmit` reader - Control frame transmitted interrupt mask"]
pub use CONTROL_RECEIVE_R as CONTROL_TRANSMIT_R;
#[doc = "Field `buffer_transmitted` writer - Buffer transmitted interrupt mask"]
pub use CONTROL_RECEIVE_W as BUFFER_TRANSMITTED_W;
#[doc = "Field `transmit_error` writer - Transmit error interrupt mask"]
pub use CONTROL_RECEIVE_W as TRANSMIT_ERROR_W;
#[doc = "Field `frame_received` writer - Frame received interrupt mask"]
pub use CONTROL_RECEIVE_W as FRAME_RECEIVED_W;
#[doc = "Field `receive_error` writer - Receive error interrupt mask"]
pub use CONTROL_RECEIVE_W as RECEIVE_ERROR_W;
#[doc = "Field `busy` writer - Lack of buffer interrupt mask"]
pub use CONTROL_RECEIVE_W as BUSY_W;
#[doc = "Field `control_transmit` writer - Control frame transmitted interrupt mask"]
pub use CONTROL_RECEIVE_W as CONTROL_TRANSMIT_W;
#[doc = "Field `control_receive` reader - Control frame received interrupt mask"]
pub type CONTROL_RECEIVE_R = crate::BitReader<INTERRUPT_MASK_A>;
#[doc = "Control frame received interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK_A {
    #[doc = "1: Mask interrupt"]
    MASK = 1,
    #[doc = "0: Unmask interrupt"]
    UNMASK = 0,
}
impl From<INTERRUPT_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl CONTROL_RECEIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPT_MASK_A {
        match self.bits {
            true => INTERRUPT_MASK_A::MASK,
            false => INTERRUPT_MASK_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INTERRUPT_MASK_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == INTERRUPT_MASK_A::UNMASK
    }
}
#[doc = "Field `control_receive` writer - Control frame received interrupt mask"]
pub type CONTROL_RECEIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPT_MASK_SPEC, INTERRUPT_MASK_A, O>;
impl<'a, const O: u8> CONTROL_RECEIVE_W<'a, O> {
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INTERRUPT_MASK_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INTERRUPT_MASK_A::UNMASK)
    }
}
impl R {
    #[doc = "Bit 0 - Buffer transmitted interrupt mask"]
    #[inline(always)]
    pub fn buffer_transmitted(&self) -> BUFFER_TRANSMITTED_R {
        BUFFER_TRANSMITTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit error interrupt mask"]
    #[inline(always)]
    pub fn transmit_error(&self) -> TRANSMIT_ERROR_R {
        TRANSMIT_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame received interrupt mask"]
    #[inline(always)]
    pub fn frame_received(&self) -> FRAME_RECEIVED_R {
        FRAME_RECEIVED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive error interrupt mask"]
    #[inline(always)]
    pub fn receive_error(&self) -> RECEIVE_ERROR_R {
        RECEIVE_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lack of buffer interrupt mask"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control frame transmitted interrupt mask"]
    #[inline(always)]
    pub fn control_transmit(&self) -> CONTROL_TRANSMIT_R {
        CONTROL_TRANSMIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control frame received interrupt mask"]
    #[inline(always)]
    pub fn control_receive(&self) -> CONTROL_RECEIVE_R {
        CONTROL_RECEIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_transmitted(&mut self) -> BUFFER_TRANSMITTED_W<0> {
        BUFFER_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 1 - Transmit error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_error(&mut self) -> TRANSMIT_ERROR_W<1> {
        TRANSMIT_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Frame received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn frame_received(&mut self) -> FRAME_RECEIVED_W<2> {
        FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 3 - Receive error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_error(&mut self) -> RECEIVE_ERROR_W<3> {
        RECEIVE_ERROR_W::new(self)
    }
    #[doc = "Bit 4 - Lack of buffer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 5 - Control frame transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn control_transmit(&mut self) -> CONTROL_TRANSMIT_W<5> {
        CONTROL_TRANSMIT_W::new(self)
    }
    #[doc = "Bit 6 - Control frame received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn control_receive(&mut self) -> CONTROL_RECEIVE_W<6> {
        CONTROL_RECEIVE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_mask to value 0"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
