pub mod defs;
pub mod errors;

use crate::defs::*;
use crate::errors::TagError;
use windows::{core::PCWSTR, Win32::Foundation::*};

#[link(name = "apphelp", kind = "static")]
extern "C" {
    pub fn SdbCreateDatabase(pwszPath: PCWSTR, eType: PATH_TYPE) -> PDB;
    pub fn SdbBeginWriteListTag(pdb: PDB, tTag: TAG) -> TAGID;
    pub fn SdbWriteStringTag(pdb: PDB, tTag: TAG, pwszData: PCWSTR) -> BOOL;
    pub fn SdbWriteBinaryTag(pdb: PDB, tTag: TAG, pBuffer: PBYTE, dwSize: DWORD) -> BOOL;
    pub fn SdbWriteWORDTag(pdb: PDB, tTag: TAG, wData: WORD) -> BOOL;
    pub fn SdbWriteDWORDTag(pdb: PDB, tTag: TAG, qwData: DWORD) -> BOOL;
    pub fn SdbWriteQWORDTag(pdb: PDB, tTag: TAG, qwData: QWORD) -> BOOL;
    pub fn SdbWriteNULLTag(pdb: PDB, tTag: TAG) -> BOOL;
    pub fn SdbEndWriteListTag(pdb: PDB, tiList: TAGID) -> TAGID;
    pub fn SdbCloseDatabaseWrite(pdb: PDB);
}

pub fn get_pwstr(input: &str) -> PCWSTR {
    // Convert to UTF-16, ensure NULL byte at the end
    let mut buff: Vec<u16> = input.encode_utf16().collect();
    buff.push(0);
    PCWSTR(buff.as_mut_ptr())
}

pub fn get_tag(input: &str) -> Result<DWORD, TagError> {
    match input {
        // Tags
        "TAGID_NULL" => Ok(TAGID_NULL),

        // TAG_TYPE_NULL
        "TAG_TYPE_NULL" => Ok(TAG_TYPE_NULL),
        "TAG_INCLUDE" => Ok(TAG_INCLUDE),
        "TAG_GENERAL" => Ok(TAG_GENERAL),
        "TAG_MATCH_LOGIC_NOT" => Ok(TAG_MATCH_LOGIC_NOT),
        "TAG_APPLY_ALL_SHIMS" => Ok(TAG_APPLY_ALL_SHIMS),
        "TAG_USE_SERVICE_PACK_FILES" => Ok(TAG_USE_SERVICE_PACK_FILES),
        "TAG_MITIGATION_OS" => Ok(TAG_MITIGATION_OS),
        "TAG_BLOCK_UPGRADE" => Ok(TAG_BLOCK_UPGRADE),
        "TAG_INCLUDEEXCLUDEDLL" => Ok(TAG_INCLUDEEXCLUDEDLL),

        // TAG_TYPE_WORD
        "TAG_TYPE_WORD" => Ok(TAG_TYPE_WORD),
        "TAG_MATCH_MODE" => Ok(TAG_MATCH_MODE),
        "TAG_TAG" => Ok(TAG_TAG),
        "TAG_INDEX_TAG" => Ok(TAG_INDEX_TAG),
        "TAG_INDEX_KEY" => Ok(TAG_INDEX_KEY),

        // TAG_TYPE_DWORD
        "TAG_TYPE_DWORD" => Ok(TAG_TYPE_DWORD),
        "TAG_SIZE" => Ok(TAG_SIZE),
        "TAG_OFFSET" => Ok(TAG_OFFSET),
        "TAG_CHECKSUM" => Ok(TAG_CHECKSUM),
        "TAG_SHIM_TAGID" => Ok(TAG_SHIM_TAGID),
        "TAG_PATCH_TAGID" => Ok(TAG_PATCH_TAGID),
        "TAG_MODULE_TYPE" => Ok(TAG_MODULE_TYPE),
        "TAG_VERDATEHI" => Ok(TAG_VERDATEHI),
        "TAG_VERDATELO" => Ok(TAG_VERDATELO),
        "TAG_VERFILEOS" => Ok(TAG_VERFILEOS),
        "TAG_VERFILETYPE" => Ok(TAG_VERFILETYPE),
        "TAG_PE_CHECKSUM" => Ok(TAG_PE_CHECKSUM),
        "TAG_PREVOSMAJORVER" => Ok(TAG_PREVOSMAJORVER),
        "TAG_PREVOSMINORVER" => Ok(TAG_PREVOSMINORVER),
        "TAG_PREVOSPLATFORMID" => Ok(TAG_PREVOSPLATFORMID),
        "TAG_PREVOSBUILDNO" => Ok(TAG_PREVOSBUILDNO),
        "TAG_PROBLEMSEVERITY" => Ok(TAG_PROBLEMSEVERITY),
        "TAG_LANGID" => Ok(TAG_LANGID),
        "TAG_VER_LANGUAGE" => Ok(TAG_VER_LANGUAGE),
        "TAG_ENGINE" => Ok(TAG_ENGINE),
        "TAG_HTMLHELPID" => Ok(TAG_HTMLHELPID),
        "TAG_INDEX_FLAGS" => Ok(TAG_INDEX_FLAGS),
        "TAG_FLAGS" => Ok(TAG_FLAGS),
        "TAG_DATA_VALUETYPE" => Ok(TAG_DATA_VALUETYPE),
        "TAG_DATA_DWORD" => Ok(TAG_DATA_DWORD),
        "TAG_LAYER_TAGID" => Ok(TAG_LAYER_TAGID),
        "TAG_MSI_TRANSFORM_TAGID" => Ok(TAG_MSI_TRANSFORM_TAGID),
        "TAG_LINKER_VERSION" => Ok(TAG_LINKER_VERSION),
        "TAG_LINK_DATE" => Ok(TAG_LINK_DATE),
        "TAG_UPTO_LINK_DATE" => Ok(TAG_UPTO_LINK_DATE),
        "TAG_OS_SERVICE_PACK" => Ok(TAG_OS_SERVICE_PACK),
        "TAG_FLAG_TAGID" => Ok(TAG_FLAG_TAGID),
        "TAG_RUNTIME_PLATFORM" => Ok(TAG_RUNTIME_PLATFORM),
        "TAG_OS_SKU" => Ok(TAG_OS_SKU),
        "TAG_OS_PLATFORM" => Ok(TAG_OS_PLATFORM),
        "TAG_APP_NAME_RC_ID" => Ok(TAG_APP_NAME_RC_ID),
        "TAG_VENDOR_NAME_RC_ID" => Ok(TAG_VENDOR_NAME_RC_ID),
        "TAG_SUMMARY_MSG_RC_ID" => Ok(TAG_SUMMARY_MSG_RC_ID),
        "TAG_VISTA_SKU" => Ok(TAG_VISTA_SKU),
        "TAG_DESCRIPTION_RC_ID" => Ok(TAG_DESCRIPTION_RC_ID),
        "TAG_PARAMETER1_RC_ID" => Ok(TAG_PARAMETER1_RC_ID),
        "TAG_TAGID" => Ok(TAG_TAGID),

        // TAG_TYPE_QWORD
        "TAG_TYPE_QWORD" => Ok(TAG_TYPE_QWORD),
        "TAG_TIME" => Ok(TAG_TIME),
        "TAG_BIN_FILE_VERSION" => Ok(TAG_BIN_FILE_VERSION),
        "TAG_BIN_PRODUCT_VERSION" => Ok(TAG_BIN_PRODUCT_VERSION),
        "TAG_MODTIME" => Ok(TAG_MODTIME),
        "TAG_FLAG_MASK_KERNEL" => Ok(TAG_FLAG_MASK_KERNEL),
        "TAG_UPTO_BIN_PRODUCT_VERSION" => Ok(TAG_UPTO_BIN_PRODUCT_VERSION),
        "TAG_DATA_QWORD" => Ok(TAG_DATA_QWORD),
        "TAG_FLAG_MASK_USER" => Ok(TAG_FLAG_MASK_USER),
        "TAG_FLAGS_NTVDM1" => Ok(TAG_FLAGS_NTVDM1),
        "TAG_FLAGS_NTVDM2" => Ok(TAG_FLAGS_NTVDM2),
        "TAG_FLAGS_NTVDM3" => Ok(TAG_FLAGS_NTVDM3),
        "TAG_FLAG_MASK_SHELL" => Ok(TAG_FLAG_MASK_SHELL),
        "TAG_UPTO_BIN_FILE_VERSION" => Ok(TAG_UPTO_BIN_FILE_VERSION),
        "TAG_FLAG_MASK_FUSION" => Ok(TAG_FLAG_MASK_FUSION),
        "TAG_FLAG_PROCESSPARAM" => Ok(TAG_FLAG_PROCESSPARAM),
        "TAG_FLAG_LUA" => Ok(TAG_FLAG_LUA),
        "TAG_FLAG_INSTALL" => Ok(TAG_FLAG_INSTALL),

        // TAG_TYPE_STRINGREF
        "TAG_TYPE_STRINGREF" => Ok(TAG_TYPE_STRINGREF),
        "TAG_NAME" => Ok(TAG_NAME),
        "TAG_DESCRIPTION" => Ok(TAG_DESCRIPTION),
        "TAG_MODULE" => Ok(TAG_MODULE),
        "TAG_API" => Ok(TAG_API),
        "TAG_VENDOR" => Ok(TAG_VENDOR),
        "TAG_APP_NAME" => Ok(TAG_APP_NAME),
        "TAG_COMMAND_LINE" => Ok(TAG_COMMAND_LINE),
        "TAG_COMPANY_NAME" => Ok(TAG_COMPANY_NAME),
        "TAG_DLLFILE" => Ok(TAG_DLLFILE),
        "TAG_WILDCARD_NAME" => Ok(TAG_WILDCARD_NAME),
        "TAG_PRODUCT_NAME" => Ok(TAG_PRODUCT_NAME),
        "TAG_PRODUCT_VERSION" => Ok(TAG_PRODUCT_VERSION),
        "TAG_FILE_DESCRIPTION" => Ok(TAG_FILE_DESCRIPTION),
        "TAG_FILE_VERSION" => Ok(TAG_FILE_VERSION),
        "TAG_ORIGINAL_FILENAME" => Ok(TAG_ORIGINAL_FILENAME),
        "TAG_INTERNAL_NAME" => Ok(TAG_INTERNAL_NAME),
        "TAG_LEGAL_COPYRIGHT" => Ok(TAG_LEGAL_COPYRIGHT),
        "TAG_16BIT_DESCRIPTION" => Ok(TAG_16BIT_DESCRIPTION),
        "TAG_APPHELP_DETAILS" => Ok(TAG_APPHELP_DETAILS),
        "TAG_LINK_URL" => Ok(TAG_LINK_URL),
        "TAG_LINK_TEXT" => Ok(TAG_LINK_TEXT),
        "TAG_APPHELP_TITLE" => Ok(TAG_APPHELP_TITLE),
        "TAG_APPHELP_CONTACT" => Ok(TAG_APPHELP_CONTACT),
        "TAG_SXS_MANIFEST" => Ok(TAG_SXS_MANIFEST),
        "TAG_DATA_STRING" => Ok(TAG_DATA_STRING),
        "TAG_MSI_TRANSFORM_FILE" => Ok(TAG_MSI_TRANSFORM_FILE),
        "TAG_16BIT_MODULE_NAME" => Ok(TAG_16BIT_MODULE_NAME),
        "TAG_LAYER_DISPLAYNAME" => Ok(TAG_LAYER_DISPLAYNAME),
        "TAG_COMPILER_VERSION" => Ok(TAG_COMPILER_VERSION),
        "TAG_ACTION_TYPE" => Ok(TAG_ACTION_TYPE),
        "TAG_EXPORT_NAME" => Ok(TAG_EXPORT_NAME),

        // TAG_TYPE_LIST
        "TAG_TYPE_LIST" => Ok(TAG_TYPE_LIST),
        "TAG_DATABASE" => Ok(TAG_DATABASE),
        "TAG_LIBRARY" => Ok(TAG_LIBRARY),
        "TAG_INEXCLUDE" => Ok(TAG_INEXCLUDE),
        "TAG_SHIM" => Ok(TAG_SHIM),
        "TAG_PATCH" => Ok(TAG_PATCH),
        "TAG_APP" => Ok(TAG_APP),
        "TAG_EXE" => Ok(TAG_EXE),
        "TAG_MATCHING_FILE" => Ok(TAG_MATCHING_FILE),
        "TAG_SHIM_REF" => Ok(TAG_SHIM_REF),
        "TAG_PATCH_REF" => Ok(TAG_PATCH_REF),
        "TAG_LAYER" => Ok(TAG_LAYER),
        "TAG_FILE" => Ok(TAG_FILE),
        "TAG_APPHELP" => Ok(TAG_APPHELP),
        "TAG_LINK" => Ok(TAG_LINK),
        "TAG_DATA" => Ok(TAG_DATA),
        "TAG_MSI_TRANSFORM" => Ok(TAG_MSI_TRANSFORM),
        "TAG_MSI_TRANSFORM_REF" => Ok(TAG_MSI_TRANSFORM_REF),
        "TAG_MSI_PACKAGE" => Ok(TAG_MSI_PACKAGE),
        "TAG_FLAG" => Ok(TAG_FLAG),
        "TAG_MSI_CUSTOM_ACTION" => Ok(TAG_MSI_CUSTOM_ACTION),
        "TAG_FLAG_REF" => Ok(TAG_FLAG_REF),
        "TAG_ACTION" => Ok(TAG_ACTION),
        "TAG_LOOKUP" => Ok(TAG_LOOKUP),
        "TAG_STRINGTABLE" => Ok(TAG_STRINGTABLE),
        "TAG_INDEXES" => Ok(TAG_INDEXES),
        "TAG_INDEX" => Ok(TAG_INDEX),

        // TAG_TYPE_STRING
        "TAG_TYPE_STRING" => Ok(TAG_TYPE_STRING),
        "TAG_STRINGTABLE_ITEM" => Ok(TAG_STRINGTABLE_ITEM),

        // TAG_TYPE_BINARY
        "TAG_TYPE_BINARY" => Ok(TAG_TYPE_BINARY),
        "TAG_PATCH_BITS" => Ok(TAG_PATCH_BITS),
        "TAG_FILE_BITS" => Ok(TAG_FILE_BITS),
        "TAG_EXE_ID" => Ok(TAG_EXE_ID),
        "TAG_DATA_BITS" => Ok(TAG_DATA_BITS),
        "TAG_MSI_PACKAGE_ID" => Ok(TAG_MSI_PACKAGE_ID),
        "TAG_DATABASE_ID" => Ok(TAG_DATABASE_ID),
        "TAG_FIX_ID" => Ok(TAG_FIX_ID),
        "TAG_INDEX_BITS" => Ok(TAG_INDEX_BITS),
        _ => Err(TagError::new(input)),
    }
}
