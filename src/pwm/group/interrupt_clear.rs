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
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 0"]
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
#[doc = "Field `threshold_low[0-3]` writer - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPT_CLEAR_SPEC, INTERRUPT_CLEAR_AW, O>;
impl<'a, const O: u8> THRESHOLD_LOW_W<'a, O> {
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INTERRUPT_CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `threshold_high[0-3]` writer - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_W as THRESHOLD_HIGH_W;
#[doc = "Field `interrupt_period` writer - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_W as INTERRUPT_PERIOD_W;
#[doc = "Field `external_break` writer - External break signal occurred"]
pub use THRESHOLD_LOW_W as EXTERNAL_BREAK_W;
#[doc = "Field `repeat` writer - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_W as REPEAT_W;
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
