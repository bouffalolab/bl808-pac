#[doc = "Register `time_hi` reader"]
pub struct R(crate::R<TIME_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `time_hi` writer"]
pub struct W(crate::W<TIME_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_HI_SPEC>;
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
impl From<crate::W<TIME_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_HI_SPEC>) -> Self {
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
#[doc = "High bits of hibernate time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_hi](index.html) module"]
pub struct TIME_HI_SPEC;
impl crate::RegisterSpec for TIME_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_hi::R](R) reader structure"]
impl crate::Readable for TIME_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_hi::W](W) writer structure"]
impl crate::Writable for TIME_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets time_hi to value 0"]
impl crate::Resettable for TIME_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
