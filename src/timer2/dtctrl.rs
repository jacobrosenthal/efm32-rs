#[doc = "Reader of register DTCTRL"]
pub type R = crate::R<u32, super::DTCTRL>;
#[doc = "Writer for register DTCTRL"]
pub type W = crate::W<u32, super::DTCTRL>;
#[doc = "Register DTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTEN`"]
pub type DTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTEN`"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DTDAS`"]
pub type DTDAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTDAS`"]
pub struct DTDAS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDAS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DTIPOL`"]
pub type DTIPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTIPOL`"]
pub struct DTIPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIPOL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DTCINV`"]
pub type DTCINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCINV`"]
pub struct DTCINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCINV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5,
}
impl From<DTPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRSSEL_A) -> Self {
        match variant {
            DTPRSSEL_A::PRSCH0 => 0,
            DTPRSSEL_A::PRSCH1 => 1,
            DTPRSSEL_A::PRSCH2 => 2,
            DTPRSSEL_A::PRSCH3 => 3,
            DTPRSSEL_A::PRSCH4 => 4,
            DTPRSSEL_A::PRSCH5 => 5,
        }
    }
}
#[doc = "Reader of field `DTPRSSEL`"]
pub type DTPRSSEL_R = crate::R<u8, DTPRSSEL_A>;
impl DTPRSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPRSSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPRSSEL_A::PRSCH0),
            1 => Val(DTPRSSEL_A::PRSCH1),
            2 => Val(DTPRSSEL_A::PRSCH2),
            3 => Val(DTPRSSEL_A::PRSCH3),
            4 => Val(DTPRSSEL_A::PRSCH4),
            5 => Val(DTPRSSEL_A::PRSCH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH5
    }
}
#[doc = "Write proxy for field `DTPRSSEL`"]
pub struct DTPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DTPRSEN`"]
pub type DTPRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRSEN`"]
pub struct DTPRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DTPRSSEL_R {
        DTPRSSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DTDAS_W {
        DTDAS_W { w: self }
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DTIPOL_W {
        DTIPOL_W { w: self }
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DTCINV_W {
        DTCINV_W { w: self }
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DTPRSSEL_W {
        DTPRSSEL_W { w: self }
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DTPRSEN_W {
        DTPRSEN_W { w: self }
    }
}
