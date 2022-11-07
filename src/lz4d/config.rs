#[doc = "Register `config` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `config` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - Enable peripheral decompression"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - Enable peripheral decompression"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `suspend` reader - Suspend peripheral decompression"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `suspend` writer - Suspend peripheral decompression"]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `has_checksum` reader - Does this block includes an LZ4 checksum?\n\n Users should read checksum flag from frame descriptor to fill in correct value for this register field."]
pub type HAS_CHECKSUM_R = crate::BitReader<bool>;
#[doc = "Field `has_checksum` writer - Does this block includes an LZ4 checksum?\n\n Users should read checksum flag from frame descriptor to fill in correct value for this register field."]
pub type HAS_CHECKSUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable peripheral decompression"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend peripheral decompression"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Does this block includes an LZ4 checksum?\n\n Users should read checksum flag from frame descriptor to fill in correct value for this register field."]
    #[inline(always)]
    pub fn has_checksum(&self) -> HAS_CHECKSUM_R {
        HAS_CHECKSUM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable peripheral decompression"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Suspend peripheral decompression"]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SUSPEND_W<1> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 4 - Does this block includes an LZ4 checksum?\n\n Users should read checksum flag from frame descriptor to fill in correct value for this register field."]
    #[inline(always)]
    #[must_use]
    pub fn has_checksum(&mut self) -> HAS_CHECKSUM_W<4> {
        HAS_CHECKSUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decompressor peripheral configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
