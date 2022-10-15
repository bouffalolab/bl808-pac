#[doc = "Register `touch_raw[%s]` reader"]
pub struct R(crate::R<TOUCH_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `touch_raw[%s]` writer"]
pub struct W(crate::W<TOUCH_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_RAW_SPEC>;
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
impl From<crate::W<TOUCH_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_RAW_SPEC>) -> Self {
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
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_raw](index.html) module"]
pub struct TOUCH_RAW_SPEC;
impl crate::RegisterSpec for TOUCH_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_raw::R](R) reader structure"]
impl crate::Readable for TOUCH_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_raw::W](W) writer structure"]
impl crate::Writable for TOUCH_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets touch_raw[%s]
to value 0"]
impl crate::Resettable for TOUCH_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}