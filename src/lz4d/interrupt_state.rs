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
#[doc = "Field `done` reader - Decompliation finished"]
pub type DONE_R = crate::BitReader<INTERRUPT_STATE_A>;
#[doc = "Decompliation finished\n\nValue on reset: 0"]
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
impl DONE_R {
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
#[doc = "Field `error` reader - Error occurred while decompliation"]
pub use DONE_R as ERROR_R;
impl R {
    #[doc = "Bit 0 - Decompliation finished"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error occurred while decompliation"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
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
