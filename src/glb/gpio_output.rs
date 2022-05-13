#[doc = "Register `gpio_output[%s]` reader"]
pub struct R(crate::R<GPIO_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpio_output[%s]` writer"]
pub struct W(crate::W<GPIO_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OUTPUT_SPEC>;
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
impl From<crate::W<GPIO_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OUTPUT_SPEC>) -> Self {
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
#[doc = "Write value to Generic Purpose Input/Output pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_output](index.html) module"]
pub struct GPIO_OUTPUT_SPEC;
impl crate::RegisterSpec for GPIO_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_output::R](R) reader structure"]
impl crate::Readable for GPIO_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_output::W](W) writer structure"]
impl crate::Writable for GPIO_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpio_output[%s]
to value 0"]
impl crate::Resettable for GPIO_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
