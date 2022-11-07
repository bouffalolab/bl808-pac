#[doc = "Register `chip_inform` reader"]
pub struct R(crate::R<CHIP_INFORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_INFORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_INFORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_INFORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `chip_inform` writer"]
pub struct W(crate::W<CHIP_INFORM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIP_INFORM_SPEC>;
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
impl From<crate::W<CHIP_INFORM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIP_INFORM_SPEC>) -> Self {
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
#[doc = "Chip information register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_inform](index.html) module"]
pub struct CHIP_INFORM_SPEC;
impl crate::RegisterSpec for CHIP_INFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_inform::R](R) reader structure"]
impl crate::Readable for CHIP_INFORM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chip_inform::W](W) writer structure"]
impl crate::Writable for CHIP_INFORM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets chip_inform to value 0"]
impl crate::Resettable for CHIP_INFORM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
