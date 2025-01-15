use windows::{
    core::GUID,
    Win32::{
        Foundation::HWND,
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Shell::{ITaskbarList3, TBPF_NOPROGRESS, TBPF_NORMAL},
    },
};

// Manually define the CLSID for TaskbarList (56FDF344-FD6D-11D0-958A-006097C9A090)
const CLSID_TASKBARLIST: GUID = GUID::from_u128(0x56fdf344_fd6d_11d0_958a_006097c9a090);

fn win_set_taskbar_progress(
    hwnd: HWND,
    progress: u64,
    max_progress: u64,
) -> windows::core::Result<()> {
    unsafe {
        // Create an instance of ITaskbarList3 using CoCreateInstance
        let taskbar: ITaskbarList3 =
            CoCreateInstance(&CLSID_TASKBARLIST, None, CLSCTX_INPROC_SERVER)?;

        // Ensure taskbar is initialized
        taskbar.HrInit()?;

        // Set the progress state and progress value
        taskbar.SetProgressState(hwnd, TBPF_NORMAL)?;
        taskbar.SetProgressValue(hwnd, progress, max_progress)?;
    }

    Ok(())
}

fn win_clear_taskbar_progress(hwnd: HWND) -> windows::core::Result<()> {
    unsafe {
        // Create an instance of ITaskbarList3 using CoCreateInstance
        let taskbar: ITaskbarList3 =
            CoCreateInstance(&CLSID_TASKBARLIST, None, CLSCTX_INPROC_SERVER)?;

        // Clear the taskbar progress
        taskbar.SetProgressState(hwnd, TBPF_NOPROGRESS)?;
    }

    Ok(())
}

/// Update the taskbar progress bar
/// `window` - The window to update the progress bar for
/// `progress` - The progress value (0.0 to 1.0)
#[tauri::command]
pub fn update_taskbar_progress(window: tauri::Window, progress: f32) {
    static MAX: u64 = 0xFFFF;
    let hwnd = windows::Win32::Foundation::HWND(window.hwnd().unwrap().0);
    let factor = (progress * (MAX as f32)) as u64;
    win_set_taskbar_progress(hwnd, factor.max(1), MAX).unwrap();
}

/// Clear the taskbar progress bar
/// `window` - The window to clear the progress bar for
#[tauri::command]
pub fn clear_taskbar_progress(window: tauri::Window) {
    let hwnd = windows::Win32::Foundation::HWND(window.hwnd().unwrap().0);
    win_clear_taskbar_progress(hwnd).unwrap();
}
