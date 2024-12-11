
use sysinfo::{Disks, System};


/// 获取本机CPU核数
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::total_num_cpus(&mut sys), Ok(16));
/// ```
pub fn total_num_cpus(sys: &mut System) -> Result<usize, String> {
    sys.refresh_cpu_all();
    let cpus = sys.cpus();
    if cpus.is_empty() {
        Err("无法获取 CPU 信息，可能系统资源不足或数据未初始化".to_string())
    } else {
        Ok(cpus.len())
    }
}


/// 获取本机可用核数
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::used_num_cpus(&mut sys), Ok(16));
/// ```
pub fn availabe_num_cpus(sys: &mut System) -> Result<usize, String> {
    sys.refresh_cpu_all();
    let cpus = sys.cpus();
    if cpus.is_empty() {
        return Err("无法获取 CPU 信息，可能系统资源不足或数据未初始化".to_string());
    }

    let allocatable_cpus = cpus
            .iter()
            .filter(|cpu| cpu.cpu_usage() <= 95.0) // CPU 使用率小于等于 95% 时可用
        .count();

    Ok(allocatable_cpus)
}


/// 获取本机内存总量，单位字节
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::total_memory(&mut sys), Ok(16));
/// ```
pub fn total_memory(sys: &mut System) -> Result<u64, String> {
    sys.refresh_memory();
    let total_memory = sys.total_memory();
    if total_memory == 0 {
        Err("无法获取总内存信息，可能系统资源不足或数据未初始化".to_string())
    } else {
        Ok(total_memory)
    }
}

/// 获取本机内存可用量，单位字节
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::available_memory(&mut sys), Ok(7233028096));
/// ```
pub fn available_memory(sys: &mut System) -> Result<u64, String> {
    sys.refresh_memory(); // 刷新内存数据
    let available_memory = sys.available_memory(); // 获取可用内存

    if available_memory == 0 {
        Err("无法获取可用内存信息，可能系统资源不足或数据未初始化".to_string())
    } else {
        Ok(available_memory)
    }
}



/// 获取本机磁盘总空间，单位字节
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::total_space(&mut sys), Ok(1254954610688));
/// ```

pub fn total_space(sys: &mut System) -> Result<u64, String> {
    let disks: Disks = Disks::new_with_refreshed_list();

    if disks.is_empty() {
        return Err("无法获取磁盘信息，磁盘列表为空".to_string());
    }

    let total_space = disks.iter().map(|disk| disk.total_space()).sum();

    Ok(total_space)
}


/// 获取本机磁盘可用总空间，单位字节
/// 
/// # Examples
/// 
/// ```
/// use sysinfo::{Components, Disks, Networks, System};
/// let mut sys: System = System::new_all();
/// assert_eq!(collection_learning::available_space(&mut sys), Ok(866847330304));
/// ```
pub fn available_space(sys: &mut System) -> Result<u64, String> {
    let disks: Disks = Disks::new_with_refreshed_list();


    if disks.is_empty() {
        return Err("无法获取磁盘信息，磁盘列表为空".to_string());
    }

    let total_available_space: u64 = disks.iter().map(|disk| disk.available_space()).sum();

    Ok(total_available_space)
}
