#[doc = "Register `interrupt_enable` reader"]
pub struct R(crate::R<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interrupt_enable` writer"]
pub struct W(crate::W<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_ENABLE_SPEC>;
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
impl From<crate::W<INTERRUPT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `done` reader - Decompliation finished"]
pub type DONE_R = crate::BitReader<INTERRUPT_ENABLE_A>;
#[doc = "Decompliation finished\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_ENABLE_A {
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
}
impl From<INTERRUPT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPT_ENABLE_A {
        match self.bits {
            true => INTERRUPT_ENABLE_A::ENABLE,
            false => INTERRUPT_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::DISABLE
    }
}
#[doc = "Field `done` writer - Decompliation finished"]
pub type DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, INTERRUPT_ENABLE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTERRUPT_ENABLE_A::ENABLE)
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTERRUPT_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `error` reader - Error occurred while decompliation"]
pub use DONE_R as ERROR_R;
#[doc = "Field `error` writer - Error occurred while decompliation"]
pub use DONE_W as ERROR_W;
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
impl W {
    #[doc = "Bit 0 - Decompliation finished"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Error occurred while decompliation"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<1> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_enable](index.html) module"]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_enable::R](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_enable to value 0x03"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
