#[doc = "Register `base_clock` reader"]
pub struct R(crate::R<BASE_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `base_clock` writer"]
pub struct W(crate::W<BASE_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_CLOCK_SPEC>;
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
impl From<crate::W<BASE_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `divide_low` reader - Lower half of base clock dividing factor"]
pub type DIVIDE_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `divide_low` writer - Lower half of base clock dividing factor"]
pub type DIVIDE_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BASE_CLOCK_SPEC, u16, u16, 12, O>;
#[doc = "Field `divide_high` reader - Higher half of base clock dividing factor"]
pub type DIVIDE_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `divide_high` writer - Higher half of base clock dividing factor"]
pub type DIVIDE_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BASE_CLOCK_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Lower half of base clock dividing factor"]
    #[inline(always)]
    pub fn divide_low(&self) -> DIVIDE_LOW_R {
        DIVIDE_LOW_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Higher half of base clock dividing factor"]
    #[inline(always)]
    pub fn divide_high(&self) -> DIVIDE_HIGH_R {
        DIVIDE_HIGH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Lower half of base clock dividing factor"]
    #[inline(always)]
    #[must_use]
    pub fn divide_low(&mut self) -> DIVIDE_LOW_W<0> {
        DIVIDE_LOW_W::new(self)
    }
    #[doc = "Bits 16:27 - Higher half of base clock dividing factor"]
    #[inline(always)]
    #[must_use]
    pub fn divide_high(&mut self) -> DIVIDE_HIGH_W<16> {
        DIVIDE_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_clock](index.html) module"]
pub struct BASE_CLOCK_SPEC;
impl crate::RegisterSpec for BASE_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base_clock::R](R) reader structure"]
impl crate::Readable for BASE_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base_clock::W](W) writer structure"]
impl crate::Writable for BASE_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets base_clock to value 0x0001_0001"]
impl crate::Resettable for BASE_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
