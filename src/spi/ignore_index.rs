#[doc = "Register `ignore_index` reader"]
pub struct R(crate::R<IGNORE_INDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IGNORE_INDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IGNORE_INDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IGNORE_INDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ignore_index` writer"]
pub struct W(crate::W<IGNORE_INDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IGNORE_INDEX_SPEC>;
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
impl From<crate::W<IGNORE_INDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IGNORE_INDEX_SPEC>) -> Self {
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
#[doc = "Receive ignore index configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ignore_index](index.html) module"]
pub struct IGNORE_INDEX_SPEC;
impl crate::RegisterSpec for IGNORE_INDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ignore_index::R](R) reader structure"]
impl crate::Readable for IGNORE_INDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ignore_index::W](W) writer structure"]
impl crate::Writable for IGNORE_INDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ignore_index to value 0"]
impl crate::Resettable for IGNORE_INDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
