use cfg_aliases::cfg_aliases;

fn main() {
	tauri_build::build();
	cfg_aliases! {
		mobile: { any(target_os = "ios", target_os = "android") },
		desktop: { not(any(target_os = "ios", target_os = "android")) }
	}
}
