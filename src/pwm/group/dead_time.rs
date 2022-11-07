#[doc = "Register `dead_time` reader"]
pub struct R(crate::R<DEAD_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEAD_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEAD_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEAD_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dead_time` writer"]
pub struct W(crate::W<DEAD_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEAD_TIME_SPEC>;
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
impl From<crate::W<DEAD_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEAD_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `channel[0-3]` reader - Dead time for each channel in cycles"]
pub type CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `channel[0-3]` writer - Dead time for each channel in cycles"]
pub type CHANNEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEAD_TIME_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Dead time for each channel in cycles"]
    #[inline(always)]
    pub unsafe fn channel(&self, n: u8) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn channel<const O: u8>(&mut self) -> CHANNEL_W<O> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bits 0:7 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> CHANNEL_W<0> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> CHANNEL_W<8> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> CHANNEL_W<16> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bits 24:31 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL_W<24> {
        CHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead time for each channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dead_time](index.html) module"]
pub struct DEAD_TIME_SPEC;
impl crate::RegisterSpec for DEAD_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dead_time::R](R) reader structure"]
impl crate::Readable for DEAD_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dead_time::W](W) writer structure"]
impl crate::Writable for DEAD_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dead_time to value 0"]
impl crate::Resettable for DEAD_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
