#[doc = "Register `wifi_receive` reader"]
pub struct R(crate::R<WIFI_RECEIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_RECEIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_RECEIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_RECEIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_receive` writer"]
pub struct W(crate::W<WIFI_RECEIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_RECEIVE_SPEC>;
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
impl From<crate::W<WIFI_RECEIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_RECEIVE_SPEC>) -> Self {
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
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_receive](index.html) module"]
pub struct WIFI_RECEIVE_SPEC;
impl crate::RegisterSpec for WIFI_RECEIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_receive::R](R) reader structure"]
impl crate::Readable for WIFI_RECEIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_receive::W](W) writer structure"]
impl crate::Writable for WIFI_RECEIVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_receive to value 0"]
impl crate::Resettable for WIFI_RECEIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
