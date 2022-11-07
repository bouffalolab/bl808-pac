#[doc = "Register `backed_gap` reader"]
pub struct R(crate::R<BACKED_GAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKED_GAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKED_GAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKED_GAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `backed_gap` writer"]
pub struct W(crate::W<BACKED_GAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKED_GAP_SPEC>;
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
impl From<crate::W<BACKED_GAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKED_GAP_SPEC>) -> Self {
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
#[doc = "Back-to-back inter-packet gap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backed_gap](index.html) module"]
pub struct BACKED_GAP_SPEC;
impl crate::RegisterSpec for BACKED_GAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backed_gap::R](R) reader structure"]
impl crate::Readable for BACKED_GAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backed_gap::W](W) writer structure"]
impl crate::Writable for BACKED_GAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets backed_gap to value 0"]
impl crate::Resettable for BACKED_GAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
