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

fn open_wlan_handle(api_version: u32) -> Result<HANDLE, windows::core::Error> {
    let mut negotiated_vesion:i32 = 0;
    let mut wlan_handle = INVALID_HANDLE_VALUE;

}

fn main() {
    println!("Hello, world!");
}
