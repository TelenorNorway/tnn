pub type DependencyName = &'static str;
pub type DependencyVersion = &'static str;

pub enum Dependency {
	Optional(DependencyName, DependencyVersion),
	Required(DependencyName, DependencyVersion),
}
