#[doc = "Register `sys_config_1` reader"]
pub struct R(crate::R<SYS_CONFIG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CONFIG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CONFIG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CONFIG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sys_config_1` writer"]
pub struct W(crate::W<SYS_CONFIG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CONFIG_1_SPEC>;
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
impl From<crate::W<SYS_CONFIG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CONFIG_1_SPEC>) -> Self {
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
#[doc = "System configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_config_1](index.html) module"]
pub struct SYS_CONFIG_1_SPEC;
impl crate::RegisterSpec for SYS_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_config_1::R](R) reader structure"]
impl crate::Readable for SYS_CONFIG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_config_1::W](W) writer structure"]
impl crate::Writable for SYS_CONFIG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sys_config_1 to value 0"]
impl crate::Resettable for SYS_CONFIG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}