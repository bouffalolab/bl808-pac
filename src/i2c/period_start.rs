#[doc = "Register `period_start` reader"]
pub struct R(crate::R<PERIOD_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `period_start` writer"]
pub struct W(crate::W<PERIOD_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_START_SPEC>;
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
impl From<crate::W<PERIOD_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `phase[0-3]` reader - Length of start condition phase %s"]
pub type PHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `phase[0-3]` writer - Length of start condition phase %s"]
pub type PHASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERIOD_START_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Length of start condition phase [0-3]"]
    #[inline(always)]
    pub unsafe fn phase(&self, n: u8) -> PHASE_R {
        PHASE_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Length of start condition phase 0"]
    #[inline(always)]
    pub fn phase0(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length of start condition phase 1"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Length of start condition phase 2"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Length of start condition phase 3"]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Length of start condition phase [0-3]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn phase<const O: u8>(&mut self) -> PHASE_W<O> {
        PHASE_W::new(self)
    }
    #[doc = "Bits 0:7 - Length of start condition phase 0"]
    #[inline(always)]
    #[must_use]
    pub fn phase0(&mut self) -> PHASE_W<0> {
        PHASE_W::new(self)
    }
    #[doc = "Bits 8:15 - Length of start condition phase 1"]
    #[inline(always)]
    #[must_use]
    pub fn phase1(&mut self) -> PHASE_W<8> {
        PHASE_W::new(self)
    }
    #[doc = "Bits 16:23 - Length of start condition phase 2"]
    #[inline(always)]
    #[must_use]
    pub fn phase2(&mut self) -> PHASE_W<16> {
        PHASE_W::new(self)
    }
    #[doc = "Bits 24:31 - Length of start condition phase 3"]
    #[inline(always)]
    #[must_use]
    pub fn phase3(&mut self) -> PHASE_W<24> {
        PHASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Duration of start phase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_start](index.html) module"]
pub struct PERIOD_START_SPEC;
impl crate::RegisterSpec for PERIOD_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period_start::R](R) reader structure"]
impl crate::Readable for PERIOD_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period_start::W](W) writer structure"]
impl crate::Writable for PERIOD_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets period_start to value 0x0f0f_0f0f"]
impl crate::Resettable for PERIOD_START_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
