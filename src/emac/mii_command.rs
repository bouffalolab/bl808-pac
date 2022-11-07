#[doc = "Register `mii_command` reader"]
pub struct R(crate::R<MII_COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mii_command` writer"]
pub struct W(crate::W<MII_COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_COMMAND_SPEC>;
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
impl From<crate::W<MII_COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_COMMAND_SPEC>) -> Self {
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
#[doc = "MII control data, read and scan state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_command](index.html) module"]
pub struct MII_COMMAND_SPEC;
impl crate::RegisterSpec for MII_COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mii_command::R](R) reader structure"]
impl crate::Readable for MII_COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_command::W](W) writer structure"]
impl crate::Writable for MII_COMMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mii_command to value 0"]
impl crate::Resettable for MII_COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
