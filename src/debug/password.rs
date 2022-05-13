#[doc = "Register `password[%s]` reader"]
pub struct R(crate::R<PASSWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASSWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASSWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASSWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `password[%s]` writer"]
pub struct W(crate::W<PASSWORD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASSWORD_SPEC>;
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
impl From<crate::W<PASSWORD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASSWORD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `word` reader - Read or write password in word"]
pub type WORD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `word` writer - Read or write password in word"]
pub type WORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PASSWORD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Read or write password in word"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read or write password in word"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W<0> {
        WORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Password of debug module\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [password](index.html) module"]
pub struct PASSWORD_SPEC;
impl crate::RegisterSpec for PASSWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [password::R](R) reader structure"]
impl crate::Readable for PASSWORD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [password::W](W) writer structure"]
impl crate::Writable for PASSWORD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets password[%s]
to value 0"]
impl crate::Resettable for PASSWORD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
