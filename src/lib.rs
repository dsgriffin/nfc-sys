#![allow(non_camel_case_types, non_snake_case)]
//! Raw Rust FFI bindings for libnfc.
//!
//! This crate follows the usual `*-sys` crate pattern: it exposes libnfc's C API
//! with minimal translation and does not manage ownership, lifetimes, or error
//! handling for you. Most useful calls are `unsafe` because callers must satisfy
//! the same requirements as the corresponding native libnfc functions.
//!
//! The `nfc_context`, `nfc_device`, and `nfc_driver` types are opaque because
//! libnfc treats them as incomplete public types. Use the accessor and lifecycle
//! functions exported by libnfc instead of relying on their internal layout.
//!
//! # Linking
//!
//! The build script links against a system-provided `libnfc`. On macOS it checks
//! common Homebrew locations. For custom installations, set `LIBNFC_LIB_DIR` to
//! the directory containing the native library.
//!
//! # Example
//!
//! ```no_run
//! use std::ffi::CStr;
//! use std::ptr;
//!
//! use nfc_sys::{nfc_exit, nfc_init, nfc_version};
//!
//! unsafe {
//!     let mut context = ptr::null_mut();
//!     nfc_init(&mut context);
//!
//!     if !context.is_null() {
//!         let version = CStr::from_ptr(nfc_version()).to_string_lossy();
//!         println!("libnfc version: {}", version);
//!
//!         nfc_exit(context);
//!     }
//! }
//! ```

extern crate libc;

use libc::{c_char, c_int, c_void, size_t};

pub const NFC_BUFSIZE_CONNSTRING: usize = 1024;

pub const NFC_SUCCESS: c_int = 0;
pub const NFC_EIO: c_int = -1;
pub const NFC_EINVARG: c_int = -2;
pub const NFC_EDEVNOTSUPP: c_int = -3;
pub const NFC_ENOTSUCHDEV: c_int = -4;
pub const NFC_EOVFLOW: c_int = -5;
pub const NFC_ETIMEOUT: c_int = -6;
pub const NFC_EOPABORTED: c_int = -7;
pub const NFC_ENOTIMPL: c_int = -8;
pub const NFC_ETGRELEASED: c_int = -10;
pub const NFC_ERFTRANS: c_int = -20;
pub const NFC_EMFCAUTHFAIL: c_int = -30;
pub const NFC_ESOFT: c_int = -80;
pub const NFC_ECHIP: c_int = -90;

/// Opaque libnfc context handle.
#[repr(C)]
pub struct nfc_context {
    _private: [u8; 0],
}

/// Opaque libnfc device handle.
#[repr(C)]
pub struct nfc_device {
    _private: [u8; 0],
}

/// Opaque libnfc driver handle.
#[repr(C)]
pub struct nfc_driver {
    _private: [u8; 0],
}

pub type nfc_connstring = [c_char; NFC_BUFSIZE_CONNSTRING];

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfc_property {
    NP_TIMEOUT_COMMAND = 0,
    NP_TIMEOUT_ATR = 1,
    NP_TIMEOUT_COM = 2,
    NP_HANDLE_CRC = 3,
    NP_HANDLE_PARITY = 4,
    NP_ACTIVATE_FIELD = 5,
    NP_ACTIVATE_CRYPTO1 = 6,
    NP_INFINITE_SELECT = 7,
    NP_ACCEPT_INVALID_FRAMES = 8,
    NP_ACCEPT_MULTIPLE_FRAMES = 9,
    NP_AUTO_ISO14443_4 = 10,
    NP_EASY_FRAMING = 11,
    NP_FORCE_ISO14443_A = 12,
    NP_FORCE_ISO14443_B = 13,
    NP_FORCE_SPEED_106 = 14,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfc_dep_mode {
    NDM_UNDEFINED = 0,
    NDM_PASSIVE = 1,
    NDM_ACTIVE = 2,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_dep_info {
    pub abtNFCID3: [u8; 10],
    pub btDID: u8,
    pub btBS: u8,
    pub btBR: u8,
    pub btTO: u8,
    pub btPP: u8,
    pub abtGB: [u8; 48],
    pub szGB: size_t,
    pub ndm: nfc_dep_mode,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443a_info {
    pub abtAtqa: [u8; 2],
    pub btSak: u8,
    pub szUidLen: size_t,
    pub abtUid: [u8; 10],
    pub szAtsLen: size_t,
    pub abtAts: [u8; 254],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_felica_info {
    pub szLen: size_t,
    pub btResCode: u8,
    pub abtId: [u8; 8],
    pub abtPad: [u8; 8],
    pub abtSysCode: [u8; 2],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443b_info {
    pub abtPupi: [u8; 4],
    pub abtApplicationData: [u8; 4],
    pub abtProtocolInfo: [u8; 3],
    pub ui8CardIdentifier: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443bi_info {
    pub abtDIV: [u8; 4],
    pub btVerLog: u8,
    pub btConfig: u8,
    pub szAtrLen: size_t,
    pub abtAtr: [u8; 33],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443biclass_info {
    pub abtUID: [u8; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443b2sr_info {
    pub abtUID: [u8; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443b2ct_info {
    pub abtUID: [u8; 4],
    pub btProdCode: u8,
    pub btFabCode: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_jewel_info {
    pub btSensRes: [u8; 2],
    pub btId: [u8; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_barcode_info {
    pub szDataLen: size_t,
    pub abtData: [u8; 32],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union nfc_target_info {
    pub nai: nfc_iso14443a_info,
    pub nfi: nfc_felica_info,
    pub nbi: nfc_iso14443b_info,
    pub nii: nfc_iso14443bi_info,
    pub nsi: nfc_iso14443b2sr_info,
    pub nci: nfc_iso14443b2ct_info,
    pub nji: nfc_jewel_info,
    pub ndi: nfc_dep_info,
    pub nti: nfc_barcode_info,
    pub nhi: nfc_iso14443biclass_info,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfc_baud_rate {
    NBR_UNDEFINED = 0,
    NBR_106 = 1,
    NBR_212 = 2,
    NBR_424 = 3,
    NBR_847 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfc_modulation_type {
    NMT_ISO14443A = 1,
    NMT_JEWEL = 2,
    NMT_ISO14443B = 3,
    NMT_ISO14443BI = 4,
    NMT_ISO14443B2SR = 5,
    NMT_ISO14443B2CT = 6,
    NMT_FELICA = 7,
    NMT_DEP = 8,
    NMT_BARCODE = 9,
    NMT_ISO14443BICLASS = 10,
}

pub const NMT_END_ENUM: nfc_modulation_type = nfc_modulation_type::NMT_ISO14443BICLASS;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfc_mode {
    N_TARGET = 0,
    N_INITIATOR = 1,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_modulation {
    pub nmt: nfc_modulation_type,
    pub nbr: nfc_baud_rate,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_target {
    pub nti: nfc_target_info,
    pub nm: nfc_modulation,
}

#[repr(C)]
pub struct nfc_emulator {
    pub target: *mut nfc_target,
    pub state_machine: *mut nfc_emulation_state_machine,
    pub user_data: *mut c_void,
}

#[repr(C)]
pub struct nfc_emulation_state_machine {
    pub io: Option<
        unsafe extern "C" fn(
            emulator: *mut nfc_emulator,
            data_in: *const u8,
            data_in_len: size_t,
            data_out: *mut u8,
            data_out_len: size_t,
        ) -> c_int,
    >,
    pub data: *mut c_void,
}

#[link(name = "nfc")]
extern "C" {
    pub fn nfc_init(context: *mut *mut nfc_context);
    pub fn nfc_exit(context: *mut nfc_context);
    pub fn nfc_register_driver(driver: *const nfc_driver) -> c_int;

    pub fn nfc_open(context: *mut nfc_context, connstring: *const c_char) -> *mut nfc_device;
    pub fn nfc_close(pnd: *mut nfc_device);
    pub fn nfc_abort_command(pnd: *mut nfc_device) -> c_int;
    pub fn nfc_list_devices(
        context: *mut nfc_context,
        connstrings: *mut nfc_connstring,
        connstrings_len: size_t,
    ) -> size_t;
    pub fn nfc_idle(pnd: *mut nfc_device) -> c_int;

    pub fn nfc_initiator_init(pnd: *mut nfc_device) -> c_int;
    pub fn nfc_initiator_init_secure_element(pnd: *mut nfc_device) -> c_int;
    pub fn nfc_initiator_select_passive_target(
        pnd: *mut nfc_device,
        nm: nfc_modulation,
        pbtInitData: *const u8,
        szInitData: size_t,
        pnt: *mut nfc_target,
    ) -> c_int;
    pub fn nfc_initiator_list_passive_targets(
        pnd: *mut nfc_device,
        nm: nfc_modulation,
        ant: *mut nfc_target,
        szTargets: size_t,
    ) -> c_int;
    pub fn nfc_initiator_poll_target(
        pnd: *mut nfc_device,
        pnmTargetTypes: *const nfc_modulation,
        szTargetTypes: size_t,
        uiPollNr: u8,
        uiPeriod: u8,
        pnt: *mut nfc_target,
    ) -> c_int;
    pub fn nfc_initiator_select_dep_target(
        pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_initiator_poll_dep_target(
        pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_initiator_deselect_target(pnd: *mut nfc_device) -> c_int;
    pub fn nfc_initiator_transceive_bytes(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: size_t,
        pbtRx: *mut u8,
        szRx: size_t,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_initiator_transceive_bits(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: size_t,
        pbtTxPar: *const u8,
        pbtRx: *mut u8,
        szRx: size_t,
        pbtRxPar: *mut u8,
    ) -> c_int;
    pub fn nfc_initiator_transceive_bytes_timed(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: size_t,
        pbtRx: *mut u8,
        szRx: size_t,
        cycles: *mut u32,
    ) -> c_int;
    pub fn nfc_initiator_transceive_bits_timed(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: size_t,
        pbtTxPar: *const u8,
        pbtRx: *mut u8,
        szRx: size_t,
        pbtRxPar: *mut u8,
        cycles: *mut u32,
    ) -> c_int;
    pub fn nfc_initiator_target_is_present(pnd: *mut nfc_device, pnt: *const nfc_target) -> c_int;

    pub fn nfc_target_init(
        pnd: *mut nfc_device,
        pnt: *mut nfc_target,
        pbtRx: *mut u8,
        szRx: size_t,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_target_send_bytes(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: size_t,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_target_receive_bytes(
        pnd: *mut nfc_device,
        pbtRx: *mut u8,
        szRx: size_t,
        timeout: c_int,
    ) -> c_int;
    pub fn nfc_target_send_bits(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: size_t,
        pbtTxPar: *const u8,
    ) -> c_int;
    pub fn nfc_target_receive_bits(
        pnd: *mut nfc_device,
        pbtRx: *mut u8,
        szRx: size_t,
        pbtRxPar: *mut u8,
    ) -> c_int;

    pub fn nfc_strerror(pnd: *const nfc_device) -> *const c_char;
    pub fn nfc_strerror_r(pnd: *const nfc_device, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn nfc_perror(pnd: *const nfc_device, s: *const c_char);
    pub fn nfc_device_get_last_error(pnd: *const nfc_device) -> c_int;

    pub fn nfc_device_get_name(pnd: *mut nfc_device) -> *const c_char;
    pub fn nfc_device_get_connstring(pnd: *mut nfc_device) -> *const c_char;
    pub fn nfc_device_get_supported_modulation(
        pnd: *mut nfc_device,
        mode: nfc_mode,
        supported_mt: *mut *const nfc_modulation_type,
    ) -> c_int;
    pub fn nfc_device_get_supported_baud_rate(
        pnd: *mut nfc_device,
        nmt: nfc_modulation_type,
        supported_br: *mut *const nfc_baud_rate,
    ) -> c_int;
    pub fn nfc_device_get_supported_baud_rate_target_mode(
        pnd: *mut nfc_device,
        nmt: nfc_modulation_type,
        supported_br: *mut *const nfc_baud_rate,
    ) -> c_int;

    pub fn nfc_device_set_property_int(
        pnd: *mut nfc_device,
        property: nfc_property,
        value: c_int,
    ) -> c_int;
    pub fn nfc_device_set_property_bool(
        pnd: *mut nfc_device,
        property: nfc_property,
        bEnable: bool,
    ) -> c_int;

    pub fn iso14443a_crc(pbtData: *mut u8, szLen: size_t, pbtCrc: *mut u8);
    pub fn iso14443a_crc_append(pbtData: *mut u8, szLen: size_t);
    pub fn iso14443b_crc(pbtData: *mut u8, szLen: size_t, pbtCrc: *mut u8);
    pub fn iso14443b_crc_append(pbtData: *mut u8, szLen: size_t);
    pub fn iso14443a_locate_historical_bytes(
        pbtAts: *mut u8,
        szAts: size_t,
        pszTk: *mut size_t,
    ) -> *mut u8;

    pub fn nfc_free(p: *mut c_void);
    pub fn nfc_version() -> *const c_char;
    pub fn nfc_device_get_information_about(pnd: *mut nfc_device, buf: *mut *mut c_char) -> c_int;

    pub fn str_nfc_modulation_type(nmt: nfc_modulation_type) -> *const c_char;
    pub fn str_nfc_baud_rate(nbr: nfc_baud_rate) -> *const c_char;
    pub fn str_nfc_target(buf: *mut *mut c_char, pnt: *const nfc_target, verbose: bool) -> c_int;

    pub fn nfc_emulate_target(
        pnd: *mut nfc_device,
        emulator: *mut nfc_emulator,
        timeout: c_int,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_call_libnfc_version() {
        let version = unsafe { nfc_version() };

        assert!(!version.is_null());
    }

    #[test]
    fn target_layout_matches_libnfc_1_8_on_64_bit() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<nfc_target_info>(), 283);
            assert_eq!(std::mem::size_of::<nfc_modulation>(), 8);
            assert_eq!(std::mem::size_of::<nfc_target>(), 291);
            assert_eq!(std::mem::align_of::<nfc_target_info>(), 1);
            assert_eq!(std::mem::align_of::<nfc_target>(), 1);
        }
    }
}
