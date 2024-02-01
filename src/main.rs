use std::os::windows;
use ::windows::core::Interface;
use std ::{ffi::OsString, os::windows::ffi::OsStringExt}; //std são standard do Rust e não precisam ser includias no cargo.toml
use windows::{
    core::{GUID,HSTRING,PCWSTR,PWSTR},
    Win32::{
        Foundation::{ERROR_SUCCESS, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
        NetworkManagement::WiFi::{
            WlanCloseHandle, WLanEnumInterfaces, WlanFreeMemory, WlanGetProfile,
            WlanGetProfileList, WlanOpenHandle, WLAN_API_VESION_2_0, WLAN_INTERFACE_INFO,
            WLAN_PROFILE_GET_KEY, WLAN_PROFILE_INFO_LIST,
        },
    },
};

fn open_wlan_handle(api_version: u32) -> Result<HANDLE, windows::core::Error> { //Abre uma conexão com o sistema de rede wireless, 
    let mut negotiated_vesion:i32 = 0;
    let mut wlan_handle = INVALID_HANDLE_VALUE;
    let result:() = unsafe {WlanOpenHandle(api_version, None, &mut negotiated_vesion, &mut wlan_handle)};

    WIN32_ERROR(result).ok()?;
    Ok(wlan_handle)
    }

fn enum_wlan_interfaces(handle :HANDLE) -> Result<*mut WLAN_INTERFACE_INFO_LIST, windows::core::Error>{ //Enumera as redes disponiveis
    let mut interface_ptr = std::ptr::null_mut();
    let result = unsafe { WLanEnumInterfaces(handle, None, &mut interface_ptr)};

    WIN32_ERROR(result).ok()?;
    Ok(interface_ptr)

}
fn grab_interface_profiles( //Pega ouma lista dos profile names de redes wireless no sistema pelo GUID
    handle :HANDLE,
    interface_guid: &GUID
)-> Result<*const WLAN_PROFILE_INFO_LIST, windows::core::Error>{
    let mut wlan_profiles_ptr = std::ptr::null_mut();
    let result = unsafe {WlanGetProfileList (handle, interface_guid, None, &mut wlan_profiles_ptr)};
    WIN32_ERROR(result).ok()?;

    Ok(wlan_profiles_ptr)

}

fn parse_utf16_slice(string_slice : &[u16] ) -> Option<OsString>{
    let null_index = string_slice.iter().position(|c| c == &0 )?;

    Some(OsString::from_wide(&string_slice[..null_index]))
}



fn main() {

    let wlan_handle = open_wlan_handle(WLAN_API_VESION_2_0).expect ("Failed to open WLAN handle!");
    let interface_ptr = match enum_wlan_interfaces(wlan_handle){
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Falha ao adquirir as interfaces wireless: {:?", e);
            unsafe {
                WlanCloseHandle(wlan_handle);
            }
        }
    };

    println!("Hello, world!");
}
