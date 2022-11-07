#[doc = "Register `sub_address` reader"]
pub struct R(crate::R<SUB_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUB_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUB_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUB_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sub_address` writer"]
pub struct W(crate::W<SUB_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB_ADDRESS_SPEC>;
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
impl From<crate::W<SUB_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `byte[0-3]` reader - I2C sub-address byte %s"]
pub type BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte[0-3]` writer - I2C sub-address byte %s"]
pub type BYTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB_ADDRESS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "I2C sub-address byte [0-3]"]
    #[inline(always)]
    pub unsafe fn byte(&self, n: u8) -> BYTE_R {
        BYTE_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - I2C sub-address byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE_R {
        BYTE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C sub-address byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C sub-address byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I2C sub-address byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "I2C sub-address byte [0-3]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn byte<const O: u8>(&mut self) -> BYTE_W<O> {
        BYTE_W::new(self)
    }
    #[doc = "Bits 0:7 - I2C sub-address byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn byte0(&mut self) -> BYTE_W<0> {
        BYTE_W::new(self)
    }
    #[doc = "Bits 8:15 - I2C sub-address byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn byte1(&mut self) -> BYTE_W<8> {
        BYTE_W::new(self)
    }
    #[doc = "Bits 16:23 - I2C sub-address byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn byte2(&mut self) -> BYTE_W<16> {
        BYTE_W::new(self)
    }
    #[doc = "Bits 24:31 - I2C sub-address byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn byte3(&mut self) -> BYTE_W<24> {
        BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register address of slave device\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub_address](index.html) module"]
pub struct SUB_ADDRESS_SPEC;
impl crate::RegisterSpec for SUB_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sub_address::R](R) reader structure"]
impl crate::Readable for SUB_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sub_address::W](W) writer structure"]
impl crate::Writable for SUB_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sub_address to value 0"]
impl crate::Resettable for SUB_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
