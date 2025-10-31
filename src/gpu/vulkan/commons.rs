use std::os::raw::c_char;
use std::ptr::{null};
use bhomz::cstr::ToCCharPtr;
use bhomz::error::{BhomzError, BhomzResult};
use bhomz::log_err;
use crate::bindings::{vkCreateInstance, Uint32, VkApplicationInfo, VkInstance, VkInstanceCreateInfo, VkPhysicalDevice, VkResult, VkResult_VK_SUCCESS, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO, VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO};

const VK_OK: VkResult = VkResult_VK_SUCCESS;

pub(crate) fn create_vulkan_instance(app_name: &str, ext_count: Uint32, ext: Vec<*const c_char>) -> BhomzResult<VkInstance> {
	let app_info = VkApplicationInfo {
		sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
		pNext: null(),
		pApplicationName: app_name.to_c_char_ptr(),
		applicationVersion: 0,
		pEngineName: "BhUIz Engine (Vulkan)".to_c_char_ptr(),
		engineVersion: 0,
		apiVersion: 0,
	};
	let instance_info = VkInstanceCreateInfo {
		sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
		pNext: null(),
		flags: 0,
		pApplicationInfo: &app_info,
		enabledLayerCount: 0,
		ppEnabledLayerNames: null(),
		enabledExtensionCount: ext_count,
		ppEnabledExtensionNames: ext.as_ptr(),
	};
	let mut instance = Box::new_uninit();
	if unsafe { vkCreateInstance(&instance_info, null(), instance.as_mut_ptr()) } != VK_OK || instance.as_ptr().is_null() {
		return log_err!(BhomzError, "Could not create Vulkan Instance");
	}
	Ok(unsafe { instance.assume_init_read() })
}

pub(crate) fn create_vulkan_phys_devices(instance: VkInstance) -> BhomzResult<Vec<VkPhysicalDevice>> {
	let mut phys_devices = Vec::new();
	Ok(phys_devices)
}