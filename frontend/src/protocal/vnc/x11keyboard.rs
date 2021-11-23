#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use yew::services::ConsoleService;

// referring:
//    https://github.com/AltF02/x11-rs/blob/master/src/keysym.rs

pub const XK_BackSpace: u32 = 0xFF08;
pub const XK_Tab: u32 = 0xFF09;
pub const XK_Linefeed: u32 = 0xFF0A;
pub const XK_Clear: u32 = 0xFF0B;
pub const XK_Return: u32 = 0xFF0D;
pub const XK_Pause: u32 = 0xFF13;
pub const XK_Scroll_Lock: u32 = 0xFF14;
pub const XK_Sys_Req: u32 = 0xFF15;
pub const XK_Escape: u32 = 0xFF1B;
pub const XK_Delete: u32 = 0xFFFF;
pub const XK_Multi_key: u32 = 0xFF20;
pub const XK_Kanji: u32 = 0xFF21;
pub const XK_Muhenkan: u32 = 0xFF22;
pub const XK_Henkan_Mode: u32 = 0xFF23;
pub const XK_Henkan: u32 = 0xFF23;
pub const XK_Romaji: u32 = 0xFF24;
pub const XK_Hiragana: u32 = 0xFF25;
pub const XK_Katakana: u32 = 0xFF26;
pub const XK_Hiragana_Katakana: u32 = 0xFF27;
pub const XK_Zenkaku: u32 = 0xFF28;
pub const XK_Hankaku: u32 = 0xFF29;
pub const XK_Zenkaku_Hankaku: u32 = 0xFF2A;
pub const XK_Touroku: u32 = 0xFF2B;
pub const XK_Massyo: u32 = 0xFF2C;
pub const XK_Kana_Lock: u32 = 0xFF2D;
pub const XK_Kana_Shift: u32 = 0xFF2E;
pub const XK_Eisu_Shift: u32 = 0xFF2F;
pub const XK_Eisu_toggle: u32 = 0xFF30;
pub const XK_Home: u32 = 0xFF50;
pub const XK_Left: u32 = 0xFF51;
pub const XK_Up: u32 = 0xFF52;
pub const XK_Right: u32 = 0xFF53;
pub const XK_Down: u32 = 0xFF54;
pub const XK_Prior: u32 = 0xFF55;
pub const XK_Page_Up: u32 = 0xFF55;
pub const XK_Next: u32 = 0xFF56;
pub const XK_Page_Down: u32 = 0xFF56;
pub const XK_End: u32 = 0xFF57;
pub const XK_Begin: u32 = 0xFF58;
pub const XK_Win_L: u32 = 0xFF5B;
pub const XK_Win_R: u32 = 0xFF5C;
pub const XK_App: u32 = 0xFF5D;
pub const XK_Select: u32 = 0xFF60;
pub const XK_Print: u32 = 0xFF61;
pub const XK_Execute: u32 = 0xFF62;
pub const XK_Insert: u32 = 0xFF63;
pub const XK_Undo: u32 = 0xFF65;
pub const XK_Redo: u32 = 0xFF66;
pub const XK_Menu: u32 = 0xFF67;
pub const XK_Find: u32 = 0xFF68;
pub const XK_Cancel: u32 = 0xFF69;
pub const XK_Help: u32 = 0xFF6A;
pub const XK_Break: u32 = 0xFF6B;
pub const XK_Mode_switch: u32 = 0xFF7E;
pub const XK_script_switch: u32 = 0xFF7E;
pub const XK_Num_Lock: u32 = 0xFF7F;
pub const XK_KP_Space: u32 = 0xFF80;
pub const XK_KP_Tab: u32 = 0xFF89;
pub const XK_KP_Enter: u32 = 0xFF8D;
pub const XK_KP_F1: u32 = 0xFF91;
pub const XK_KP_F2: u32 = 0xFF92;
pub const XK_KP_F3: u32 = 0xFF93;
pub const XK_KP_F4: u32 = 0xFF94;
pub const XK_KP_Home: u32 = 0xFF95;
pub const XK_KP_Left: u32 = 0xFF96;
pub const XK_KP_Up: u32 = 0xFF97;
pub const XK_KP_Right: u32 = 0xFF98;
pub const XK_KP_Down: u32 = 0xFF99;
pub const XK_KP_Prior: u32 = 0xFF9A;
pub const XK_KP_Page_Up: u32 = 0xFF9A;
pub const XK_KP_Next: u32 = 0xFF9B;
pub const XK_KP_Page_Down: u32 = 0xFF9B;
pub const XK_KP_End: u32 = 0xFF9C;
pub const XK_KP_Begin: u32 = 0xFF9D;
pub const XK_KP_Insert: u32 = 0xFF9E;
pub const XK_KP_Delete: u32 = 0xFF9F;
pub const XK_KP_Equal: u32 = 0xFFBD;
pub const XK_KP_Multiply: u32 = 0xFFAA;
pub const XK_KP_Add: u32 = 0xFFAB;
pub const XK_KP_Separator: u32 = 0xFFAC;
pub const XK_KP_Subtract: u32 = 0xFFAD;
pub const XK_KP_Decimal: u32 = 0xFFAE;
pub const XK_KP_Divide: u32 = 0xFFAF;
pub const XK_KP_0: u32 = 0xFFB0;
pub const XK_KP_1: u32 = 0xFFB1;
pub const XK_KP_2: u32 = 0xFFB2;
pub const XK_KP_3: u32 = 0xFFB3;
pub const XK_KP_4: u32 = 0xFFB4;
pub const XK_KP_5: u32 = 0xFFB5;
pub const XK_KP_6: u32 = 0xFFB6;
pub const XK_KP_7: u32 = 0xFFB7;
pub const XK_KP_8: u32 = 0xFFB8;
pub const XK_KP_9: u32 = 0xFFB9;
pub const XK_F1: u32 = 0xFFBE;
pub const XK_F2: u32 = 0xFFBF;
pub const XK_F3: u32 = 0xFFC0;
pub const XK_F4: u32 = 0xFFC1;
pub const XK_F5: u32 = 0xFFC2;
pub const XK_F6: u32 = 0xFFC3;
pub const XK_F7: u32 = 0xFFC4;
pub const XK_F8: u32 = 0xFFC5;
pub const XK_F9: u32 = 0xFFC6;
pub const XK_F10: u32 = 0xFFC7;
pub const XK_F11: u32 = 0xFFC8;
pub const XK_L1: u32 = 0xFFC8;
pub const XK_F12: u32 = 0xFFC9;
pub const XK_L2: u32 = 0xFFC9;
pub const XK_F13: u32 = 0xFFCA;
pub const XK_L3: u32 = 0xFFCA;
pub const XK_F14: u32 = 0xFFCB;
pub const XK_L4: u32 = 0xFFCB;
pub const XK_F15: u32 = 0xFFCC;
pub const XK_L5: u32 = 0xFFCC;
pub const XK_F16: u32 = 0xFFCD;
pub const XK_L6: u32 = 0xFFCD;
pub const XK_F17: u32 = 0xFFCE;
pub const XK_L7: u32 = 0xFFCE;
pub const XK_F18: u32 = 0xFFCF;
pub const XK_L8: u32 = 0xFFCF;
pub const XK_F19: u32 = 0xFFD0;
pub const XK_L9: u32 = 0xFFD0;
pub const XK_F20: u32 = 0xFFD1;
pub const XK_L10: u32 = 0xFFD1;
pub const XK_F21: u32 = 0xFFD2;
pub const XK_R1: u32 = 0xFFD2;
pub const XK_F22: u32 = 0xFFD3;
pub const XK_R2: u32 = 0xFFD3;
pub const XK_F23: u32 = 0xFFD4;
pub const XK_R3: u32 = 0xFFD4;
pub const XK_F24: u32 = 0xFFD5;
pub const XK_R4: u32 = 0xFFD5;
pub const XK_F25: u32 = 0xFFD6;
pub const XK_R5: u32 = 0xFFD6;
pub const XK_F26: u32 = 0xFFD7;
pub const XK_R6: u32 = 0xFFD7;
pub const XK_F27: u32 = 0xFFD8;
pub const XK_R7: u32 = 0xFFD8;
pub const XK_F28: u32 = 0xFFD9;
pub const XK_R8: u32 = 0xFFD9;
pub const XK_F29: u32 = 0xFFDA;
pub const XK_R9: u32 = 0xFFDA;
pub const XK_F30: u32 = 0xFFDB;
pub const XK_R10: u32 = 0xFFDB;
pub const XK_F31: u32 = 0xFFDC;
pub const XK_R11: u32 = 0xFFDC;
pub const XK_F32: u32 = 0xFFDD;
pub const XK_R12: u32 = 0xFFDD;
pub const XK_F33: u32 = 0xFFDE;
pub const XK_R13: u32 = 0xFFDE;
pub const XK_F34: u32 = 0xFFDF;
pub const XK_R14: u32 = 0xFFDF;
pub const XK_F35: u32 = 0xFFE0;
pub const XK_R15: u32 = 0xFFE0;
pub const XK_Shift_L: u32 = 0xFFE1;
pub const XK_Shift_R: u32 = 0xFFE2;
pub const XK_Control_L: u32 = 0xFFE3;
pub const XK_Control_R: u32 = 0xFFE4;
pub const XK_Caps_Lock: u32 = 0xFFE5;
pub const XK_Shift_Lock: u32 = 0xFFE6;
pub const XK_Meta_L: u32 = 0xFFE7;
pub const XK_Meta_R: u32 = 0xFFE8;
pub const XK_Alt_L: u32 = 0xFFE9;
pub const XK_Alt_R: u32 = 0xFFEA;
pub const XK_Super_L: u32 = 0xFFEB;
pub const XK_Super_R: u32 = 0xFFEC;
pub const XK_Hyper_L: u32 = 0xFFED;
pub const XK_Hyper_R: u32 = 0xFFEE;
pub const XK_space: u32 = 0x020;
pub const XK_exclam: u32 = 0x021;
pub const XK_quotedbl: u32 = 0x022;
pub const XK_numbersign: u32 = 0x023;
pub const XK_dollar: u32 = 0x024;
pub const XK_percent: u32 = 0x025;
pub const XK_ampersand: u32 = 0x026;
pub const XK_apostrophe: u32 = 0x027;
pub const XK_quoteright: u32 = 0x027;
pub const XK_parenleft: u32 = 0x028;
pub const XK_parenright: u32 = 0x029;
pub const XK_asterisk: u32 = 0x02a;
pub const XK_plus: u32 = 0x02b;
pub const XK_comma: u32 = 0x02c;
pub const XK_minus: u32 = 0x02d;
pub const XK_period: u32 = 0x02e;
pub const XK_slash: u32 = 0x02f;
pub const XK_0: u32 = 0x030;
pub const XK_1: u32 = 0x031;
pub const XK_2: u32 = 0x032;
pub const XK_3: u32 = 0x033;
pub const XK_4: u32 = 0x034;
pub const XK_5: u32 = 0x035;
pub const XK_6: u32 = 0x036;
pub const XK_7: u32 = 0x037;
pub const XK_8: u32 = 0x038;
pub const XK_9: u32 = 0x039;
pub const XK_colon: u32 = 0x03a;
pub const XK_semicolon: u32 = 0x03b;
pub const XK_less: u32 = 0x03c;
pub const XK_equal: u32 = 0x03d;
pub const XK_greater: u32 = 0x03e;
pub const XK_question: u32 = 0x03f;
pub const XK_at: u32 = 0x040;
pub const XK_A: u32 = 0x041;
pub const XK_B: u32 = 0x042;
pub const XK_C: u32 = 0x043;
pub const XK_D: u32 = 0x044;
pub const XK_E: u32 = 0x045;
pub const XK_F: u32 = 0x046;
pub const XK_G: u32 = 0x047;
pub const XK_H: u32 = 0x048;
pub const XK_I: u32 = 0x049;
pub const XK_J: u32 = 0x04a;
pub const XK_K: u32 = 0x04b;
pub const XK_L: u32 = 0x04c;
pub const XK_M: u32 = 0x04d;
pub const XK_N: u32 = 0x04e;
pub const XK_O: u32 = 0x04f;
pub const XK_P: u32 = 0x050;
pub const XK_Q: u32 = 0x051;
pub const XK_R: u32 = 0x052;
pub const XK_S: u32 = 0x053;
pub const XK_T: u32 = 0x054;
pub const XK_U: u32 = 0x055;
pub const XK_V: u32 = 0x056;
pub const XK_W: u32 = 0x057;
pub const XK_X: u32 = 0x058;
pub const XK_Y: u32 = 0x059;
pub const XK_Z: u32 = 0x05a;
pub const XK_bracketleft: u32 = 0x05b;
pub const XK_backslash: u32 = 0x05c;
pub const XK_bracketright: u32 = 0x05d;
pub const XK_asciicircum: u32 = 0x05e;
pub const XK_underscore: u32 = 0x05f;
pub const XK_grave: u32 = 0x060;
pub const XK_quoteleft: u32 = 0x060;
pub const XK_a: u32 = 0x061;
pub const XK_b: u32 = 0x062;
pub const XK_c: u32 = 0x063;
pub const XK_d: u32 = 0x064;
pub const XK_e: u32 = 0x065;
pub const XK_f: u32 = 0x066;
pub const XK_g: u32 = 0x067;
pub const XK_h: u32 = 0x068;
pub const XK_i: u32 = 0x069;
pub const XK_j: u32 = 0x06a;
pub const XK_k: u32 = 0x06b;
pub const XK_l: u32 = 0x06c;
pub const XK_m: u32 = 0x06d;
pub const XK_n: u32 = 0x06e;
pub const XK_o: u32 = 0x06f;
pub const XK_p: u32 = 0x070;
pub const XK_q: u32 = 0x071;
pub const XK_r: u32 = 0x072;
pub const XK_s: u32 = 0x073;
pub const XK_t: u32 = 0x074;
pub const XK_u: u32 = 0x075;
pub const XK_v: u32 = 0x076;
pub const XK_w: u32 = 0x077;
pub const XK_x: u32 = 0x078;
pub const XK_y: u32 = 0x079;
pub const XK_z: u32 = 0x07a;
pub const XK_braceleft: u32 = 0x07b;
pub const XK_bar: u32 = 0x07c;
pub const XK_braceright: u32 = 0x07d;
pub const XK_asciitilde: u32 = 0x07e;
pub const XK_nobreakspace: u32 = 0x0a0;
pub const XK_exclamdown: u32 = 0x0a1;
pub const XK_cent: u32 = 0x0a2;
pub const XK_sterling: u32 = 0x0a3;
pub const XK_currency: u32 = 0x0a4;
pub const XK_yen: u32 = 0x0a5;
pub const XK_brokenbar: u32 = 0x0a6;
pub const XK_section: u32 = 0x0a7;
pub const XK_diaeresis: u32 = 0x0a8;
pub const XK_copyright: u32 = 0x0a9;
pub const XK_ordfeminine: u32 = 0x0aa;
pub const XK_guillemotleft: u32 = 0x0ab;
pub const XK_notsign: u32 = 0x0ac;
pub const XK_hyphen: u32 = 0x0ad;
pub const XK_registered: u32 = 0x0ae;
pub const XK_macron: u32 = 0x0af;
pub const XK_degree: u32 = 0x0b0;
pub const XK_plusminus: u32 = 0x0b1;
pub const XK_twosuperior: u32 = 0x0b2;
pub const XK_threesuperior: u32 = 0x0b3;
pub const XK_acute: u32 = 0x0b4;
pub const XK_mu: u32 = 0x0b5;
pub const XK_paragraph: u32 = 0x0b6;
pub const XK_periodcentered: u32 = 0x0b7;
pub const XK_cedilla: u32 = 0x0b8;
pub const XK_onesuperior: u32 = 0x0b9;
pub const XK_masculine: u32 = 0x0ba;
pub const XK_guillemotright: u32 = 0x0bb;
pub const XK_onequarter: u32 = 0x0bc;
pub const XK_onehalf: u32 = 0x0bd;
pub const XK_threequarters: u32 = 0x0be;
pub const XK_questiondown: u32 = 0x0bf;
pub const XK_Agrave: u32 = 0x0c0;
pub const XK_Aacute: u32 = 0x0c1;
pub const XK_Acircumflex: u32 = 0x0c2;
pub const XK_Atilde: u32 = 0x0c3;
pub const XK_Adiaeresis: u32 = 0x0c4;
pub const XK_Aring: u32 = 0x0c5;
pub const XK_AE: u32 = 0x0c6;
pub const XK_Ccedilla: u32 = 0x0c7;
pub const XK_Egrave: u32 = 0x0c8;
pub const XK_Eacute: u32 = 0x0c9;
pub const XK_Ecircumflex: u32 = 0x0ca;
pub const XK_Ediaeresis: u32 = 0x0cb;
pub const XK_Igrave: u32 = 0x0cc;
pub const XK_Iacute: u32 = 0x0cd;
pub const XK_Icircumflex: u32 = 0x0ce;
pub const XK_Idiaeresis: u32 = 0x0cf;
pub const XK_ETH: u32 = 0x0d0;
pub const XK_Eth: u32 = 0x0d0;
pub const XK_Ntilde: u32 = 0x0d1;
pub const XK_Ograve: u32 = 0x0d2;
pub const XK_Oacute: u32 = 0x0d3;
pub const XK_Ocircumflex: u32 = 0x0d4;
pub const XK_Otilde: u32 = 0x0d5;
pub const XK_Odiaeresis: u32 = 0x0d6;
pub const XK_multiply: u32 = 0x0d7;
pub const XK_Ooblique: u32 = 0x0d8;
pub const XK_Ugrave: u32 = 0x0d9;
pub const XK_Uacute: u32 = 0x0da;
pub const XK_Ucircumflex: u32 = 0x0db;
pub const XK_Udiaeresis: u32 = 0x0dc;
pub const XK_Yacute: u32 = 0x0dd;
pub const XK_THORN: u32 = 0x0de;
pub const XK_Thorn: u32 = 0x0de;
pub const XK_ssharp: u32 = 0x0df;
pub const XK_agrave: u32 = 0x0e0;
pub const XK_aacute: u32 = 0x0e1;
pub const XK_acircumflex: u32 = 0x0e2;
pub const XK_atilde: u32 = 0x0e3;
pub const XK_adiaeresis: u32 = 0x0e4;
pub const XK_aring: u32 = 0x0e5;
pub const XK_ae: u32 = 0x0e6;
pub const XK_ccedilla: u32 = 0x0e7;
pub const XK_egrave: u32 = 0x0e8;
pub const XK_eacute: u32 = 0x0e9;
pub const XK_ecircumflex: u32 = 0x0ea;
pub const XK_ediaeresis: u32 = 0x0eb;
pub const XK_igrave: u32 = 0x0ec;
pub const XK_iacute: u32 = 0x0ed;
pub const XK_icircumflex: u32 = 0x0ee;
pub const XK_idiaeresis: u32 = 0x0ef;
pub const XK_eth: u32 = 0x0f0;
pub const XK_ntilde: u32 = 0x0f1;
pub const XK_ograve: u32 = 0x0f2;
pub const XK_oacute: u32 = 0x0f3;
pub const XK_ocircumflex: u32 = 0x0f4;
pub const XK_otilde: u32 = 0x0f5;
pub const XK_odiaeresis: u32 = 0x0f6;
pub const XK_division: u32 = 0x0f7;
pub const XK_oslash: u32 = 0x0f8;
pub const XK_ugrave: u32 = 0x0f9;
pub const XK_uacute: u32 = 0x0fa;
pub const XK_ucircumflex: u32 = 0x0fb;
pub const XK_udiaeresis: u32 = 0x0fc;
pub const XK_yacute: u32 = 0x0fd;
pub const XK_thorn: u32 = 0x0fe;
pub const XK_ydiaeresis: u32 = 0x0ff;
pub const XK_Aogonek: u32 = 0x1a1;
pub const XK_breve: u32 = 0x1a2;
pub const XK_Lstroke: u32 = 0x1a3;
pub const XK_Lcaron: u32 = 0x1a5;
pub const XK_Sacute: u32 = 0x1a6;
pub const XK_Scaron: u32 = 0x1a9;
pub const XK_Scedilla: u32 = 0x1aa;
pub const XK_Tcaron: u32 = 0x1ab;
pub const XK_Zacute: u32 = 0x1ac;
pub const XK_Zcaron: u32 = 0x1ae;
pub const XK_Zabovedot: u32 = 0x1af;
pub const XK_aogonek: u32 = 0x1b1;
pub const XK_ogonek: u32 = 0x1b2;
pub const XK_lstroke: u32 = 0x1b3;
pub const XK_lcaron: u32 = 0x1b5;
pub const XK_sacute: u32 = 0x1b6;
pub const XK_caron: u32 = 0x1b7;
pub const XK_scaron: u32 = 0x1b9;
pub const XK_scedilla: u32 = 0x1ba;
pub const XK_tcaron: u32 = 0x1bb;
pub const XK_zacute: u32 = 0x1bc;
pub const XK_doubleacute: u32 = 0x1bd;
pub const XK_zcaron: u32 = 0x1be;
pub const XK_zabovedot: u32 = 0x1bf;
pub const XK_Racute: u32 = 0x1c0;
pub const XK_Abreve: u32 = 0x1c3;
pub const XK_Lacute: u32 = 0x1c5;
pub const XK_Cacute: u32 = 0x1c6;
pub const XK_Ccaron: u32 = 0x1c8;
pub const XK_Eogonek: u32 = 0x1ca;
pub const XK_Ecaron: u32 = 0x1cc;
pub const XK_Dcaron: u32 = 0x1cf;
pub const XK_Dstroke: u32 = 0x1d0;
pub const XK_Nacute: u32 = 0x1d1;
pub const XK_Ncaron: u32 = 0x1d2;
pub const XK_Odoubleacute: u32 = 0x1d5;
pub const XK_Rcaron: u32 = 0x1d8;
pub const XK_Uring: u32 = 0x1d9;
pub const XK_Udoubleacute: u32 = 0x1db;
pub const XK_Tcedilla: u32 = 0x1de;
pub const XK_racute: u32 = 0x1e0;
pub const XK_abreve: u32 = 0x1e3;
pub const XK_lacute: u32 = 0x1e5;
pub const XK_cacute: u32 = 0x1e6;
pub const XK_ccaron: u32 = 0x1e8;
pub const XK_eogonek: u32 = 0x1ea;
pub const XK_ecaron: u32 = 0x1ec;
pub const XK_dcaron: u32 = 0x1ef;
pub const XK_dstroke: u32 = 0x1f0;
pub const XK_nacute: u32 = 0x1f1;
pub const XK_ncaron: u32 = 0x1f2;
pub const XK_odoubleacute: u32 = 0x1f5;
pub const XK_udoubleacute: u32 = 0x1fb;
pub const XK_rcaron: u32 = 0x1f8;
pub const XK_uring: u32 = 0x1f9;
pub const XK_tcedilla: u32 = 0x1fe;
pub const XK_abovedot: u32 = 0x1ff;
pub const XK_Hstroke: u32 = 0x2a1;
pub const XK_Hcircumflex: u32 = 0x2a6;
pub const XK_Iabovedot: u32 = 0x2a9;
pub const XK_Gbreve: u32 = 0x2ab;
pub const XK_Jcircumflex: u32 = 0x2ac;
pub const XK_hstroke: u32 = 0x2b1;
pub const XK_hcircumflex: u32 = 0x2b6;
pub const XK_idotless: u32 = 0x2b9;
pub const XK_gbreve: u32 = 0x2bb;
pub const XK_jcircumflex: u32 = 0x2bc;
pub const XK_Cabovedot: u32 = 0x2c5;
pub const XK_Ccircumflex: u32 = 0x2c6;
pub const XK_Gabovedot: u32 = 0x2d5;
pub const XK_Gcircumflex: u32 = 0x2d8;
pub const XK_Ubreve: u32 = 0x2dd;
pub const XK_Scircumflex: u32 = 0x2de;
pub const XK_cabovedot: u32 = 0x2e5;
pub const XK_ccircumflex: u32 = 0x2e6;
pub const XK_gabovedot: u32 = 0x2f5;
pub const XK_gcircumflex: u32 = 0x2f8;
pub const XK_ubreve: u32 = 0x2fd;
pub const XK_scircumflex: u32 = 0x2fe;
pub const XK_kra: u32 = 0x3a2;
pub const XK_kappa: u32 = 0x3a2;
pub const XK_Rcedilla: u32 = 0x3a3;
pub const XK_Itilde: u32 = 0x3a5;
pub const XK_Lcedilla: u32 = 0x3a6;
pub const XK_Emacron: u32 = 0x3aa;
pub const XK_Gcedilla: u32 = 0x3ab;
pub const XK_Tslash: u32 = 0x3ac;
pub const XK_rcedilla: u32 = 0x3b3;
pub const XK_itilde: u32 = 0x3b5;
pub const XK_lcedilla: u32 = 0x3b6;
pub const XK_emacron: u32 = 0x3ba;
pub const XK_gcedilla: u32 = 0x3bb;
pub const XK_tslash: u32 = 0x3bc;
pub const XK_ENG: u32 = 0x3bd;
pub const XK_eng: u32 = 0x3bf;
pub const XK_Amacron: u32 = 0x3c0;
pub const XK_Iogonek: u32 = 0x3c7;
pub const XK_Eabovedot: u32 = 0x3cc;
pub const XK_Imacron: u32 = 0x3cf;
pub const XK_Ncedilla: u32 = 0x3d1;
pub const XK_Omacron: u32 = 0x3d2;
pub const XK_Kcedilla: u32 = 0x3d3;
pub const XK_Uogonek: u32 = 0x3d9;
pub const XK_Utilde: u32 = 0x3dd;
pub const XK_Umacron: u32 = 0x3de;
pub const XK_amacron: u32 = 0x3e0;
pub const XK_iogonek: u32 = 0x3e7;
pub const XK_eabovedot: u32 = 0x3ec;
pub const XK_imacron: u32 = 0x3ef;
pub const XK_ncedilla: u32 = 0x3f1;
pub const XK_omacron: u32 = 0x3f2;
pub const XK_kcedilla: u32 = 0x3f3;
pub const XK_uogonek: u32 = 0x3f9;
pub const XK_utilde: u32 = 0x3fd;
pub const XK_umacron: u32 = 0x3fe;
pub const XK_overline: u32 = 0x47e;
pub const XK_kana_fullstop: u32 = 0x4a1;
pub const XK_kana_openingbracket: u32 = 0x4a2;
pub const XK_kana_closingbracket: u32 = 0x4a3;
pub const XK_kana_comma: u32 = 0x4a4;
pub const XK_kana_conjunctive: u32 = 0x4a5;
pub const XK_kana_middledot: u32 = 0x4a5;
pub const XK_kana_WO: u32 = 0x4a6;
pub const XK_kana_a: u32 = 0x4a7;
pub const XK_kana_i: u32 = 0x4a8;
pub const XK_kana_u: u32 = 0x4a9;
pub const XK_kana_e: u32 = 0x4aa;
pub const XK_kana_o: u32 = 0x4ab;
pub const XK_kana_ya: u32 = 0x4ac;
pub const XK_kana_yu: u32 = 0x4ad;
pub const XK_kana_yo: u32 = 0x4ae;
pub const XK_kana_tsu: u32 = 0x4af;
pub const XK_kana_tu: u32 = 0x4af;
pub const XK_prolongedsound: u32 = 0x4b0;
pub const XK_kana_A: u32 = 0x4b1;
pub const XK_kana_I: u32 = 0x4b2;
pub const XK_kana_U: u32 = 0x4b3;
pub const XK_kana_E: u32 = 0x4b4;
pub const XK_kana_O: u32 = 0x4b5;
pub const XK_kana_KA: u32 = 0x4b6;
pub const XK_kana_KI: u32 = 0x4b7;
pub const XK_kana_KU: u32 = 0x4b8;
pub const XK_kana_KE: u32 = 0x4b9;
pub const XK_kana_KO: u32 = 0x4ba;
pub const XK_kana_SA: u32 = 0x4bb;
pub const XK_kana_SHI: u32 = 0x4bc;
pub const XK_kana_SU: u32 = 0x4bd;
pub const XK_kana_SE: u32 = 0x4be;
pub const XK_kana_SO: u32 = 0x4bf;
pub const XK_kana_TA: u32 = 0x4c0;
pub const XK_kana_CHI: u32 = 0x4c1;
pub const XK_kana_TI: u32 = 0x4c1;
pub const XK_kana_TSU: u32 = 0x4c2;
pub const XK_kana_TU: u32 = 0x4c2;
pub const XK_kana_TE: u32 = 0x4c3;
pub const XK_kana_TO: u32 = 0x4c4;
pub const XK_kana_NA: u32 = 0x4c5;
pub const XK_kana_NI: u32 = 0x4c6;
pub const XK_kana_NU: u32 = 0x4c7;
pub const XK_kana_NE: u32 = 0x4c8;
pub const XK_kana_NO: u32 = 0x4c9;
pub const XK_kana_HA: u32 = 0x4ca;
pub const XK_kana_HI: u32 = 0x4cb;
pub const XK_kana_FU: u32 = 0x4cc;
pub const XK_kana_HU: u32 = 0x4cc;
pub const XK_kana_HE: u32 = 0x4cd;
pub const XK_kana_HO: u32 = 0x4ce;
pub const XK_kana_MA: u32 = 0x4cf;
pub const XK_kana_MI: u32 = 0x4d0;
pub const XK_kana_MU: u32 = 0x4d1;
pub const XK_kana_ME: u32 = 0x4d2;
pub const XK_kana_MO: u32 = 0x4d3;
pub const XK_kana_YA: u32 = 0x4d4;
pub const XK_kana_YU: u32 = 0x4d5;
pub const XK_kana_YO: u32 = 0x4d6;
pub const XK_kana_RA: u32 = 0x4d7;
pub const XK_kana_RI: u32 = 0x4d8;
pub const XK_kana_RU: u32 = 0x4d9;
pub const XK_kana_RE: u32 = 0x4da;
pub const XK_kana_RO: u32 = 0x4db;
pub const XK_kana_WA: u32 = 0x4dc;
pub const XK_kana_N: u32 = 0x4dd;
pub const XK_voicedsound: u32 = 0x4de;
pub const XK_semivoicedsound: u32 = 0x4df;
pub const XK_kana_switch: u32 = 0xFF7E;
pub const XK_Arabic_comma: u32 = 0x5ac;
pub const XK_Arabic_semicolon: u32 = 0x5bb;
pub const XK_Arabic_question_mark: u32 = 0x5bf;
pub const XK_Arabic_hamza: u32 = 0x5c1;
pub const XK_Arabic_maddaonalef: u32 = 0x5c2;
pub const XK_Arabic_hamzaonalef: u32 = 0x5c3;
pub const XK_Arabic_hamzaonwaw: u32 = 0x5c4;
pub const XK_Arabic_hamzaunderalef: u32 = 0x5c5;
pub const XK_Arabic_hamzaonyeh: u32 = 0x5c6;
pub const XK_Arabic_alef: u32 = 0x5c7;
pub const XK_Arabic_beh: u32 = 0x5c8;
pub const XK_Arabic_tehmarbuta: u32 = 0x5c9;
pub const XK_Arabic_teh: u32 = 0x5ca;
pub const XK_Arabic_theh: u32 = 0x5cb;
pub const XK_Arabic_jeem: u32 = 0x5cc;
pub const XK_Arabic_hah: u32 = 0x5cd;
pub const XK_Arabic_khah: u32 = 0x5ce;
pub const XK_Arabic_dal: u32 = 0x5cf;
pub const XK_Arabic_thal: u32 = 0x5d0;
pub const XK_Arabic_ra: u32 = 0x5d1;
pub const XK_Arabic_zain: u32 = 0x5d2;
pub const XK_Arabic_seen: u32 = 0x5d3;
pub const XK_Arabic_sheen: u32 = 0x5d4;
pub const XK_Arabic_sad: u32 = 0x5d5;
pub const XK_Arabic_dad: u32 = 0x5d6;
pub const XK_Arabic_tah: u32 = 0x5d7;
pub const XK_Arabic_zah: u32 = 0x5d8;
pub const XK_Arabic_ain: u32 = 0x5d9;
pub const XK_Arabic_ghain: u32 = 0x5da;
pub const XK_Arabic_tatweel: u32 = 0x5e0;
pub const XK_Arabic_feh: u32 = 0x5e1;
pub const XK_Arabic_qaf: u32 = 0x5e2;
pub const XK_Arabic_kaf: u32 = 0x5e3;
pub const XK_Arabic_lam: u32 = 0x5e4;
pub const XK_Arabic_meem: u32 = 0x5e5;
pub const XK_Arabic_noon: u32 = 0x5e6;
pub const XK_Arabic_ha: u32 = 0x5e7;
pub const XK_Arabic_heh: u32 = 0x5e7;
pub const XK_Arabic_waw: u32 = 0x5e8;
pub const XK_Arabic_alefmaksura: u32 = 0x5e9;
pub const XK_Arabic_yeh: u32 = 0x5ea;
pub const XK_Arabic_fathatan: u32 = 0x5eb;
pub const XK_Arabic_dammatan: u32 = 0x5ec;
pub const XK_Arabic_kasratan: u32 = 0x5ed;
pub const XK_Arabic_fatha: u32 = 0x5ee;
pub const XK_Arabic_damma: u32 = 0x5ef;
pub const XK_Arabic_kasra: u32 = 0x5f0;
pub const XK_Arabic_shadda: u32 = 0x5f1;
pub const XK_Arabic_sukun: u32 = 0x5f2;
pub const XK_Arabic_switch: u32 = 0xFF7E;
pub const XK_Serbian_dje: u32 = 0x6a1;
pub const XK_Macedonia_gje: u32 = 0x6a2;
pub const XK_Cyrillic_io: u32 = 0x6a3;
pub const XK_Ukrainian_ie: u32 = 0x6a4;
pub const XK_Ukranian_je: u32 = 0x6a4;
pub const XK_Macedonia_dse: u32 = 0x6a5;
pub const XK_Ukrainian_i: u32 = 0x6a6;
pub const XK_Ukranian_i: u32 = 0x6a6;
pub const XK_Ukrainian_yi: u32 = 0x6a7;
pub const XK_Ukranian_yi: u32 = 0x6a7;
pub const XK_Cyrillic_je: u32 = 0x6a8;
pub const XK_Serbian_je: u32 = 0x6a8;
pub const XK_Cyrillic_lje: u32 = 0x6a9;
pub const XK_Serbian_lje: u32 = 0x6a9;
pub const XK_Cyrillic_nje: u32 = 0x6aa;
pub const XK_Serbian_nje: u32 = 0x6aa;
pub const XK_Serbian_tshe: u32 = 0x6ab;
pub const XK_Macedonia_kje: u32 = 0x6ac;
pub const XK_Byelorussian_shortu: u32 = 0x6ae;
pub const XK_Cyrillic_dzhe: u32 = 0x6af;
pub const XK_Serbian_dze: u32 = 0x6af;
pub const XK_numerosign: u32 = 0x6b0;
pub const XK_Serbian_DJE: u32 = 0x6b1;
pub const XK_Macedonia_GJE: u32 = 0x6b2;
pub const XK_Cyrillic_IO: u32 = 0x6b3;
pub const XK_Ukrainian_IE: u32 = 0x6b4;
pub const XK_Ukranian_JE: u32 = 0x6b4;
pub const XK_Macedonia_DSE: u32 = 0x6b5;
pub const XK_Ukrainian_I: u32 = 0x6b6;
pub const XK_Ukranian_I: u32 = 0x6b6;
pub const XK_Ukrainian_YI: u32 = 0x6b7;
pub const XK_Ukranian_YI: u32 = 0x6b7;
pub const XK_Cyrillic_JE: u32 = 0x6b8;
pub const XK_Serbian_JE: u32 = 0x6b8;
pub const XK_Cyrillic_LJE: u32 = 0x6b9;
pub const XK_Serbian_LJE: u32 = 0x6b9;
pub const XK_Cyrillic_NJE: u32 = 0x6ba;
pub const XK_Serbian_NJE: u32 = 0x6ba;
pub const XK_Serbian_TSHE: u32 = 0x6bb;
pub const XK_Macedonia_KJE: u32 = 0x6bc;
pub const XK_Byelorussian_SHORTU: u32 = 0x6be;
pub const XK_Cyrillic_DZHE: u32 = 0x6bf;
pub const XK_Serbian_DZE: u32 = 0x6bf;
pub const XK_Cyrillic_yu: u32 = 0x6c0;
pub const XK_Cyrillic_a: u32 = 0x6c1;
pub const XK_Cyrillic_be: u32 = 0x6c2;
pub const XK_Cyrillic_tse: u32 = 0x6c3;
pub const XK_Cyrillic_de: u32 = 0x6c4;
pub const XK_Cyrillic_ie: u32 = 0x6c5;
pub const XK_Cyrillic_ef: u32 = 0x6c6;
pub const XK_Cyrillic_ghe: u32 = 0x6c7;
pub const XK_Cyrillic_ha: u32 = 0x6c8;
pub const XK_Cyrillic_i: u32 = 0x6c9;
pub const XK_Cyrillic_shorti: u32 = 0x6ca;
pub const XK_Cyrillic_ka: u32 = 0x6cb;
pub const XK_Cyrillic_el: u32 = 0x6cc;
pub const XK_Cyrillic_em: u32 = 0x6cd;
pub const XK_Cyrillic_en: u32 = 0x6ce;
pub const XK_Cyrillic_o: u32 = 0x6cf;
pub const XK_Cyrillic_pe: u32 = 0x6d0;
pub const XK_Cyrillic_ya: u32 = 0x6d1;
pub const XK_Cyrillic_er: u32 = 0x6d2;
pub const XK_Cyrillic_es: u32 = 0x6d3;
pub const XK_Cyrillic_te: u32 = 0x6d4;
pub const XK_Cyrillic_u: u32 = 0x6d5;
pub const XK_Cyrillic_zhe: u32 = 0x6d6;
pub const XK_Cyrillic_ve: u32 = 0x6d7;
pub const XK_Cyrillic_softsign: u32 = 0x6d8;
pub const XK_Cyrillic_yeru: u32 = 0x6d9;
pub const XK_Cyrillic_ze: u32 = 0x6da;
pub const XK_Cyrillic_sha: u32 = 0x6db;
pub const XK_Cyrillic_e: u32 = 0x6dc;
pub const XK_Cyrillic_shcha: u32 = 0x6dd;
pub const XK_Cyrillic_che: u32 = 0x6de;
pub const XK_Cyrillic_hardsign: u32 = 0x6df;
pub const XK_Cyrillic_YU: u32 = 0x6e0;
pub const XK_Cyrillic_A: u32 = 0x6e1;
pub const XK_Cyrillic_BE: u32 = 0x6e2;
pub const XK_Cyrillic_TSE: u32 = 0x6e3;
pub const XK_Cyrillic_DE: u32 = 0x6e4;
pub const XK_Cyrillic_IE: u32 = 0x6e5;
pub const XK_Cyrillic_EF: u32 = 0x6e6;
pub const XK_Cyrillic_GHE: u32 = 0x6e7;
pub const XK_Cyrillic_HA: u32 = 0x6e8;
pub const XK_Cyrillic_I: u32 = 0x6e9;
pub const XK_Cyrillic_SHORTI: u32 = 0x6ea;
pub const XK_Cyrillic_KA: u32 = 0x6eb;
pub const XK_Cyrillic_EL: u32 = 0x6ec;
pub const XK_Cyrillic_EM: u32 = 0x6ed;
pub const XK_Cyrillic_EN: u32 = 0x6ee;
pub const XK_Cyrillic_O: u32 = 0x6ef;
pub const XK_Cyrillic_PE: u32 = 0x6f0;
pub const XK_Cyrillic_YA: u32 = 0x6f1;
pub const XK_Cyrillic_ER: u32 = 0x6f2;
pub const XK_Cyrillic_ES: u32 = 0x6f3;
pub const XK_Cyrillic_TE: u32 = 0x6f4;
pub const XK_Cyrillic_U: u32 = 0x6f5;
pub const XK_Cyrillic_ZHE: u32 = 0x6f6;
pub const XK_Cyrillic_VE: u32 = 0x6f7;
pub const XK_Cyrillic_SOFTSIGN: u32 = 0x6f8;
pub const XK_Cyrillic_YERU: u32 = 0x6f9;
pub const XK_Cyrillic_ZE: u32 = 0x6fa;
pub const XK_Cyrillic_SHA: u32 = 0x6fb;
pub const XK_Cyrillic_E: u32 = 0x6fc;
pub const XK_Cyrillic_SHCHA: u32 = 0x6fd;
pub const XK_Cyrillic_CHE: u32 = 0x6fe;
pub const XK_Cyrillic_HARDSIGN: u32 = 0x6ff;
pub const XK_Greek_ALPHAaccent: u32 = 0x7a1;
pub const XK_Greek_EPSILONaccent: u32 = 0x7a2;
pub const XK_Greek_ETAaccent: u32 = 0x7a3;
pub const XK_Greek_IOTAaccent: u32 = 0x7a4;
pub const XK_Greek_IOTAdiaeresis: u32 = 0x7a5;
pub const XK_Greek_OMICRONaccent: u32 = 0x7a7;
pub const XK_Greek_UPSILONaccent: u32 = 0x7a8;
pub const XK_Greek_UPSILONdieresis: u32 = 0x7a9;
pub const XK_Greek_OMEGAaccent: u32 = 0x7ab;
pub const XK_Greek_accentdieresis: u32 = 0x7ae;
pub const XK_Greek_horizbar: u32 = 0x7af;
pub const XK_Greek_alphaaccent: u32 = 0x7b1;
pub const XK_Greek_epsilonaccent: u32 = 0x7b2;
pub const XK_Greek_etaaccent: u32 = 0x7b3;
pub const XK_Greek_iotaaccent: u32 = 0x7b4;
pub const XK_Greek_iotadieresis: u32 = 0x7b5;
pub const XK_Greek_iotaaccentdieresis: u32 = 0x7b6;
pub const XK_Greek_omicronaccent: u32 = 0x7b7;
pub const XK_Greek_upsilonaccent: u32 = 0x7b8;
pub const XK_Greek_upsilondieresis: u32 = 0x7b9;
pub const XK_Greek_upsilonaccentdieresis: u32 = 0x7ba;
pub const XK_Greek_omegaaccent: u32 = 0x7bb;
pub const XK_Greek_ALPHA: u32 = 0x7c1;
pub const XK_Greek_BETA: u32 = 0x7c2;
pub const XK_Greek_GAMMA: u32 = 0x7c3;
pub const XK_Greek_DELTA: u32 = 0x7c4;
pub const XK_Greek_EPSILON: u32 = 0x7c5;
pub const XK_Greek_ZETA: u32 = 0x7c6;
pub const XK_Greek_ETA: u32 = 0x7c7;
pub const XK_Greek_THETA: u32 = 0x7c8;
pub const XK_Greek_IOTA: u32 = 0x7c9;
pub const XK_Greek_KAPPA: u32 = 0x7ca;
pub const XK_Greek_LAMDA: u32 = 0x7cb;
pub const XK_Greek_LAMBDA: u32 = 0x7cb;
pub const XK_Greek_MU: u32 = 0x7cc;
pub const XK_Greek_NU: u32 = 0x7cd;
pub const XK_Greek_XI: u32 = 0x7ce;
pub const XK_Greek_OMICRON: u32 = 0x7cf;
pub const XK_Greek_PI: u32 = 0x7d0;
pub const XK_Greek_RHO: u32 = 0x7d1;
pub const XK_Greek_SIGMA: u32 = 0x7d2;
pub const XK_Greek_TAU: u32 = 0x7d4;
pub const XK_Greek_UPSILON: u32 = 0x7d5;
pub const XK_Greek_PHI: u32 = 0x7d6;
pub const XK_Greek_CHI: u32 = 0x7d7;
pub const XK_Greek_PSI: u32 = 0x7d8;
pub const XK_Greek_OMEGA: u32 = 0x7d9;
pub const XK_Greek_alpha: u32 = 0x7e1;
pub const XK_Greek_beta: u32 = 0x7e2;
pub const XK_Greek_gamma: u32 = 0x7e3;
pub const XK_Greek_delta: u32 = 0x7e4;
pub const XK_Greek_epsilon: u32 = 0x7e5;
pub const XK_Greek_zeta: u32 = 0x7e6;
pub const XK_Greek_eta: u32 = 0x7e7;
pub const XK_Greek_theta: u32 = 0x7e8;
pub const XK_Greek_iota: u32 = 0x7e9;
pub const XK_Greek_kappa: u32 = 0x7ea;
pub const XK_Greek_lamda: u32 = 0x7eb;
pub const XK_Greek_lambda: u32 = 0x7eb;
pub const XK_Greek_mu: u32 = 0x7ec;
pub const XK_Greek_nu: u32 = 0x7ed;
pub const XK_Greek_xi: u32 = 0x7ee;
pub const XK_Greek_omicron: u32 = 0x7ef;
pub const XK_Greek_pi: u32 = 0x7f0;
pub const XK_Greek_rho: u32 = 0x7f1;
pub const XK_Greek_sigma: u32 = 0x7f2;
pub const XK_Greek_finalsmallsigma: u32 = 0x7f3;
pub const XK_Greek_tau: u32 = 0x7f4;
pub const XK_Greek_upsilon: u32 = 0x7f5;
pub const XK_Greek_phi: u32 = 0x7f6;
pub const XK_Greek_chi: u32 = 0x7f7;
pub const XK_Greek_psi: u32 = 0x7f8;
pub const XK_Greek_omega: u32 = 0x7f9;
pub const XK_Greek_switch: u32 = 0xFF7E;
pub const XK_leftradical: u32 = 0x8a1;
pub const XK_topleftradical: u32 = 0x8a2;
pub const XK_horizconnector: u32 = 0x8a3;
pub const XK_topintegral: u32 = 0x8a4;
pub const XK_botintegral: u32 = 0x8a5;
pub const XK_vertconnector: u32 = 0x8a6;
pub const XK_topleftsqbracket: u32 = 0x8a7;
pub const XK_botleftsqbracket: u32 = 0x8a8;
pub const XK_toprightsqbracket: u32 = 0x8a9;
pub const XK_botrightsqbracket: u32 = 0x8aa;
pub const XK_topleftparens: u32 = 0x8ab;
pub const XK_botleftparens: u32 = 0x8ac;
pub const XK_toprightparens: u32 = 0x8ad;
pub const XK_botrightparens: u32 = 0x8ae;
pub const XK_leftmiddlecurlybrace: u32 = 0x8af;
pub const XK_rightmiddlecurlybrace: u32 = 0x8b0;
pub const XK_topleftsummation: u32 = 0x8b1;
pub const XK_botleftsummation: u32 = 0x8b2;
pub const XK_topvertsummationconnector: u32 = 0x8b3;
pub const XK_botvertsummationconnector: u32 = 0x8b4;
pub const XK_toprightsummation: u32 = 0x8b5;
pub const XK_botrightsummation: u32 = 0x8b6;
pub const XK_rightmiddlesummation: u32 = 0x8b7;
pub const XK_lessthanequal: u32 = 0x8bc;
pub const XK_notequal: u32 = 0x8bd;
pub const XK_greaterthanequal: u32 = 0x8be;
pub const XK_integral: u32 = 0x8bf;
pub const XK_therefore: u32 = 0x8c0;
pub const XK_variation: u32 = 0x8c1;
pub const XK_infinity: u32 = 0x8c2;
pub const XK_nabla: u32 = 0x8c5;
pub const XK_approximate: u32 = 0x8c8;
pub const XK_similarequal: u32 = 0x8c9;
pub const XK_ifonlyif: u32 = 0x8cd;
pub const XK_implies: u32 = 0x8ce;
pub const XK_identical: u32 = 0x8cf;
pub const XK_radical: u32 = 0x8d6;
pub const XK_includedin: u32 = 0x8da;
pub const XK_includes: u32 = 0x8db;
pub const XK_intersection: u32 = 0x8dc;
pub const XK_union: u32 = 0x8dd;
pub const XK_logicaland: u32 = 0x8de;
pub const XK_logicalor: u32 = 0x8df;
pub const XK_partialderivative: u32 = 0x8ef;
pub const XK_function: u32 = 0x8f6;
pub const XK_leftarrow: u32 = 0x8fb;
pub const XK_uparrow: u32 = 0x8fc;
pub const XK_rightarrow: u32 = 0x8fd;
pub const XK_downarrow: u32 = 0x8fe;
pub const XK_blank: u32 = 0x9df;
pub const XK_soliddiamond: u32 = 0x9e0;
pub const XK_checkerboard: u32 = 0x9e1;
pub const XK_ht: u32 = 0x9e2;
pub const XK_ff: u32 = 0x9e3;
pub const XK_cr: u32 = 0x9e4;
pub const XK_lf: u32 = 0x9e5;
pub const XK_nl: u32 = 0x9e8;
pub const XK_vt: u32 = 0x9e9;
pub const XK_lowrightcorner: u32 = 0x9ea;
pub const XK_uprightcorner: u32 = 0x9eb;
pub const XK_upleftcorner: u32 = 0x9ec;
pub const XK_lowleftcorner: u32 = 0x9ed;
pub const XK_crossinglines: u32 = 0x9ee;
pub const XK_horizlinescan1: u32 = 0x9ef;
pub const XK_horizlinescan3: u32 = 0x9f0;
pub const XK_horizlinescan5: u32 = 0x9f1;
pub const XK_horizlinescan7: u32 = 0x9f2;
pub const XK_horizlinescan9: u32 = 0x9f3;
pub const XK_leftt: u32 = 0x9f4;
pub const XK_rightt: u32 = 0x9f5;
pub const XK_bott: u32 = 0x9f6;
pub const XK_topt: u32 = 0x9f7;
pub const XK_vertbar: u32 = 0x9f8;
pub const XK_emspace: u32 = 0xaa1;
pub const XK_enspace: u32 = 0xaa2;
pub const XK_em3space: u32 = 0xaa3;
pub const XK_em4space: u32 = 0xaa4;
pub const XK_digitspace: u32 = 0xaa5;
pub const XK_punctspace: u32 = 0xaa6;
pub const XK_thinspace: u32 = 0xaa7;
pub const XK_hairspace: u32 = 0xaa8;
pub const XK_emdash: u32 = 0xaa9;
pub const XK_endash: u32 = 0xaaa;
pub const XK_signifblank: u32 = 0xaac;
pub const XK_ellipsis: u32 = 0xaae;
pub const XK_doubbaselinedot: u32 = 0xaaf;
pub const XK_onethird: u32 = 0xab0;
pub const XK_twothirds: u32 = 0xab1;
pub const XK_onefifth: u32 = 0xab2;
pub const XK_twofifths: u32 = 0xab3;
pub const XK_threefifths: u32 = 0xab4;
pub const XK_fourfifths: u32 = 0xab5;
pub const XK_onesixth: u32 = 0xab6;
pub const XK_fivesixths: u32 = 0xab7;
pub const XK_careof: u32 = 0xab8;
pub const XK_figdash: u32 = 0xabb;
pub const XK_leftanglebracket: u32 = 0xabc;
pub const XK_decimalpoint: u32 = 0xabd;
pub const XK_rightanglebracket: u32 = 0xabe;
pub const XK_marker: u32 = 0xabf;
pub const XK_oneeighth: u32 = 0xac3;
pub const XK_threeeighths: u32 = 0xac4;
pub const XK_fiveeighths: u32 = 0xac5;
pub const XK_seveneighths: u32 = 0xac6;
pub const XK_trademark: u32 = 0xac9;
pub const XK_signaturemark: u32 = 0xaca;
pub const XK_trademarkincircle: u32 = 0xacb;
pub const XK_leftopentriangle: u32 = 0xacc;
pub const XK_rightopentriangle: u32 = 0xacd;
pub const XK_emopencircle: u32 = 0xace;
pub const XK_emopenrectangle: u32 = 0xacf;
pub const XK_leftsinglequotemark: u32 = 0xad0;
pub const XK_rightsinglequotemark: u32 = 0xad1;
pub const XK_leftdoublequotemark: u32 = 0xad2;
pub const XK_rightdoublequotemark: u32 = 0xad3;
pub const XK_prescription: u32 = 0xad4;
pub const XK_minutes: u32 = 0xad6;
pub const XK_seconds: u32 = 0xad7;
pub const XK_latincross: u32 = 0xad9;
pub const XK_hexagram: u32 = 0xada;
pub const XK_filledrectbullet: u32 = 0xadb;
pub const XK_filledlefttribullet: u32 = 0xadc;
pub const XK_filledrighttribullet: u32 = 0xadd;
pub const XK_emfilledcircle: u32 = 0xade;
pub const XK_emfilledrect: u32 = 0xadf;
pub const XK_enopencircbullet: u32 = 0xae0;
pub const XK_enopensquarebullet: u32 = 0xae1;
pub const XK_openrectbullet: u32 = 0xae2;
pub const XK_opentribulletup: u32 = 0xae3;
pub const XK_opentribulletdown: u32 = 0xae4;
pub const XK_openstar: u32 = 0xae5;
pub const XK_enfilledcircbullet: u32 = 0xae6;
pub const XK_enfilledsqbullet: u32 = 0xae7;
pub const XK_filledtribulletup: u32 = 0xae8;
pub const XK_filledtribulletdown: u32 = 0xae9;
pub const XK_leftpointer: u32 = 0xaea;
pub const XK_rightpointer: u32 = 0xaeb;
pub const XK_club: u32 = 0xaec;
pub const XK_diamond: u32 = 0xaed;
pub const XK_heart: u32 = 0xaee;
pub const XK_maltesecross: u32 = 0xaf0;
pub const XK_dagger: u32 = 0xaf1;
pub const XK_doubledagger: u32 = 0xaf2;
pub const XK_checkmark: u32 = 0xaf3;
pub const XK_ballotcross: u32 = 0xaf4;
pub const XK_musicalsharp: u32 = 0xaf5;
pub const XK_musicalflat: u32 = 0xaf6;
pub const XK_malesymbol: u32 = 0xaf7;
pub const XK_femalesymbol: u32 = 0xaf8;
pub const XK_telephone: u32 = 0xaf9;
pub const XK_telephonerecorder: u32 = 0xafa;
pub const XK_phonographcopyright: u32 = 0xafb;
pub const XK_caret: u32 = 0xafc;
pub const XK_singlelowquotemark: u32 = 0xafd;
pub const XK_doublelowquotemark: u32 = 0xafe;
pub const XK_cursor: u32 = 0xaff;
pub const XK_leftcaret: u32 = 0xba3;
pub const XK_rightcaret: u32 = 0xba6;
pub const XK_downcaret: u32 = 0xba8;
pub const XK_upcaret: u32 = 0xba9;
pub const XK_overbar: u32 = 0xbc0;
pub const XK_downtack: u32 = 0xbc2;
pub const XK_upshoe: u32 = 0xbc3;
pub const XK_downstile: u32 = 0xbc4;
pub const XK_underbar: u32 = 0xbc6;
pub const XK_jot: u32 = 0xbca;
pub const XK_quad: u32 = 0xbcc;
pub const XK_uptack: u32 = 0xbce;
pub const XK_circle: u32 = 0xbcf;
pub const XK_upstile: u32 = 0xbd3;
pub const XK_downshoe: u32 = 0xbd6;
pub const XK_rightshoe: u32 = 0xbd8;
pub const XK_leftshoe: u32 = 0xbda;
pub const XK_lefttack: u32 = 0xbdc;
pub const XK_righttack: u32 = 0xbfc;
pub const XK_hebrew_doublelowline: u32 = 0xcdf;
pub const XK_hebrew_aleph: u32 = 0xce0;
pub const XK_hebrew_bet: u32 = 0xce1;
pub const XK_hebrew_beth: u32 = 0xce1;
pub const XK_hebrew_gimel: u32 = 0xce2;
pub const XK_hebrew_gimmel: u32 = 0xce2;
pub const XK_hebrew_dalet: u32 = 0xce3;
pub const XK_hebrew_daleth: u32 = 0xce3;
pub const XK_hebrew_he: u32 = 0xce4;
pub const XK_hebrew_waw: u32 = 0xce5;
pub const XK_hebrew_zain: u32 = 0xce6;
pub const XK_hebrew_zayin: u32 = 0xce6;
pub const XK_hebrew_chet: u32 = 0xce7;
pub const XK_hebrew_het: u32 = 0xce7;
pub const XK_hebrew_tet: u32 = 0xce8;
pub const XK_hebrew_teth: u32 = 0xce8;
pub const XK_hebrew_yod: u32 = 0xce9;
pub const XK_hebrew_finalkaph: u32 = 0xcea;
pub const XK_hebrew_kaph: u32 = 0xceb;
pub const XK_hebrew_lamed: u32 = 0xcec;
pub const XK_hebrew_finalmem: u32 = 0xced;
pub const XK_hebrew_mem: u32 = 0xcee;
pub const XK_hebrew_finalnun: u32 = 0xcef;
pub const XK_hebrew_nun: u32 = 0xcf0;
pub const XK_hebrew_samech: u32 = 0xcf1;
pub const XK_hebrew_samekh: u32 = 0xcf1;
pub const XK_hebrew_ayin: u32 = 0xcf2;
pub const XK_hebrew_finalpe: u32 = 0xcf3;
pub const XK_hebrew_pe: u32 = 0xcf4;
pub const XK_hebrew_finalzade: u32 = 0xcf5;
pub const XK_hebrew_finalzadi: u32 = 0xcf5;
pub const XK_hebrew_zade: u32 = 0xcf6;
pub const XK_hebrew_zadi: u32 = 0xcf6;
pub const XK_hebrew_qoph: u32 = 0xcf7;
pub const XK_hebrew_kuf: u32 = 0xcf7;
pub const XK_hebrew_resh: u32 = 0xcf8;
pub const XK_hebrew_shin: u32 = 0xcf9;
pub const XK_hebrew_taw: u32 = 0xcfa;
pub const XK_hebrew_taf: u32 = 0xcfa;
pub const XK_Hebrew_switch: u32 = 0xFF7E;

pub const XF86XK_ModeLock: u32 = 0x1008FF01;
pub const XF86XK_MonBrightnessUp: u32 = 0x1008FF02;
pub const XF86XK_MonBrightnessDown: u32 = 0x1008FF03;
pub const XF86XK_KbdLightOnOff: u32 = 0x1008FF04;
pub const XF86XK_KbdBrightnessUp: u32 = 0x1008FF05;
pub const XF86XK_KbdBrightnessDown: u32 = 0x1008FF06;
pub const XF86XK_Standby: u32 = 0x1008FF10;
pub const XF86XK_AudioLowerVolume: u32 = 0x1008FF11;
pub const XF86XK_AudioMute: u32 = 0x1008FF12;
pub const XF86XK_AudioRaiseVolume: u32 = 0x1008FF13;
pub const XF86XK_AudioPlay: u32 = 0x1008FF14;
pub const XF86XK_AudioStop: u32 = 0x1008FF15;
pub const XF86XK_AudioPrev: u32 = 0x1008FF16;
pub const XF86XK_AudioNext: u32 = 0x1008FF17;
pub const XF86XK_HomePage: u32 = 0x1008FF18;
pub const XF86XK_Mail: u32 = 0x1008FF19;
pub const XF86XK_Start: u32 = 0x1008FF1A;
pub const XF86XK_Search: u32 = 0x1008FF1B;
pub const XF86XK_AudioRecord: u32 = 0x1008FF1C;
pub const XF86XK_Calculator: u32 = 0x1008FF1D;
pub const XF86XK_Memo: u32 = 0x1008FF1E;
pub const XF86XK_ToDoList: u32 = 0x1008FF1F;
pub const XF86XK_Calendar: u32 = 0x1008FF20;
pub const XF86XK_PowerDown: u32 = 0x1008FF21;
pub const XF86XK_ContrastAdjust: u32 = 0x1008FF22;
pub const XF86XK_RockerUp: u32 = 0x1008FF23;
pub const XF86XK_RockerDown: u32 = 0x1008FF24;
pub const XF86XK_RockerEnter: u32 = 0x1008FF25;
pub const XF86XK_Back: u32 = 0x1008FF26;
pub const XF86XK_Forward: u32 = 0x1008FF27;
pub const XF86XK_Stop: u32 = 0x1008FF28;
pub const XF86XK_Refresh: u32 = 0x1008FF29;
pub const XF86XK_PowerOff: u32 = 0x1008FF2A;
pub const XF86XK_WakeUp: u32 = 0x1008FF2B;
pub const XF86XK_Eject: u32 = 0x1008FF2C;
pub const XF86XK_ScreenSaver: u32 = 0x1008FF2D;
pub const XF86XK_WWW: u32 = 0x1008FF2E;
pub const XF86XK_Sleep: u32 = 0x1008FF2F;
pub const XF86XK_Favorites: u32 = 0x1008FF30;
pub const XF86XK_AudioPause: u32 = 0x1008FF31;
pub const XF86XK_AudioMedia: u32 = 0x1008FF32;
pub const XF86XK_MyComputer: u32 = 0x1008FF33;
pub const XF86XK_VendorHome: u32 = 0x1008FF34;
pub const XF86XK_LightBulb: u32 = 0x1008FF35;
pub const XF86XK_Shop: u32 = 0x1008FF36;
pub const XF86XK_History: u32 = 0x1008FF37;
pub const XF86XK_OpenURL: u32 = 0x1008FF38;
pub const XF86XK_AddFavorite: u32 = 0x1008FF39;
pub const XF86XK_HotLinks: u32 = 0x1008FF3A;
pub const XF86XK_BrightnessAdjust: u32 = 0x1008FF3B;
pub const XF86XK_Finance: u32 = 0x1008FF3C;
pub const XF86XK_Community: u32 = 0x1008FF3D;
pub const XF86XK_AudioRewind: u32 = 0x1008FF3E;
pub const XF86XK_BackForward: u32 = 0x1008FF3F;
pub const XF86XK_Launch0: u32 = 0x1008FF40;
pub const XF86XK_Launch1: u32 = 0x1008FF41;
pub const XF86XK_Launch2: u32 = 0x1008FF42;
pub const XF86XK_Launch3: u32 = 0x1008FF43;
pub const XF86XK_Launch4: u32 = 0x1008FF44;
pub const XF86XK_Launch5: u32 = 0x1008FF45;
pub const XF86XK_Launch6: u32 = 0x1008FF46;
pub const XF86XK_Launch7: u32 = 0x1008FF47;
pub const XF86XK_Launch8: u32 = 0x1008FF48;
pub const XF86XK_Launch9: u32 = 0x1008FF49;
pub const XF86XK_LaunchA: u32 = 0x1008FF4A;
pub const XF86XK_LaunchB: u32 = 0x1008FF4B;
pub const XF86XK_LaunchC: u32 = 0x1008FF4C;
pub const XF86XK_LaunchD: u32 = 0x1008FF4D;
pub const XF86XK_LaunchE: u32 = 0x1008FF4E;
pub const XF86XK_LaunchF: u32 = 0x1008FF4F;
pub const XF86XK_ApplicationLeft: u32 = 0x1008FF50;
pub const XF86XK_ApplicationRight: u32 = 0x1008FF51;
pub const XF86XK_Book: u32 = 0x1008FF52;
pub const XF86XK_CD: u32 = 0x1008FF53;
pub const XF86XK_Calculater: u32 = 0x1008FF54;
pub const XF86XK_Clear: u32 = 0x1008FF55;
pub const XF86XK_Close: u32 = 0x1008FF56;
pub const XF86XK_Copy: u32 = 0x1008FF57;
pub const XF86XK_Cut: u32 = 0x1008FF58;
pub const XF86XK_Display: u32 = 0x1008FF59;
pub const XF86XK_DOS: u32 = 0x1008FF5A;
pub const XF86XK_Documents: u32 = 0x1008FF5B;
pub const XF86XK_Excel: u32 = 0x1008FF5C;
pub const XF86XK_Explorer: u32 = 0x1008FF5D;
pub const XF86XK_Game: u32 = 0x1008FF5E;
pub const XF86XK_Go: u32 = 0x1008FF5F;
pub const XF86XK_iTouch: u32 = 0x1008FF60;
pub const XF86XK_LogOff: u32 = 0x1008FF61;
pub const XF86XK_Market: u32 = 0x1008FF62;
pub const XF86XK_Meeting: u32 = 0x1008FF63;
pub const XF86XK_MenuKB: u32 = 0x1008FF65;
pub const XF86XK_MenuPB: u32 = 0x1008FF66;
pub const XF86XK_MySites: u32 = 0x1008FF67;
pub const XF86XK_New: u32 = 0x1008FF68;
pub const XF86XK_News: u32 = 0x1008FF69;
pub const XF86XK_OfficeHome: u32 = 0x1008FF6A;
pub const XF86XK_Open: u32 = 0x1008FF6B;
pub const XF86XK_Option: u32 = 0x1008FF6C;
pub const XF86XK_Paste: u32 = 0x1008FF6D;
pub const XF86XK_Phone: u32 = 0x1008FF6E;
pub const XF86XK_Q: u32 = 0x1008FF70;
pub const XF86XK_Reply: u32 = 0x1008FF72;
pub const XF86XK_Reload: u32 = 0x1008FF73;
pub const XF86XK_RotateWindows: u32 = 0x1008FF74;
pub const XF86XK_RotationPB: u32 = 0x1008FF75;
pub const XF86XK_RotationKB: u32 = 0x1008FF76;
pub const XF86XK_Save: u32 = 0x1008FF77;
pub const XF86XK_ScrollUp: u32 = 0x1008FF78;
pub const XF86XK_ScrollDown: u32 = 0x1008FF79;
pub const XF86XK_ScrollClick: u32 = 0x1008FF7A;
pub const XF86XK_Send: u32 = 0x1008FF7B;
pub const XF86XK_Spell: u32 = 0x1008FF7C;
pub const XF86XK_SplitScreen: u32 = 0x1008FF7D;
pub const XF86XK_Support: u32 = 0x1008FF7E;
pub const XF86XK_TaskPane: u32 = 0x1008FF7F;
pub const XF86XK_Terminal: u32 = 0x1008FF80;
pub const XF86XK_Tools: u32 = 0x1008FF81;
pub const XF86XK_Travel: u32 = 0x1008FF82;
pub const XF86XK_UserPB: u32 = 0x1008FF84;
pub const XF86XK_User1KB: u32 = 0x1008FF85;
pub const XF86XK_User2KB: u32 = 0x1008FF86;
pub const XF86XK_Video: u32 = 0x1008FF87;
pub const XF86XK_WheelButton: u32 = 0x1008FF88;
pub const XF86XK_Word: u32 = 0x1008FF89;
pub const XF86XK_Xfer: u32 = 0x1008FF8A;
pub const XF86XK_ZoomIn: u32 = 0x1008FF8B;
pub const XF86XK_ZoomOut: u32 = 0x1008FF8C;
pub const XF86XK_Away: u32 = 0x1008FF8D;
pub const XF86XK_Messenger: u32 = 0x1008FF8E;
pub const XF86XK_WebCam: u32 = 0x1008FF8F;
pub const XF86XK_MailForward: u32 = 0x1008FF90;
pub const XF86XK_Pictures: u32 = 0x1008FF91;
pub const XF86XK_Music: u32 = 0x1008FF92;
pub const XF86XK_Battery: u32 = 0x1008FF93;
pub const XF86XK_Bluetooth: u32 = 0x1008FF94;
pub const XF86XK_WLAN: u32 = 0x1008FF95;
pub const XF86XK_UWB: u32 = 0x1008FF96;
pub const XF86XK_AudioForward: u32 = 0x1008FF97;
pub const XF86XK_AudioRepeat: u32 = 0x1008FF98;
pub const XF86XK_AudioRandomPlay: u32 = 0x1008FF99;
pub const XF86XK_Subtitle: u32 = 0x1008FF9A;
pub const XF86XK_AudioCycleTrack: u32 = 0x1008FF9B;
pub const XF86XK_CycleAngle: u32 = 0x1008FF9C;
pub const XF86XK_FrameBack: u32 = 0x1008FF9D;
pub const XF86XK_FrameForward: u32 = 0x1008FF9E;
pub const XF86XK_Time: u32 = 0x1008FF9F;
pub const XF86XK_Select: u32 = 0x1008FFA0;
pub const XF86XK_View: u32 = 0x1008FFA1;
pub const XF86XK_TopMenu: u32 = 0x1008FFA2;
pub const XF86XK_Red: u32 = 0x1008FFA3;
pub const XF86XK_Green: u32 = 0x1008FFA4;
pub const XF86XK_Yellow: u32 = 0x1008FFA5;
pub const XF86XK_Blue: u32 = 0x1008FFA6;
pub const XF86XK_Suspend: u32 = 0x1008FFA7;
pub const XF86XK_Hibernate: u32 = 0x1008FFA8;
pub const XF86XK_TouchpadToggle: u32 = 0x1008FFA9;
pub const XF86XK_TouchpadOn: u32 = 0x1008FFB0;
pub const XF86XK_TouchpadOff: u32 = 0x1008FFB1;
pub const XF86XK_AudioMicMute: u32 = 0x1008FFB2;
pub const XF86XK_Switch_VT_1: u32 = 0x1008FE01;
pub const XF86XK_Switch_VT_2: u32 = 0x1008FE02;
pub const XF86XK_Switch_VT_3: u32 = 0x1008FE03;
pub const XF86XK_Switch_VT_4: u32 = 0x1008FE04;
pub const XF86XK_Switch_VT_5: u32 = 0x1008FE05;
pub const XF86XK_Switch_VT_6: u32 = 0x1008FE06;
pub const XF86XK_Switch_VT_7: u32 = 0x1008FE07;
pub const XF86XK_Switch_VT_8: u32 = 0x1008FE08;
pub const XF86XK_Switch_VT_9: u32 = 0x1008FE09;
pub const XF86XK_Switch_VT_10: u32 = 0x1008FE0A;
pub const XF86XK_Switch_VT_11: u32 = 0x1008FE0B;
pub const XF86XK_Switch_VT_12: u32 = 0x1008FE0C;
pub const XF86XK_Ungrab: u32 = 0x1008FE20;
pub const XF86XK_ClearGrab: u32 = 0x1008FE21;
pub const XF86XK_Next_VMode: u32 = 0x1008FE22;
pub const XF86XK_Prev_VMode: u32 = 0x1008FE23;
pub const XF86XK_LogWindowTree: u32 = 0x1008FE24;
pub const XF86XK_LogGrabInfo: u32 = 0x1008FE25;

pub const XK_ISO_Lock: u32 = 0xfe01;
pub const XK_ISO_Level2_Latch: u32 = 0xfe02;
pub const XK_ISO_Level3_Shift: u32 = 0xfe03;
pub const XK_ISO_Level3_Latch: u32 = 0xfe04;
pub const XK_ISO_Level3_Lock: u32 = 0xfe05;
pub const XK_ISO_Level5_Shift: u32 = 0xfe11;
pub const XK_ISO_Level5_Latch: u32 = 0xfe12;
pub const XK_ISO_Level5_Lock: u32 = 0xfe13;
pub const XK_ISO_Group_Shift: u32 = 0xff7e;
pub const XK_ISO_Group_Latch: u32 = 0xfe06;
pub const XK_ISO_Group_Lock: u32 = 0xfe07;
pub const XK_ISO_Next_Group: u32 = 0xfe08;
pub const XK_ISO_Next_Group_Lock: u32 = 0xfe09;
pub const XK_ISO_Prev_Group: u32 = 0xfe0a;
pub const XK_ISO_Prev_Group_Lock: u32 = 0xfe0b;
pub const XK_ISO_First_Group: u32 = 0xfe0c;
pub const XK_ISO_First_Group_Lock: u32 = 0xfe0d;
pub const XK_ISO_Last_Group: u32 = 0xfe0e;
pub const XK_ISO_Last_Group_Lock: u32 = 0xfe0f;

pub const XK_ISO_Left_Tab: u32 = 0xfe20;
pub const XK_ISO_Move_Line_Up: u32 = 0xfe21;
pub const XK_ISO_Move_Line_Down: u32 = 0xfe22;
pub const XK_ISO_Partial_Line_Up: u32 = 0xfe23;
pub const XK_ISO_Partial_Line_Down: u32 = 0xfe24;
pub const XK_ISO_Partial_Space_Left: u32 = 0xfe25;
pub const XK_ISO_Partial_Space_Right: u32 = 0xfe26;
pub const XK_ISO_Set_Margin_Left: u32 = 0xfe27;
pub const XK_ISO_Set_Margin_Right: u32 = 0xfe28;
pub const XK_ISO_Release_Margin_Left: u32 = 0xfe29;
pub const XK_ISO_Release_Margin_Right: u32 = 0xfe2a;
pub const XK_ISO_Release_Both_Margins: u32 = 0xfe2b;
pub const XK_ISO_Fast_Cursor_Left: u32 = 0xfe2c;
pub const XK_ISO_Fast_Cursor_Right: u32 = 0xfe2d;
pub const XK_ISO_Fast_Cursor_Up: u32 = 0xfe2e;
pub const XK_ISO_Fast_Cursor_Down: u32 = 0xfe2f;
pub const XK_ISO_Continuous_Underline: u32 = 0xfe30;
pub const XK_ISO_Discontinuous_Underline: u32 = 0xfe31;
pub const XK_ISO_Emphasize: u32 = 0xfe32;
pub const XK_ISO_Center_Object: u32 = 0xfe33;
pub const XK_ISO_Enter: u32 = 0xfe34;

pub const XK_dead_grave: u32 = 0xfe50;
pub const XK_dead_acute: u32 = 0xfe51;
pub const XK_dead_circumflex: u32 = 0xfe52;
pub const XK_dead_tilde: u32 = 0xfe53;
pub const XK_dead_perispomeni: u32 = 0xfe53;
pub const XK_dead_macron: u32 = 0xfe54;
pub const XK_dead_breve: u32 = 0xfe55;
pub const XK_dead_abovedot: u32 = 0xfe56;
pub const XK_dead_diaeresis: u32 = 0xfe57;
pub const XK_dead_abovering: u32 = 0xfe58;
pub const XK_dead_doubleacute: u32 = 0xfe59;
pub const XK_dead_caron: u32 = 0xfe5a;
pub const XK_dead_cedilla: u32 = 0xfe5b;
pub const XK_dead_ogonek: u32 = 0xfe5c;
pub const XK_dead_iota: u32 = 0xfe5d;
pub const XK_dead_voiced_sound: u32 = 0xfe5e;
pub const XK_dead_semivoiced_sound: u32 = 0xfe5f;
pub const XK_dead_belowdot: u32 = 0xfe60;
pub const XK_dead_hook: u32 = 0xfe61;
pub const XK_dead_horn: u32 = 0xfe62;
pub const XK_dead_stroke: u32 = 0xfe63;
pub const XK_dead_abovecomma: u32 = 0xfe64;
pub const XK_dead_psili: u32 = 0xfe64;
pub const XK_dead_abovereversedcomma: u32 = 0xfe65;
pub const XK_dead_dasia: u32 = 0xfe65;
pub const XK_dead_doublegrave: u32 = 0xfe66;
pub const XK_dead_belowring: u32 = 0xfe67;
pub const XK_dead_belowmacron: u32 = 0xfe68;
pub const XK_dead_belowcircumflex: u32 = 0xfe69;
pub const XK_dead_belowtilde: u32 = 0xfe6a;
pub const XK_dead_belowbreve: u32 = 0xfe6b;
pub const XK_dead_belowdiaeresis: u32 = 0xfe6c;
pub const XK_dead_invertedbreve: u32 = 0xfe6d;
pub const XK_dead_belowcomma: u32 = 0xfe6e;
pub const XK_dead_currency: u32 = 0xfe6f;

pub const XK_dead_lowline: u32 = 0xfe90;
pub const XK_dead_aboveverticalline: u32 = 0xfe91;
pub const XK_dead_belowverticalline: u32 = 0xfe92;
pub const XK_dead_longsolidusoverlay: u32 = 0xfe93;

pub const XK_dead_a: u32 = 0xfe80;
pub const XK_dead_A: u32 = 0xfe81;
pub const XK_dead_e: u32 = 0xfe82;
pub const XK_dead_E: u32 = 0xfe83;
pub const XK_dead_i: u32 = 0xfe84;
pub const XK_dead_I: u32 = 0xfe85;
pub const XK_dead_o: u32 = 0xfe86;
pub const XK_dead_O: u32 = 0xfe87;
pub const XK_dead_u: u32 = 0xfe88;
pub const XK_dead_U: u32 = 0xfe89;
pub const XK_dead_small_schwa: u32 = 0xfe8a;
pub const XK_dead_capital_schwa: u32 = 0xfe8b;

pub const XK_dead_greek: u32 = 0xfe8c;

pub const XK_First_Virtual_Screen: u32 = 0xfed0;
pub const XK_Prev_Virtual_Screen: u32 = 0xfed1;
pub const XK_Next_Virtual_Screen: u32 = 0xfed2;
pub const XK_Last_Virtual_Screen: u32 = 0xfed4;
pub const XK_Terminate_Server: u32 = 0xfed5;

pub const XK_AccessX_Enable: u32 = 0xfe70;
pub const XK_AccessX_Feedback_Enable: u32 = 0xfe71;
pub const XK_RepeatKeys_Enable: u32 = 0xfe72;
pub const XK_SlowKeys_Enable: u32 = 0xfe73;
pub const XK_BounceKeys_Enable: u32 = 0xfe74;
pub const XK_StickyKeys_Enable: u32 = 0xfe75;
pub const XK_MouseKeys_Enable: u32 = 0xfe76;
pub const XK_MouseKeys_Accel_Enable: u32 = 0xfe77;
pub const XK_Overlay1_Enable: u32 = 0xfe78;
pub const XK_Overlay2_Enable: u32 = 0xfe79;
pub const XK_AudibleBell_Enable: u32 = 0xfe7a;

pub const XK_Pointer_Left: u32 = 0xfee0;
pub const XK_Pointer_Right: u32 = 0xfee1;
pub const XK_Pointer_Up: u32 = 0xfee2;
pub const XK_Pointer_Down: u32 = 0xfee3;
pub const XK_Pointer_UpLeft: u32 = 0xfee4;
pub const XK_Pointer_UpRight: u32 = 0xfee5;
pub const XK_Pointer_DownLeft: u32 = 0xfee6;
pub const XK_Pointer_DownRight: u32 = 0xfee7;
pub const XK_Pointer_Button_Dflt: u32 = 0xfee8;
pub const XK_Pointer_Button1: u32 = 0xfee9;
pub const XK_Pointer_Button2: u32 = 0xfeea;
pub const XK_Pointer_Button3: u32 = 0xfeeb;
pub const XK_Pointer_Button4: u32 = 0xfeec;
pub const XK_Pointer_Button5: u32 = 0xfeed;
pub const XK_Pointer_DblClick_Dflt: u32 = 0xfeee;
pub const XK_Pointer_DblClick1: u32 = 0xfeef;
pub const XK_Pointer_DblClick2: u32 = 0xfef0;
pub const XK_Pointer_DblClick3: u32 = 0xfef1;
pub const XK_Pointer_DblClick4: u32 = 0xfef2;
pub const XK_Pointer_DblClick5: u32 = 0xfef3;
pub const XK_Pointer_Drag_Dflt: u32 = 0xfef4;
pub const XK_Pointer_Drag1: u32 = 0xfef5;
pub const XK_Pointer_Drag2: u32 = 0xfef6;
pub const XK_Pointer_Drag3: u32 = 0xfef7;
pub const XK_Pointer_Drag4: u32 = 0xfef8;
pub const XK_Pointer_Drag5: u32 = 0xfefd;

pub const XK_Pointer_EnableKeys: u32 = 0xfef9;
pub const XK_Pointer_Accelerate: u32 = 0xfefa;
pub const XK_Pointer_DfltBtnNext: u32 = 0xfefb;
pub const XK_Pointer_DfltBtnPrev: u32 = 0xfefc;

pub const XK_ch: u32 = 0xfea0;
pub const XK_Ch: u32 = 0xfea1;
pub const XK_CH: u32 = 0xfea2;
pub const XK_c_h: u32 = 0xfea3;
pub const XK_C_h: u32 = 0xfea4;
pub const XK_C_H: u32 = 0xfea5;

pub struct KeyboardUtils;

impl KeyboardUtils {
    pub fn get_keysym(event: web_sys::KeyboardEvent) -> u32 {
        // let ctrl = event.ctrl_key();
        let shift = event.shift_key();
        // let alt = event.alt_key();
        // let meta = event.meta_key();
        let location = event.location();
        let capslock = event.get_modifier_state("CapsLock");
        let upper = capslock ^ shift;
        let which = event.which();
        ConsoleService::log(&format!("which {}, shift {}", which, shift));
        match which {
            8_u32 => {
                // Backspace
                XK_BackSpace
            }
            9_u32 => {
                // Tab
                XK_Tab
            }
            13_u32 => {
                // Enter
                XK_Linefeed
            }
            16_u32 => {
                // ShiftLeft
                match location {
                    2_u32 => XK_Shift_R,
                    _ => XK_Shift_L,
                }
            }
            17_u32 => {
                // ControlLeft
                match location {
                    2_u32 => XK_Control_R,
                    _ => XK_Control_L,
                }
            }
            18_u32 => {
                // AltLeft
                match location {
                    2_u32 => XK_Alt_R,
                    _ => XK_Alt_L,
                }
            }
            19_u32 => {
                // Pause
                XK_Pause
            }
            20_u32 => {
                // CapsLock
                XK_Caps_Lock
            }
            27_u32 => {
                // Escape
                XK_Escape
            }
            32_u32 => {
                // Space
                XK_space
            }
            33_u32 => {
                // PageUp
                XK_Page_Up
            }
            34_u32 => {
                // PageDown
                XK_Page_Down
            }
            35_u32 => {
                // End
                XK_End
            }
            36_u32 => {
                // Home
                XK_Home
            }
            37_u32 => {
                // ArrowLeft
                XK_Left
            }
            38_u32 => {
                // ArrowUp
                XK_Up
            }
            39_u32 => {
                // ArrowRight
                XK_Right
            }
            40_u32 => {
                // ArrowDown
                XK_Down
            }
            44_u32 => {
                // PrintScreen
                XF86XK_ScreenSaver
            }
            45_u32 => {
                // Insert
                XK_Insert
            }
            46_u32 => {
                // Delete
                XK_Delete
            }
            48_u32 => {
                // Digit0
                if shift {
                    XK_parenright
                } else {
                    XK_0
                }
            }
            49_u32 => {
                // Digit1
                if shift {
                    XK_exclam
                } else {
                    XK_1
                }
            }
            50_u32 => {
                // Digit2
                if shift {
                    XK_at
                } else {
                    XK_2
                }
            }
            51_u32 => {
                // Digit3
                if shift {
                    XK_numbersign
                } else {
                    XK_3
                }
            }
            52_u32 => {
                // Digit4
                if shift {
                    XK_dollar
                } else {
                    XK_4
                }
            }
            53_u32 => {
                // Digit5
                if shift {
                    XK_percent
                } else {
                    XK_5
                }
            }
            54_u32 => {
                // Digit6
                if shift {
                    XK_asciicircum
                } else {
                    XK_6
                }
            }
            55_u32 => {
                // Digit7
                if shift {
                    XK_ampersand
                } else {
                    XK_7
                }
            }
            56_u32 => {
                // Digit8
                if shift {
                    XK_asterisk
                } else {
                    XK_8
                }
            }
            57_u32 => {
                // Digit9
                if shift {
                    XK_parenleft
                } else {
                    XK_9
                }
            }
            65_u32 => {
                // KeyA
                if upper {
                    XK_A
                } else {
                    XK_a
                }
            }
            66_u32 => {
                // KeyB
                if upper {
                    XK_B
                } else {
                    XK_b
                }
            }
            67_u32 => {
                // KeyC
                if upper {
                    XK_C
                } else {
                    XK_c
                }
            }
            68_u32 => {
                // KeyD
                if upper {
                    XK_D
                } else {
                    XK_d
                }
            }
            69_u32 => {
                // KeyE
                if upper {
                    XK_E
                } else {
                    XK_e
                }
            }
            70_u32 => {
                // KeyF
                if upper {
                    XK_F
                } else {
                    XK_f
                }
            }
            71_u32 => {
                // KeyG
                if upper {
                    XK_G
                } else {
                    XK_g
                }
            }
            72_u32 => {
                // KeyH
                if upper {
                    XK_H
                } else {
                    XK_h
                }
            }
            73_u32 => {
                // KeyI
                if upper {
                    XK_I
                } else {
                    XK_i
                }
            }
            74_u32 => {
                // KeyJ
                if upper {
                    XK_J
                } else {
                    XK_j
                }
            }
            75_u32 => {
                // KeyK
                if upper {
                    XK_K
                } else {
                    XK_k
                }
            }
            76_u32 => {
                // KeyL
                if upper {
                    XK_L
                } else {
                    XK_l
                }
            }
            77_u32 => {
                // KeyM
                if upper {
                    XK_M
                } else {
                    XK_m
                }
            }
            78_u32 => {
                // KeyN
                if upper {
                    XK_N
                } else {
                    XK_n
                }
            }
            79_u32 => {
                // KeyO
                if upper {
                    XK_O
                } else {
                    XK_o
                }
            }
            80_u32 => {
                // KeyP
                if upper {
                    XK_P
                } else {
                    XK_p
                }
            }
            81_u32 => {
                // KeyQ
                if upper {
                    XK_Q
                } else {
                    XK_q
                }
            }
            82_u32 => {
                // KeyR
                if upper {
                    XK_R
                } else {
                    XK_r
                }
            }
            83_u32 => {
                // KeyS
                if upper {
                    XK_S
                } else {
                    XK_s
                }
            }
            84_u32 => {
                // KeyT
                if upper {
                    XK_T
                } else {
                    XK_t
                }
            }
            85_u32 => {
                // KeyU
                if upper {
                    XK_U
                } else {
                    XK_u
                }
            }
            86_u32 => {
                // KeyV
                if upper {
                    XK_V
                } else {
                    XK_v
                }
            }
            87_u32 => {
                // KeyW
                if upper {
                    XK_W
                } else {
                    XK_w
                }
            }
            88_u32 => {
                // KeyX
                if upper {
                    XK_X
                } else {
                    XK_x
                }
            }
            89_u32 => {
                // KeyY
                if upper {
                    XK_Y
                } else {
                    XK_y
                }
            }
            90_u32 => {
                // KeyZ
                if upper {
                    XK_Z
                } else {
                    XK_z
                }
            }
            91_u32 => {
                // MetaLeft
                XK_Meta_L
            }
            92_u32 => {
                // MetaRight
                XK_Meta_R
            }
            93_u32 => {
                // ContextMenu
                XK_Menu
            }
            96_u32 => {
                // Numpad0
                XK_KP_0
            }
            97_u32 => {
                // Numpad1
                XK_KP_1
            }
            98_u32 => {
                // Numpad2
                XK_KP_2
            }
            99_u32 => {
                // Numpad3
                XK_KP_3
            }
            100_u32 => {
                // Numpad4
                XK_KP_4
            }
            101_u32 => {
                // Numpad5
                XK_KP_5
            }
            102_u32 => {
                // Numpad6
                XK_KP_6
            }
            103_u32 => {
                // Numpad7
                XK_KP_7
            }
            104_u32 => {
                // Numpad8
                XK_KP_8
            }
            105_u32 => {
                // Numpad9
                XK_KP_9
            }
            106_u32 => {
                // NumpadMultiply
                XK_KP_Multiply
            }
            107_u32 => {
                // NumpadAdd
                XK_KP_Add
            }
            109_u32 => {
                // NumpadSubtract
                XK_KP_Subtract
            }
            110_u32 => {
                // NumpadDecimal
                XK_KP_Decimal
            }
            111_u32 => {
                // NumpadDivide
                XK_KP_Divide
            }
            112_u32 => {
                // F1
                XK_F1
            }
            113_u32 => {
                // F2
                XK_F2
            }
            114_u32 => {
                // F3
                XK_F3
            }
            115_u32 => {
                // F4
                XK_F4
            }
            116_u32 => {
                // F5
                XK_F5
            }
            117_u32 => {
                // F6
                XK_F6
            }
            118_u32 => {
                // F7
                XK_F7
            }
            119_u32 => {
                // F8
                XK_F8
            }
            120_u32 => {
                // F9
                XK_F9
            }
            121_u32 => {
                // F10
                XK_F10
            }
            122_u32 => {
                // F11
                XK_F11
            }
            123_u32 => {
                // F12
                XK_F12
            }
            144_u32 => {
                // NumLock
                XK_Num_Lock
            }
            145_u32 => {
                // ScrollLock
                XK_Scroll_Lock
            }
            186_u32 => {
                // Semicolon
                if shift {
                    XK_colon
                } else {
                    XK_semicolon
                }
            }
            187_u32 => {
                // Equal
                if shift {
                    XK_plus
                } else {
                    XK_equal
                }
            }
            188_u32 => {
                // Comma
                if shift {
                    XK_less
                } else {
                    XK_comma
                }
            }
            189_u32 => {
                // Minus
                if shift {
                    XK_underscore
                } else {
                    XK_minus
                }
            }
            190_u32 => {
                // Period
                if shift {
                    XK_greater
                } else {
                    XK_period
                }
            }
            191_u32 => {
                // Slash
                if shift {
                    XK_question
                } else {
                    XK_slash
                }
            }
            192_u32 => {
                // Backquote
                if shift {
                    XK_asciitilde
                } else {
                    XK_grave // also quote left
                }
            }
            219_u32 => {
                // BracketLeft
                if shift {
                    XK_braceleft
                } else {
                    XK_bracketleft
                }
            }
            220_u32 => {
                // Backslash
                if shift {
                    XK_bar
                } else {
                    XK_backslash
                }
            }
            221_u32 => {
                // BracketRight
                if shift {
                    XK_braceright
                } else {
                    XK_bracketright
                }
            }
            222_u32 => {
                // Quote
                if shift {
                    XK_quoteright
                } else {
                    XK_quotedbl
                }
            }
            _ => which,
        }
    }
}
