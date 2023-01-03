use std::{
	sync::atomic::Ordering,
	thread,
	time::{Duration, Instant},
};

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, MousePosition};
use fltk::app::get_mouse;

use crate::profile::Profile;

use super::EffectPlayer;

pub(super) struct Fade;

impl EffectPlayer for Fade {
	fn play(manager: &mut super::EffectManager, p: Profile, _thread_rng: &mut rand::rngs::ThreadRng) {
		let stop_signals = manager.stop_signals.clone();
		thread::spawn(move || {
			let device_state = DeviceState::new();
			let mut mouse_coords_global: MousePosition = device_state.get_mouse().coords;
			while !stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
				let keys: Vec<Keycode> = device_state.get_keys();
				let mouse = device_state.get_mouse();
				let mouse_coords = mouse.coords;
				if !keys.is_empty() && !mouse.button_pressed[1] && !mouse.button_pressed[2] && !mouse.button_pressed[3] && mouse_coords == mouse_coords_global {
					stop_signals.keyboard_stop_signal.store(true, Ordering::SeqCst);
				} else {
					mouse_coords_global = device_state.get_mouse().coords;
				}
				thread::sleep(Duration::from_millis(5));
			}
		});

		let device_state = DeviceState::new();
		let mut now = Instant::now();
		let mut mouse_coords_global = device_state.get_mouse().coords;
		while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
			let keys: Vec<Keycode> = device_state.get_keys();
			let mouse: MouseState = device_state.get_mouse();
			let mouse_coords = mouse.coords;
			if keys.is_empty() && !mouse.button_pressed[1] && !mouse.button_pressed[2] && !mouse.button_pressed[3] && mouse_coords == mouse_coords_global {
				if now.elapsed() > Duration::from_secs(p.fade_time / u64::from(p.speed)) {
					manager.keyboard.transition_colors_to(&[0; 12], 230, 3);
				} else {
					thread::sleep(Duration::from_millis(5));
				}
			} else {
				manager.keyboard.set_colors_to(&p.rgb_array);
				manager.stop_signals.keyboard_stop_signal.store(false, Ordering::SeqCst);
				now = Instant::now();
				mouse_coords_global = device_state.get_mouse().coords;
			}
		}
	}
}
