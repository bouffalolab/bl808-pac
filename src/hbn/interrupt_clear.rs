#[doc = "Register `interrupt_clear` reader"]
pub struct R(crate::R<INTERRUPT_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
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
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear hibernate interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_clear](index.html) module"]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_clear::R](R) reader structure"]
impl crate::Readable for INTERRUPT_CLEAR_SPEC {
    type Reader = R;
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
