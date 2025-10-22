use std::os::raw::{c_char, c_uint};
use bindings::{VkInstance, VkInstanceCreateInfo};
use interfaces::{cstr_ptr, log_err};
use interfaces::graphics::{GpuError, GpuAdapter, GpuResult};
use crate::bindings::*;

#[allow(warnings)]
mod bindings;

pub struct Vulkan {
	pub instance: Vec<VkInstance>,
	pub physical_devices: Vec<VkPhysicalDevice>,
	pub physical_devices_families: Vec<VkQueueFamilyProperties>,
	pub surface: Vec<VkSurfaceKHR>
}

impl Vulkan {
	pub fn init(app_name: &str, engine_name: &str, ext_count: c_uint, extensions: Vec<*const c_char>) -> GpuResult<Vulkan> {
		log::debug!("Starting vulkan, app_name: '{app_name}', engine_name: '{engine_name}', extensions: {:?}", &extensions);
		let instance: Vec<VkInstance> = Self::create_instance(app_name, engine_name, ext_count, extensions)?;

		let physical_devices = Self::get_physical_devices(&instance);
		log::debug!("Physical Devices: {:?}", &physical_devices);

		let physical_devices_families = Self::get_physical_devices_families(&physical_devices);
		log::debug!("Physical Devices Families: {:?}", &physical_devices_families);

		Ok(Vulkan {
			instance,
			physical_devices,
			physical_devices_families,
			surface: Vec::with_capacity(1)
		})
	}

	fn create_instance(app_name: &str, engine_name: &str, ext_count: c_uint, extensions: Vec<*const c_char>) -> GpuResult<Vec<VkInstance>> {
		let mut instance: Vec<VkInstance> = vec![std::ptr::null_mut()];
		let app_info = VkApplicationInfo {
			pNext: std::ptr::null_mut(),

			sType: 0,
			pApplicationName: cstr_ptr(app_name),
			applicationVersion: 0,

			pEngineName: cstr_ptr(engine_name),
			engineVersion: 0,

			apiVersion: 0,
		};
		let ins_info = VkInstanceCreateInfo {
			pNext: std::ptr::null_mut(),

			sType: 0,
			flags: 0,
			pApplicationInfo: &app_info,

			enabledLayerCount: 0,
			ppEnabledLayerNames: std::ptr::null_mut(),

			enabledExtensionCount: ext_count,
			ppEnabledExtensionNames: extensions.as_ptr(),
		};
		unsafe {
			if vkCreateInstance(&ins_info, std::ptr::null(), instance.as_mut_ptr()) != VkResult_VK_SUCCESS {
				return log_err!("failed to create Vulkan instance", GpuError);
			}
		}

		Ok(instance)
	}

	fn get_physical_devices(instance: &Vec<VkInstance>) -> Vec<VkPhysicalDevice> {
		let mut physical_device_count = 0u32 as c_uint;
		unsafe { vkEnumeratePhysicalDevices(*instance.as_ptr(), &mut physical_device_count, std::ptr::null_mut()); }

		let mut physical_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(physical_device_count as usize);
		unsafe {
			vkEnumeratePhysicalDevices(*instance.as_ptr(), &mut physical_device_count, physical_devices.as_mut_ptr());
			// Since C doesnt update the len, update here
			physical_devices.set_len(physical_device_count as usize);
		}

		physical_devices
	}

	fn get_physical_devices_families(physical_devices: &Vec<VkPhysicalDevice>) -> Vec<VkQueueFamilyProperties> {
		let mut physical_device_count = 0u32 as c_uint;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(*physical_devices.as_ptr(), &mut physical_device_count, std::ptr::null_mut()); }

		let mut physical_devices_families: Vec<VkQueueFamilyProperties> = Vec::with_capacity(physical_device_count as usize);
		unsafe {
			vkGetPhysicalDeviceQueueFamilyProperties(*physical_devices.as_ptr(), &mut physical_device_count, physical_devices_families.as_mut_ptr());
			// Since C doesnt update the len, update here
			physical_devices_families.set_len(physical_device_count as usize);
		}

		physical_devices_families
	}

	pub fn init_devices(&mut self) {
		let mut graphics_index = u32::MAX;
		let mut presents_index = u32::MAX;
		for (index, familyQueue) in self.physical_devices_families.iter().enumerate() {
		}
	}
}

impl GpuAdapter for Vulkan {
	fn close(self) {
		for instance in self.instance {
			if !instance.is_null() {
				unsafe { vkDestroyInstance(instance, std::ptr::null()); }
			} else {
				log::info!("Vulkan was not initialized properly: instance is a nullptr");
			}
		}
	}
}