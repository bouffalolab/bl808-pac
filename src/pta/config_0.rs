#[doc = "Register `config_0` reader"]
pub struct R(crate::R<CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `config_0` writer"]
pub struct W(crate::W<CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_0_SPEC>;
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
impl From<crate::W<CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `function` reader - Enable packet traffic arbitration"]
pub type FUNCTION_R = crate::BitReader<bool>;
#[doc = "Field `function` writer - Enable packet traffic arbitration"]
pub type FUNCTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_0_SPEC, bool, O>;
#[doc = "Field `wifi` reader - ??"]
pub type WIFI_R = crate::BitReader<bool>;
#[doc = "Field `wifi` writer - ??"]
pub type WIFI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_0_SPEC, bool, O>;
#[doc = "Field `priority` reader - ??"]
pub type PRIORITY_R = crate::BitReader<bool>;
#[doc = "Field `priority` writer - ??"]
pub type PRIORITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable packet traffic arbitration"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ??"]
    #[inline(always)]
    pub fn wifi(&self) -> WIFI_R {
        WIFI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ??"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable packet traffic arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<0> {
        FUNCTION_W::new(self)
    }
    #[doc = "Bit 1 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn wifi(&mut self) -> WIFI_W<1> {
        WIFI_W::new(self)
    }
    #[doc = "Bit 4 - ??"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<4> {
        PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitration configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_0](index.html) module"]
pub struct CONFIG_0_SPEC;
impl crate::RegisterSpec for CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_0::R](R) reader structure"]
impl crate::Readable for CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_0::W](W) writer structure"]
impl crate::Writable for CONFIG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config_0 to value 0"]
impl crate::Resettable for CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
