#[doc = "Reader of register DTFC"]
pub type R = crate::R<u32, super::DTFC>;
#[doc = "Writer for register DTFC"]
pub type W = crate::W<u32, super::DTFC>;
#[doc = "Register DTFC `reset()`'s with value 0"]
impl crate::ResetValue for super::DTFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DTI PRS Fault Source 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS0FSEL_A {
    #[doc = "0: PRS Channel 0 selected as fault source 0"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected as fault source 0"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected as fault source 0"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected as fault source 0"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected as fault source 0"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected as fault source 0"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected as fault source 0"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected as fault source 0"]
    PRSCH7,
}
impl From<DTPRS0FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS0FSEL_A) -> Self {
        match variant {
            DTPRS0FSEL_A::PRSCH0 => 0,
            DTPRS0FSEL_A::PRSCH1 => 1,
            DTPRS0FSEL_A::PRSCH2 => 2,
            DTPRS0FSEL_A::PRSCH3 => 3,
            DTPRS0FSEL_A::PRSCH4 => 4,
            DTPRS0FSEL_A::PRSCH5 => 5,
            DTPRS0FSEL_A::PRSCH6 => 6,
            DTPRS0FSEL_A::PRSCH7 => 7,
        }
    }
}
#[doc = "Reader of field `DTPRS0FSEL`"]
pub type DTPRS0FSEL_R = crate::R<u8, DTPRS0FSEL_A>;
impl DTPRS0FSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTPRS0FSEL_A {
        match self.bits {
            0 => DTPRS0FSEL_A::PRSCH0,
            1 => DTPRS0FSEL_A::PRSCH1,
            2 => DTPRS0FSEL_A::PRSCH2,
            3 => DTPRS0FSEL_A::PRSCH3,
            4 => DTPRS0FSEL_A::PRSCH4,
            5 => DTPRS0FSEL_A::PRSCH5,
            6 => DTPRS0FSEL_A::PRSCH6,
            7 => DTPRS0FSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH7
    }
}
#[doc = "Write proxy for field `DTPRS0FSEL`"]
pub struct DTPRS0FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRS0FSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "DTI PRS Fault Source 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS1FSEL_A {
    #[doc = "0: PRS Channel 0 selected as fault source 1"]
    PRSCH0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "2: PRS Channel 2 selected as fault source 1"]
    PRSCH2,
    #[doc = "3: PRS Channel 3 selected as fault source 1"]
    PRSCH3,
    #[doc = "4: PRS Channel 4 selected as fault source 1"]
    PRSCH4,
    #[doc = "5: PRS Channel 5 selected as fault source 1"]
    PRSCH5,
    #[doc = "6: PRS Channel 6 selected as fault source 1"]
    PRSCH6,
    #[doc = "7: PRS Channel 7 selected as fault source 1"]
    PRSCH7,
}
impl From<DTPRS1FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS1FSEL_A) -> Self {
        match variant {
            DTPRS1FSEL_A::PRSCH0 => 0,
            DTPRS1FSEL_A::PRSCH1 => 1,
            DTPRS1FSEL_A::PRSCH2 => 2,
            DTPRS1FSEL_A::PRSCH3 => 3,
            DTPRS1FSEL_A::PRSCH4 => 4,
            DTPRS1FSEL_A::PRSCH5 => 5,
            DTPRS1FSEL_A::PRSCH6 => 6,
            DTPRS1FSEL_A::PRSCH7 => 7,
        }
    }
}
#[doc = "Reader of field `DTPRS1FSEL`"]
pub type DTPRS1FSEL_R = crate::R<u8, DTPRS1FSEL_A>;
impl DTPRS1FSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTPRS1FSEL_A {
        match self.bits {
            0 => DTPRS1FSEL_A::PRSCH0,
            1 => DTPRS1FSEL_A::PRSCH1,
            2 => DTPRS1FSEL_A::PRSCH2,
            3 => DTPRS1FSEL_A::PRSCH3,
            4 => DTPRS1FSEL_A::PRSCH4,
            5 => DTPRS1FSEL_A::PRSCH5,
            6 => DTPRS1FSEL_A::PRSCH6,
            7 => DTPRS1FSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH7
    }
}
#[doc = "Write proxy for field `DTPRS1FSEL`"]
pub struct DTPRS1FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRS1FSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFA_A {
    #[doc = "0: No action on fault"]
    NONE,
    #[doc = "1: Set outputs inactive"]
    INACTIVE,
    #[doc = "2: Clear outputs"]
    CLEAR,
    #[doc = "3: Tristate outputs"]
    TRISTATE,
}
impl From<DTFA_A> for u8 {
    #[inline(always)]
    fn from(variant: DTFA_A) -> Self {
        match variant {
            DTFA_A::NONE => 0,
            DTFA_A::INACTIVE => 1,
            DTFA_A::CLEAR => 2,
            DTFA_A::TRISTATE => 3,
        }
    }
}
#[doc = "Reader of field `DTFA`"]
pub type DTFA_R = crate::R<u8, DTFA_A>;
impl DTFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFA_A {
        match self.bits {
            0 => DTFA_A::NONE,
            1 => DTFA_A::INACTIVE,
            2 => DTFA_A::CLEAR,
            3 => DTFA_A::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA_A::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA_A::TRISTATE
    }
}
#[doc = "Write proxy for field `DTFA`"]
pub struct DTFA_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFA_A::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFA_A::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFA_A::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFA_A::TRISTATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DTPRS0FEN`"]
pub type DTPRS0FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRS0FEN`"]
pub struct DTPRS0FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FEN_W<'a> {
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
#[doc = "Reader of field `DTPRS1FEN`"]
pub type DTPRS1FEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTPRS1FEN`"]
pub struct DTPRS1FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DTDBGFEN`"]
pub type DTDBGFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTDBGFEN`"]
pub struct DTDBGFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDBGFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DTLOCKUPFEN`"]
pub type DTLOCKUPFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTLOCKUPFEN`"]
pub struct DTLOCKUPFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLOCKUPFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> DTPRS0FSEL_R {
        DTPRS0FSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> DTPRS1FSEL_R {
        DTPRS1FSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DTFA_R {
        DTFA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> DTPRS0FEN_R {
        DTPRS0FEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> DTPRS1FEN_R {
        DTPRS1FEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DTDBGFEN_R {
        DTDBGFEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DTLOCKUPFEN_R {
        DTLOCKUPFEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&mut self) -> DTPRS0FSEL_W {
        DTPRS0FSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&mut self) -> DTPRS1FSEL_W {
        DTPRS1FSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&mut self) -> DTFA_W {
        DTFA_W { w: self }
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&mut self) -> DTPRS0FEN_W {
        DTPRS0FEN_W { w: self }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&mut self) -> DTPRS1FEN_W {
        DTPRS1FEN_W { w: self }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&mut self) -> DTDBGFEN_W {
        DTDBGFEN_W { w: self }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&mut self) -> DTLOCKUPFEN_W {
        DTLOCKUPFEN_W { w: self }
    }
}
