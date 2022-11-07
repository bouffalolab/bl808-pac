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
#[doc = "Field `threshold_low[0-3]` reader - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_R = crate::BitReader<INTERRUPT_MASK_A>;
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 1"]
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
impl THRESHOLD_LOW_R {
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
#[doc = "Field `threshold_low[0-3]` writer - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPT_MASK_SPEC, INTERRUPT_MASK_A, O>;
impl<'a, const O: u8> THRESHOLD_LOW_W<'a, O> {
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
#[doc = "Field `threshold_high[0-3]` reader - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_R as THRESHOLD_HIGH_R;
#[doc = "Field `interrupt_period` reader - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_R as INTERRUPT_PERIOD_R;
#[doc = "Field `external_break` reader - External break signal occurred"]
pub use THRESHOLD_LOW_R as EXTERNAL_BREAK_R;
#[doc = "Field `repeat` reader - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_R as REPEAT_R;
#[doc = "Field `threshold_high[0-3]` writer - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_W as THRESHOLD_HIGH_W;
#[doc = "Field `interrupt_period` writer - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_W as INTERRUPT_PERIOD_W;
#[doc = "Field `external_break` writer - External break signal occurred"]
pub use THRESHOLD_LOW_W as EXTERNAL_BREAK_W;
#[doc = "Field `repeat` writer - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_W as REPEAT_W;
impl R {
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub unsafe fn threshold_low(&self, n: u8) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low0(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low1(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low2(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low3(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub unsafe fn threshold_high(&self, n: u8) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high0(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high1(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high2(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high3(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Intenal counter for channel have exceeded interrupt cycle threshold"]
    #[inline(always)]
    pub fn interrupt_period(&self) -> INTERRUPT_PERIOD_R {
        INTERRUPT_PERIOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External break signal occurred"]
    #[inline(always)]
    pub fn external_break(&self) -> EXTERNAL_BREAK_R {
        EXTERNAL_BREAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral group have completed one repeat cycle"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn threshold_low<const O: u8>(&mut self) -> THRESHOLD_LOW_W<O> {
        THRESHOLD_LOW_W::new(self)
    }
    #[doc = "Bit 0 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low0(&mut self) -> THRESHOLD_LOW_W<0> {
        THRESHOLD_LOW_W::new(self)
    }
    #[doc = "Bit 2 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low1(&mut self) -> THRESHOLD_LOW_W<2> {
        THRESHOLD_LOW_W::new(self)
    }
    #[doc = "Bit 4 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low2(&mut self) -> THRESHOLD_LOW_W<4> {
        THRESHOLD_LOW_W::new(self)
    }
    #[doc = "Bit 6 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low3(&mut self) -> THRESHOLD_LOW_W<6> {
        THRESHOLD_LOW_W::new(self)
    }
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn threshold_high<const O: u8>(&mut self) -> THRESHOLD_HIGH_W<O> {
        THRESHOLD_HIGH_W::new(self)
    }
    #[doc = "Bit 1 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high0(&mut self) -> THRESHOLD_HIGH_W<1> {
        THRESHOLD_HIGH_W::new(self)
    }
    #[doc = "Bit 3 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high1(&mut self) -> THRESHOLD_HIGH_W<3> {
        THRESHOLD_HIGH_W::new(self)
    }
    #[doc = "Bit 5 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high2(&mut self) -> THRESHOLD_HIGH_W<5> {
        THRESHOLD_HIGH_W::new(self)
    }
    #[doc = "Bit 7 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high3(&mut self) -> THRESHOLD_HIGH_W<7> {
        THRESHOLD_HIGH_W::new(self)
    }
    #[doc = "Bit 8 - Intenal counter for channel have exceeded interrupt cycle threshold"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_period(&mut self) -> INTERRUPT_PERIOD_W<8> {
        INTERRUPT_PERIOD_W::new(self)
    }
    #[doc = "Bit 9 - External break signal occurred"]
    #[inline(always)]
    #[must_use]
    pub fn external_break(&mut self) -> EXTERNAL_BREAK_W<9> {
        EXTERNAL_BREAK_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral group have completed one repeat cycle"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<10> {
        REPEAT_W::new(self)
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
#[doc = "`reset()` method sets interrupt_mask to value 0x7f"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
