#[doc = "Register `channel` reader"]
pub struct R(crate::R<CHANNEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `channel` writer"]
pub struct W(crate::W<CHANNEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_SPEC>;
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
impl From<crate::W<CHANNEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `positive_signal[0-3]` reader - Enable or disable positive signal for channel"]
pub type POSITIVE_SIGNAL_R = crate::BitReader<bool>;
#[doc = "Field `positive_signal[0-3]` writer - Enable or disable positive signal for channel"]
pub type POSITIVE_SIGNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `positive_idle[0-3]` reader - Idle state of positive signal"]
pub type POSITIVE_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `positive_idle[0-3]` writer - Idle state of positive signal"]
pub type POSITIVE_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `negative_signal[0-3]` reader - Enable or disable negative signal for channel"]
pub type NEGATIVE_SIGNAL_R = crate::BitReader<bool>;
#[doc = "Field `negative_signal[0-3]` writer - Enable or disable negative signal for channel"]
pub type NEGATIVE_SIGNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `negative_idle[0-3]` reader - Idle state of negative signal"]
pub type NEGATIVE_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `negative_idle[0-3]` writer - Idle state of negative signal"]
pub type NEGATIVE_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `positive_polarity[0-3]` reader - Polarity of positive signal"]
pub type POSITIVE_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `positive_polarity[0-3]` writer - Polarity of positive signal"]
pub type POSITIVE_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `negative_polarity[0-3]` reader - Polarity of negative signal"]
pub type NEGATIVE_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `negative_polarity[0-3]` writer - Polarity of negative signal"]
pub type NEGATIVE_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `positive_break[0-3]` reader - Break state on positive signal of this channel"]
pub type POSITIVE_BREAK_R = crate::BitReader<bool>;
#[doc = "Field `positive_break[0-3]` writer - Break state on positive signal of this channel"]
pub type POSITIVE_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
#[doc = "Field `negative_break[0-3]` reader - Break state on negative signal of this channel"]
pub type NEGATIVE_BREAK_R = crate::BitReader<bool>;
#[doc = "Field `negative_break[0-3]` writer - Break state on negative signal of this channel"]
pub type NEGATIVE_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_SPEC, bool, O>;
impl R {
    #[doc = "Enable or disable positive signal for channel"]
    #[inline(always)]
    pub unsafe fn positive_signal(&self, n: u8) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal0(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal1(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal2(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal3(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Idle state of positive signal"]
    #[inline(always)]
    pub unsafe fn positive_idle(&self, n: u8) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle0(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle1(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle2(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle3(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Enable or disable negative signal for channel"]
    #[inline(always)]
    pub unsafe fn negative_signal(&self, n: u8) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal0(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal1(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal2(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal3(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Idle state of negative signal"]
    #[inline(always)]
    pub unsafe fn negative_idle(&self, n: u8) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Bit 3 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle0(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle1(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle2(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle3(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Polarity of positive signal"]
    #[inline(always)]
    pub unsafe fn positive_polarity(&self, n: u8) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> (n * 2 + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity0(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity1(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity2(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity3(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Polarity of negative signal"]
    #[inline(always)]
    pub unsafe fn negative_polarity(&self, n: u8) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Bit 17 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity0(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity1(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity2(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity3(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Break state on positive signal of this channel"]
    #[inline(always)]
    pub unsafe fn positive_break(&self, n: u8) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> (n * 2 + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break0(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break1(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break2(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break3(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Break state on negative signal of this channel"]
    #[inline(always)]
    pub unsafe fn negative_break(&self, n: u8) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0)
    }
    #[doc = "Bit 25 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break0(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break1(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break2(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break3(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn positive_signal<const O: u8>(&mut self) -> POSITIVE_SIGNAL_W<O> {
        POSITIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 0 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal0(&mut self) -> POSITIVE_SIGNAL_W<0> {
        POSITIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal1(&mut self) -> POSITIVE_SIGNAL_W<4> {
        POSITIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal2(&mut self) -> POSITIVE_SIGNAL_W<8> {
        POSITIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 12 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal3(&mut self) -> POSITIVE_SIGNAL_W<12> {
        POSITIVE_SIGNAL_W::new(self)
    }
    #[doc = "Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn positive_idle<const O: u8>(&mut self) -> POSITIVE_IDLE_W<O> {
        POSITIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 1 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle0(&mut self) -> POSITIVE_IDLE_W<1> {
        POSITIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 5 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle1(&mut self) -> POSITIVE_IDLE_W<5> {
        POSITIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 9 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle2(&mut self) -> POSITIVE_IDLE_W<9> {
        POSITIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 13 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle3(&mut self) -> POSITIVE_IDLE_W<13> {
        POSITIVE_IDLE_W::new(self)
    }
    #[doc = "Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn negative_signal<const O: u8>(&mut self) -> NEGATIVE_SIGNAL_W<O> {
        NEGATIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal0(&mut self) -> NEGATIVE_SIGNAL_W<2> {
        NEGATIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal1(&mut self) -> NEGATIVE_SIGNAL_W<6> {
        NEGATIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 10 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal2(&mut self) -> NEGATIVE_SIGNAL_W<10> {
        NEGATIVE_SIGNAL_W::new(self)
    }
    #[doc = "Bit 14 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal3(&mut self) -> NEGATIVE_SIGNAL_W<14> {
        NEGATIVE_SIGNAL_W::new(self)
    }
    #[doc = "Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn negative_idle<const O: u8>(&mut self) -> NEGATIVE_IDLE_W<O> {
        NEGATIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 3 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle0(&mut self) -> NEGATIVE_IDLE_W<3> {
        NEGATIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 7 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle1(&mut self) -> NEGATIVE_IDLE_W<7> {
        NEGATIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 11 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle2(&mut self) -> NEGATIVE_IDLE_W<11> {
        NEGATIVE_IDLE_W::new(self)
    }
    #[doc = "Bit 15 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle3(&mut self) -> NEGATIVE_IDLE_W<15> {
        NEGATIVE_IDLE_W::new(self)
    }
    #[doc = "Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn positive_polarity<const O: u8>(&mut self) -> POSITIVE_POLARITY_W<O> {
        POSITIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 16 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity0(&mut self) -> POSITIVE_POLARITY_W<16> {
        POSITIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 18 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity1(&mut self) -> POSITIVE_POLARITY_W<18> {
        POSITIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 20 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity2(&mut self) -> POSITIVE_POLARITY_W<20> {
        POSITIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 22 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity3(&mut self) -> POSITIVE_POLARITY_W<22> {
        POSITIVE_POLARITY_W::new(self)
    }
    #[doc = "Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn negative_polarity<const O: u8>(&mut self) -> NEGATIVE_POLARITY_W<O> {
        NEGATIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 17 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity0(&mut self) -> NEGATIVE_POLARITY_W<17> {
        NEGATIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 19 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity1(&mut self) -> NEGATIVE_POLARITY_W<19> {
        NEGATIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 21 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity2(&mut self) -> NEGATIVE_POLARITY_W<21> {
        NEGATIVE_POLARITY_W::new(self)
    }
    #[doc = "Bit 23 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity3(&mut self) -> NEGATIVE_POLARITY_W<23> {
        NEGATIVE_POLARITY_W::new(self)
    }
    #[doc = "Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn positive_break<const O: u8>(&mut self) -> POSITIVE_BREAK_W<O> {
        POSITIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 24 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break0(&mut self) -> POSITIVE_BREAK_W<24> {
        POSITIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 26 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break1(&mut self) -> POSITIVE_BREAK_W<26> {
        POSITIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 28 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break2(&mut self) -> POSITIVE_BREAK_W<28> {
        POSITIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 30 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break3(&mut self) -> POSITIVE_BREAK_W<30> {
        POSITIVE_BREAK_W::new(self)
    }
    #[doc = "Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn negative_break<const O: u8>(&mut self) -> NEGATIVE_BREAK_W<O> {
        NEGATIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 25 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break0(&mut self) -> NEGATIVE_BREAK_W<25> {
        NEGATIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 27 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break1(&mut self) -> NEGATIVE_BREAK_W<27> {
        NEGATIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 29 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break2(&mut self) -> NEGATIVE_BREAK_W<29> {
        NEGATIVE_BREAK_W::new(self)
    }
    #[doc = "Bit 31 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break3(&mut self) -> NEGATIVE_BREAK_W<31> {
        NEGATIVE_BREAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel](index.html) module"]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel::R](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel::W](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel to value 0"]
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
