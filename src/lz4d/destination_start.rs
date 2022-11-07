#[doc = "Register `destination_start` reader"]
pub struct R(crate::R<DESTINATION_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATION_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATION_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATION_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `destination_start` writer"]
pub struct W(crate::W<DESTINATION_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATION_START_SPEC>;
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
impl From<crate::W<DESTINATION_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATION_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `start` reader - Start address"]
pub type START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `start` writer - Start address"]
pub type START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATION_START_SPEC, u32, u32, 26, O>;
#[doc = "Field `base` reader - Address base"]
pub type BASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `base` writer - Address base"]
pub type BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATION_START_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:25 - Start address"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:31 - Address base"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25 - Start address"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 26:31 - Address base"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<26> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writes destination address before decompression\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destination_start](index.html) module"]
pub struct DESTINATION_START_SPEC;
impl crate::RegisterSpec for DESTINATION_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destination_start::R](R) reader structure"]
impl crate::Readable for DESTINATION_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destination_start::W](W) writer structure"]
impl crate::Writable for DESTINATION_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets destination_start to value 0"]
impl crate::Resettable for DESTINATION_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
