#[doc = "Register `control` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `control` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `password_busy` reader - ??"]
pub type PASSWORD_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `password_trigger` reader - ??"]
pub type PASSWORD_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `password_trigger` writer - ??"]
pub type PASSWORD_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `password_count` reader - ??"]
pub type PASSWORD_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `password_count` writer - ??"]
pub type PASSWORD_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u32, u32, 20, O>;
#[doc = "Field `debug_mode` reader - ??"]
pub type DEBUG_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `debug_enable` reader - Read if debug module is enabled"]
pub type DEBUG_ENABLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - ??"]
    #[inline(always)]
    pub fn password_busy(&self) -> PASSWORD_BUSY_R {
        PASSWORD_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ??"]
    #[inline(always)]
    pub fn password_trigger(&self) -> PASSWORD_TRIGGER_R {
        PASSWORD_TRIGGER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:23 - ??"]
    #[inline(always)]
    pub fn password_count(&self) -> PASSWORD_COUNT_R {
        PASSWORD_COUNT_R::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:27 - ??"]
    #[inline(always)]
    pub fn debug_mode(&self) -> DEBUG_MODE_R {
        DEBUG_MODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Read if debug module is enabled"]
    #[inline(always)]
    pub fn debug_enable(&self) -> DEBUG_ENABLE_R {
        DEBUG_ENABLE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn password_trigger(&mut self) -> PASSWORD_TRIGGER_W<1> {
        PASSWORD_TRIGGER_W::new(self)
    }
    #[doc = "Bits 4:23 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn password_count(&mut self) -> PASSWORD_COUNT_W<4> {
        PASSWORD_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
