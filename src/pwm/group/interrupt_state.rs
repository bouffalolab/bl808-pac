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
#[doc = "Field `threshold_low[0-3]` reader - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_R = crate::BitReader<INTERRUPT_STATE_A>;
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_STATE_A {
    #[doc = "1: Has interrupt"]
    HAS_INTERRUPT = 1,
    #[doc = "0: No interrupt occurred"]
    NO_INTERRUPT = 0,
}
impl From<INTERRUPT_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl THRESHOLD_LOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPT_STATE_A {
        match self.bits {
            true => INTERRUPT_STATE_A::HAS_INTERRUPT,
            false => INTERRUPT_STATE_A::NO_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT`"]
    #[inline(always)]
    pub fn is_has_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::HAS_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::NO_INTERRUPT
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
    const RESET_VALUE: Self::Ux = 0;
}
