pub enum Dependency {
	Required {
		name: &'static str,
		version_matcher: &'static str,
	},
	Optional {
		name: &'static str,
		version_matcher: &'static str,
	},
}
