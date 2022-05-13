#[doc = "Register `dma_config_2` reader"]
pub struct R(crate::R<DMA_CONFIG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONFIG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONFIG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONFIG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dma_config_2` writer"]
pub struct W(crate::W<DMA_CONFIG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONFIG_2_SPEC>;
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
impl From<crate::W<DMA_CONFIG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONFIG_2_SPEC>) -> Self {
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
#[doc = "Direct memory access configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_config_2](index.html) module"]
pub struct DMA_CONFIG_2_SPEC;
impl crate::RegisterSpec for DMA_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_config_2::R](R) reader structure"]
impl crate::Readable for DMA_CONFIG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_config_2::W](W) writer structure"]
impl crate::Writable for DMA_CONFIG_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dma_config_2 to value 0"]
impl crate::Resettable for DMA_CONFIG_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
