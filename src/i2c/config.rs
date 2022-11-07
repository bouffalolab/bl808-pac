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
#[doc = "Field `master_enable` reader - Enable signal of I2C master function\n\n Asserting this bit will trigger the transaction, and should be de-asserted after finish."]
pub type MASTER_ENABLE_R = crate::BitReader<MASTER_ENABLE_A>;
#[doc = "Enable signal of I2C master function\n\n Asserting this bit will trigger the transaction, and should be de-asserted after finish.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_ENABLE_A {
    #[doc = "1: Enable I2C master function"]
    ENABLE = 1,
    #[doc = "0: Disable I2C master function"]
    DISABLE = 0,
}
impl From<MASTER_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_ENABLE_A {
        match self.bits {
            true => MASTER_ENABLE_A::ENABLE,
            false => MASTER_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MASTER_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MASTER_ENABLE_A::DISABLE
    }
}
#[doc = "Field `master_enable` writer - Enable signal of I2C master function\n\n Asserting this bit will trigger the transaction, and should be de-asserted after finish."]
pub type MASTER_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, MASTER_ENABLE_A, O>;
impl<'a, const O: u8> MASTER_ENABLE_W<'a, O> {
    #[doc = "Enable I2C master function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MASTER_ENABLE_A::ENABLE)
    }
    #[doc = "Disable I2C master function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MASTER_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `transfer_direction` reader - Packet transfer direction"]
pub type TRANSFER_DIRECTION_R = crate::BitReader<TRANSFER_DIRECTION_A>;
#[doc = "Packet transfer direction\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANSFER_DIRECTION_A {
    #[doc = "0: Write from master side"]
    WRITE = 0,
    #[doc = "1: Read from master side"]
    READ = 1,
}
impl From<TRANSFER_DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSFER_DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANSFER_DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANSFER_DIRECTION_A {
        match self.bits {
            false => TRANSFER_DIRECTION_A::WRITE,
            true => TRANSFER_DIRECTION_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == TRANSFER_DIRECTION_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == TRANSFER_DIRECTION_A::READ
    }
}
#[doc = "Field `transfer_direction` writer - Packet transfer direction"]
pub type TRANSFER_DIRECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, TRANSFER_DIRECTION_A, O>;
impl<'a, const O: u8> TRANSFER_DIRECTION_W<'a, O> {
    #[doc = "Write from master side"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(TRANSFER_DIRECTION_A::WRITE)
    }
    #[doc = "Read from master side"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(TRANSFER_DIRECTION_A::READ)
    }
}
#[doc = "Field `deglitch_enable` reader - Enable de-glitch function on all input pins"]
pub type DEGLITCH_ENABLE_R = crate::BitReader<DEGLITCH_ENABLE_A>;
#[doc = "Enable de-glitch function on all input pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEGLITCH_ENABLE_A {
    #[doc = "1: Enable de-glitch function on inputs"]
    ENABLE = 1,
    #[doc = "0: Disable de-glitch function on inputs"]
    DISABLE = 0,
}
impl From<DEGLITCH_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DEGLITCH_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEGLITCH_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEGLITCH_ENABLE_A {
        match self.bits {
            true => DEGLITCH_ENABLE_A::ENABLE,
            false => DEGLITCH_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEGLITCH_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEGLITCH_ENABLE_A::DISABLE
    }
}
#[doc = "Field `deglitch_enable` writer - Enable de-glitch function on all input pins"]
pub type DEGLITCH_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, DEGLITCH_ENABLE_A, O>;
impl<'a, const O: u8> DEGLITCH_ENABLE_W<'a, O> {
    #[doc = "Enable de-glitch function on inputs"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEGLITCH_ENABLE_A::ENABLE)
    }
    #[doc = "Disable de-glitch function on inputs"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEGLITCH_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `clock_synchronize` reader - Enable I2C clock synchronization\n\n Enable this bit to support multi-master and clock-stretching. It should not be turned-off normally."]
pub type CLOCK_SYNCHRONIZE_R = crate::BitReader<CLOCK_SYNCHRONIZE_A>;
#[doc = "Enable I2C clock synchronization\n\n Enable this bit to support multi-master and clock-stretching. It should not be turned-off normally.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_SYNCHRONIZE_A {
    #[doc = "1: Enable clock synchronization"]
    ENABLE = 1,
    #[doc = "0: Disable clock synchronization"]
    DISABLE = 0,
}
impl From<CLOCK_SYNCHRONIZE_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_SYNCHRONIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_SYNCHRONIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_SYNCHRONIZE_A {
        match self.bits {
            true => CLOCK_SYNCHRONIZE_A::ENABLE,
            false => CLOCK_SYNCHRONIZE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCK_SYNCHRONIZE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLOCK_SYNCHRONIZE_A::DISABLE
    }
}
#[doc = "Field `clock_synchronize` writer - Enable I2C clock synchronization\n\n Enable this bit to support multi-master and clock-stretching. It should not be turned-off normally."]
pub type CLOCK_SYNCHRONIZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, CLOCK_SYNCHRONIZE_A, O>;
impl<'a, const O: u8> CLOCK_SYNCHRONIZE_W<'a, O> {
    #[doc = "Enable clock synchronization"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCK_SYNCHRONIZE_A::ENABLE)
    }
    #[doc = "Disable clock synchronization"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLOCK_SYNCHRONIZE_A::DISABLE)
    }
}
#[doc = "Field `sub_address_enable` reader - Enable sub-address fields"]
pub type SUB_ADDRESS_ENABLE_R = crate::BitReader<SUB_ADDRESS_ENABLE_A>;
#[doc = "Enable sub-address fields\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB_ADDRESS_ENABLE_A {
    #[doc = "1: Enable sub-address fields"]
    ENABLE = 1,
    #[doc = "0: Disable sub-address fields"]
    DISABLE = 0,
}
impl From<SUB_ADDRESS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SUB_ADDRESS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SUB_ADDRESS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUB_ADDRESS_ENABLE_A {
        match self.bits {
            true => SUB_ADDRESS_ENABLE_A::ENABLE,
            false => SUB_ADDRESS_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SUB_ADDRESS_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SUB_ADDRESS_ENABLE_A::DISABLE
    }
}
#[doc = "Field `sub_address_enable` writer - Enable sub-address fields"]
pub type SUB_ADDRESS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, SUB_ADDRESS_ENABLE_A, O>;
impl<'a, const O: u8> SUB_ADDRESS_ENABLE_W<'a, O> {
    #[doc = "Enable sub-address fields"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_ENABLE_A::ENABLE)
    }
    #[doc = "Disable sub-address fields"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `sub_address_length` reader - Byte count for I2C sub-address"]
pub type SUB_ADDRESS_LENGTH_R = crate::FieldReader<u8, SUB_ADDRESS_LENGTH_A>;
#[doc = "Byte count for I2C sub-address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUB_ADDRESS_LENGTH_A {
    #[doc = "0: Sub-addresses include 1 byte"]
    ONE = 0,
    #[doc = "1: Sub-addresses include 2 bytes"]
    TWO = 1,
    #[doc = "2: Sub-addresses include 3 bytes"]
    THREE = 2,
    #[doc = "3: Sub-addresses include 4 bytes"]
    FOUR = 3,
}
impl From<SUB_ADDRESS_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_ADDRESS_LENGTH_A) -> Self {
        variant as _
    }
}
impl SUB_ADDRESS_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUB_ADDRESS_LENGTH_A {
        match self.bits {
            0 => SUB_ADDRESS_LENGTH_A::ONE,
            1 => SUB_ADDRESS_LENGTH_A::TWO,
            2 => SUB_ADDRESS_LENGTH_A::THREE,
            3 => SUB_ADDRESS_LENGTH_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SUB_ADDRESS_LENGTH_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SUB_ADDRESS_LENGTH_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == SUB_ADDRESS_LENGTH_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == SUB_ADDRESS_LENGTH_A::FOUR
    }
}
#[doc = "Field `sub_address_length` writer - Byte count for I2C sub-address"]
pub type SUB_ADDRESS_LENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, SUB_ADDRESS_LENGTH_A, 2, O>;
impl<'a, const O: u8> SUB_ADDRESS_LENGTH_W<'a, O> {
    #[doc = "Sub-addresses include 1 byte"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_LENGTH_A::ONE)
    }
    #[doc = "Sub-addresses include 2 bytes"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_LENGTH_A::TWO)
    }
    #[doc = "Sub-addresses include 3 bytes"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_LENGTH_A::THREE)
    }
    #[doc = "Sub-addresses include 4 bytes"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(SUB_ADDRESS_LENGTH_A::FOUR)
    }
}
#[doc = "Field `slave_address` reader - I2C transaction slave address"]
pub type SLAVE_ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `slave_address` writer - I2C transaction slave address"]
pub type SLAVE_ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 7, O>;
#[doc = "Field `packet_length` reader - Byte count for each packet"]
pub type PACKET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `packet_length` writer - Byte count for each packet"]
pub type PACKET_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `deglitch_cycle` reader - De-glitch function cycle count"]
pub type DEGLITCH_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `deglitch_cycle` writer - De-glitch function cycle count"]
pub type DEGLITCH_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable signal of I2C master function\n\n Asserting this bit will trigger the transaction, and should be de-asserted after finish."]
    #[inline(always)]
    pub fn master_enable(&self) -> MASTER_ENABLE_R {
        MASTER_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet transfer direction"]
    #[inline(always)]
    pub fn transfer_direction(&self) -> TRANSFER_DIRECTION_R {
        TRANSFER_DIRECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable de-glitch function on all input pins"]
    #[inline(always)]
    pub fn deglitch_enable(&self) -> DEGLITCH_ENABLE_R {
        DEGLITCH_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable I2C clock synchronization\n\n Enable this bit to support multi-master and clock-stretching. It should not be turned-off normally."]
    #[inline(always)]
    pub fn clock_synchronize(&self) -> CLOCK_SYNCHRONIZE_R {
        CLOCK_SYNCHRONIZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable sub-address fields"]
    #[inline(always)]
    pub fn sub_address_enable(&self) -> SUB_ADDRESS_ENABLE_R {
        SUB_ADDRESS_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Byte count for I2C sub-address"]
    #[inline(always)]
    pub fn sub_address_length(&self) -> SUB_ADDRESS_LENGTH_R {
        SUB_ADDRESS_LENGTH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:14 - I2C transaction slave address"]
    #[inline(always)]
    pub fn slave_address(&self) -> SLAVE_ADDRESS_R {
        SLAVE_ADDRESS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Byte count for each packet"]
    #[inline(always)]
    pub fn packet_length(&self) -> PACKET_LENGTH_R {
        PACKET_LENGTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - De-glitch function cycle count"]
    #[inline(always)]
    pub fn deglitch_cycle(&self) -> DEGLITCH_CYCLE_R {
        DEGLITCH_CYCLE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signal of I2C master function\n\n Asserting this bit will trigger the transaction, and should be de-asserted after finish."]
    #[inline(always)]
    #[must_use]
    pub fn master_enable(&mut self) -> MASTER_ENABLE_W<0> {
        MASTER_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Packet transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_direction(&mut self) -> TRANSFER_DIRECTION_W<1> {
        TRANSFER_DIRECTION_W::new(self)
    }
    #[doc = "Bit 2 - Enable de-glitch function on all input pins"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch_enable(&mut self) -> DEGLITCH_ENABLE_W<2> {
        DEGLITCH_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable I2C clock synchronization\n\n Enable this bit to support multi-master and clock-stretching. It should not be turned-off normally."]
    #[inline(always)]
    #[must_use]
    pub fn clock_synchronize(&mut self) -> CLOCK_SYNCHRONIZE_W<3> {
        CLOCK_SYNCHRONIZE_W::new(self)
    }
    #[doc = "Bit 4 - Enable sub-address fields"]
    #[inline(always)]
    #[must_use]
    pub fn sub_address_enable(&mut self) -> SUB_ADDRESS_ENABLE_W<4> {
        SUB_ADDRESS_ENABLE_W::new(self)
    }
    #[doc = "Bits 5:6 - Byte count for I2C sub-address"]
    #[inline(always)]
    #[must_use]
    pub fn sub_address_length(&mut self) -> SUB_ADDRESS_LENGTH_W<5> {
        SUB_ADDRESS_LENGTH_W::new(self)
    }
    #[doc = "Bits 8:14 - I2C transaction slave address"]
    #[inline(always)]
    #[must_use]
    pub fn slave_address(&mut self) -> SLAVE_ADDRESS_W<8> {
        SLAVE_ADDRESS_W::new(self)
    }
    #[doc = "Bits 16:23 - Byte count for each packet"]
    #[inline(always)]
    #[must_use]
    pub fn packet_length(&mut self) -> PACKET_LENGTH_W<16> {
        PACKET_LENGTH_W::new(self)
    }
    #[doc = "Bits 28:31 - De-glitch function cycle count"]
    #[inline(always)]
    #[must_use]
    pub fn deglitch_cycle(&mut self) -> DEGLITCH_CYCLE_W<28> {
        DEGLITCH_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
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
#[doc = "`reset()` method sets config to value 0x0a"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
