#[doc = "Register `debug_config_4` reader"]
pub struct R(crate::R<DEBUG_CONFIG_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CONFIG_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CONFIG_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CONFIG_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `debug_config_4` writer"]
pub struct W(crate::W<DEBUG_CONFIG_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CONFIG_4_SPEC>;
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
impl From<crate::W<DEBUG_CONFIG_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CONFIG_4_SPEC>) -> Self {
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
#[doc = "Debug configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_config_4](index.html) module"]
pub struct DEBUG_CONFIG_4_SPEC;
impl crate::RegisterSpec for DEBUG_CONFIG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_config_4::R](R) reader structure"]
impl crate::Readable for DEBUG_CONFIG_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_config_4::W](W) writer structure"]
impl crate::Writable for DEBUG_CONFIG_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug_config_4 to value 0"]
impl crate::Resettable for DEBUG_CONFIG_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
