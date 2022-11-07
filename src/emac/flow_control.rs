#[doc = "Register `flow_control` reader"]
pub struct R(crate::R<FLOW_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `flow_control` writer"]
pub struct W(crate::W<FLOW_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_CONTROL_SPEC>;
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
impl From<crate::W<FLOW_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_CONTROL_SPEC>) -> Self {
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
#[doc = "Control frame function register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_control](index.html) module"]
pub struct FLOW_CONTROL_SPEC;
impl crate::RegisterSpec for FLOW_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow_control::R](R) reader structure"]
impl crate::Readable for FLOW_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow_control::W](W) writer structure"]
impl crate::Writable for FLOW_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets flow_control to value 0"]
impl crate::Resettable for FLOW_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
