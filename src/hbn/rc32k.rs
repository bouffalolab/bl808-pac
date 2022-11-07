#[doc = "Register `rc32k` reader"]
pub struct R(crate::R<RC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32k` writer"]
pub struct W(crate::W<RC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32K_SPEC>;
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
impl From<crate::W<RC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32K_SPEC>) -> Self {
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
#[doc = "32-kHz internal RC oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32k](index.html) module"]
pub struct RC32K_SPEC;
impl crate::RegisterSpec for RC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32k::R](R) reader structure"]
impl crate::Readable for RC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32k::W](W) writer structure"]
impl crate::Writable for RC32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32k to value 0"]
impl crate::Resettable for RC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
