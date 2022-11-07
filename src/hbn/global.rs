#[doc = "Register `global` reader"]
pub struct R(crate::R<GLOBAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `global` writer"]
pub struct W(crate::W<GLOBAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBAL_SPEC>;
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
impl From<crate::W<GLOBAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBAL_SPEC>) -> Self {
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
#[doc = "Global hibernate configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global](index.html) module"]
pub struct GLOBAL_SPEC;
impl crate::RegisterSpec for GLOBAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [global::R](R) reader structure"]
impl crate::Readable for GLOBAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [global::W](W) writer structure"]
impl crate::Writable for GLOBAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets global to value 0"]
impl crate::Resettable for GLOBAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
