#[doc = "Register `threshold[%s]` reader"]
pub struct R(crate::R<THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `threshold[%s]` writer"]
pub struct W(crate::W<THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESHOLD_SPEC>;
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
impl From<crate::W<THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `low` reader - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `low` writer - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THRESHOLD_SPEC, u16, u16, 16, O>;
#[doc = "Field `high` reader - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `high` writer - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
pub type HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THRESHOLD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lowest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<0> {
        LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - Highest value for internal counter that sets positive signal to 1 and negative to 0"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<16> {
        HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel internal counter threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](index.html) module"]
pub struct THRESHOLD_SPEC;
impl crate::RegisterSpec for THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [threshold::R](R) reader structure"]
impl crate::Readable for THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [threshold::W](W) writer structure"]
impl crate::Writable for THRESHOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets threshold[%s]
to value 0"]
impl crate::Resettable for THRESHOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
