use std::os::raw::{c_char, c_uint};
use bindings::{VkInstance, VkInstanceCreateInfo};
use interfaces::cstr_ptr;
use interfaces::graphics::driver::{GpuInterface};
use interfaces::graphics::errors::{GpuError, GpuResult};
use crate::bindings::*;

#[allow(warnings)]
mod bindings;

pub struct Vulkan {
	app_info: *mut VkInstanceCreateInfo,
	instance: Vec<VkInstance>
}

impl Vulkan {
	pub fn init(ext_count: c_uint, extensions: Vec<*const c_char>) -> GpuResult<Vulkan> {
		let app_info = std::ptr::null_mut();
		let mut instance: Vec<VkInstance> = vec![std::ptr::null_mut()];

		unsafe {
			let application_info = VkApplicationInfo {
				sType: 0,
				pNext: std::ptr::null_mut(),
				pApplicationName: cstr_ptr("wa"),
				applicationVersion: 0,
				pEngineName: cstr_ptr("vulkan"),
				engineVersion: 0,
				apiVersion: 0,
			};
			let app_info: *const VkInstanceCreateInfo = std::ptr::from_ref(&VkInstanceCreateInfo {
				sType: 0,
				pNext: std::ptr::null_mut(),
				flags: 0,
				pApplicationInfo: &application_info,
				enabledLayerCount: 0,
				ppEnabledLayerNames: std::ptr::null_mut(),
				enabledExtensionCount: ext_count,
				ppEnabledExtensionNames: extensions.as_ptr(),
			});
			if vkCreateInstance(app_info, std::ptr::null(), instance.as_mut_ptr()) != VkResult_VK_SUCCESS {
				return Err(GpuError("failed to create Vulkan instance".to_string()));
			}
		}
		Ok(Vulkan {
			app_info,
			instance
		})
	}
}

impl GpuInterface for Vulkan {
	fn close(self) {
		unsafe {
			for instance in self.instance {
				if !instance.is_null() {
					vkDestroyInstance(instance, std::ptr::null());
				} else {
					println!("Why vkInstance null?")
				}
			}
		}
	}
}