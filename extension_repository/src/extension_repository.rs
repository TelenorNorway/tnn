use std::{
	collections::{HashMap, HashSet},
	sync::Arc,
};

use anyhow::Result;
use async_recursion::async_recursion;
use extension::{Dependency, Extension, ExtensionContext};
use semver::{Version, VersionReq};
use thiserror::Error;
use tokio::sync::Mutex;

pub struct ExtensionRepository {
	locked: Arc<Mutex<bool>>,

	/// All the extensions that has been added to the
	/// repository.
	all_extensions: Arc<Mutex<HashMap<&'static str, &'static Extension>>>,

	/// The extension IDs that has been activated.
	activated_extensions: Arc<Mutex<Vec<&'static str>>>,

	/// A map of extension id to extension versions.
	extension_id_to_version: Arc<Mutex<HashMap<&'static str, &'static str>>>,

	/// Used for access control.
	///
	/// Extensions/dependencies is added to the map/set once
	/// they are resolved.
	///
	/// HashMap<ExtensionName, HashSet<DependencyName>>
	extension_dependencies_resolved: Arc<Mutex<HashMap<&'static str, HashSet<&'static str>>>>,

	/// Used for dependency resolution.
	///
	/// HashMap<ExtensionName, HashMap<DependencyName, (VersionMatcher, IsRequired)>>
	extension_dependencies_expected: Arc<Mutex<HashMap<&'static str, HashMap<&'static str, (&'static str, bool)>>>>,

	/// Used to locate dependents requirements.
	extensions_dependents_expected: Arc<Mutex<HashMap<&'static str, Vec<&'static str>>>>,

	/// Vec<(ExtensionName, ExtensionVersion, DependencyName, DependencyVersion, DependencyVersionMatcher)>
	version_mismatches: Arc<Mutex<Vec<(&'static str, &'static str, &'static str, &'static str, &'static str)>>>,
}

impl ExtensionRepository {
	fn construct() -> ExtensionRepository {
		ExtensionRepository {
			locked: Arc::new(Mutex::new(false)),
			all_extensions: Arc::new(Mutex::new(HashMap::new())),
			activated_extensions: Arc::new(Mutex::new(Vec::new())),
			extension_id_to_version: Arc::new(Mutex::new(HashMap::new())),
			extension_dependencies_resolved: Arc::new(Mutex::new(HashMap::new())),
			extension_dependencies_expected: Arc::new(Mutex::new(HashMap::new())),
			extensions_dependents_expected: Arc::new(Mutex::new(HashMap::new())),
			version_mismatches: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn new() -> ExtensionRepository {
		let repository = Self::construct();
		repository
	}

	/// Get the version of an added extension, returns None
	/// if an extension by the given name does not exist.
	async fn get_extension_version_for(&self, extension_name: &'static str) -> Option<&'static str> {
		self.extension_id_to_version
			.lock()
			.await
			.get(extension_name)
			.map(|extension_version| *extension_version)
	}

	/// Insert an extension into the repository.
	///
	/// Note that the extension might not be activated
	/// immediately if the extension has unresolved
	/// dependencies. Use
	/// [`ExtensionRepository::assert_all_activated`] to
	/// ensure that all added extensions has been activated.
	pub async fn add(&self, extension: &'static Extension) -> Result<()> {
		self.try_insert_extension(extension).await
	}

	pub async fn print_problems(&self) {
		for (extension, dependencies) in self.extension_dependencies_expected.lock().await.iter() {
			if dependencies.len() == 0 {
				continue;
			}
			let mut missing: Vec<&'static str> = Vec::new();
			for (dependency, (_, is_required)) in dependencies {
				if *is_required {
					missing.push(dependency);
				}
			}
			logs::critical!(
				"Extension '{}@{}' was not activated, missing '{}'",
				extension,
				self.get_extension_version_for(extension).await.unwrap(),
				missing.join("', '"),
			)
		}
	}

	/// Insert an extension into the repository and activate
	/// it immediately.
	///
	/// Note that this returns errors if there are unresolved
	/// dependencies or version mismatches.
	pub async fn inject(&self, _extension: &'static Extension) -> Result<()> {
		if *self.locked.lock().await {
			return Err(ExtensionInstallationError::Locked.into());
		}

		todo!("James Bradlee: Implement inject")
	}

	/// UNSAFE: Insert extension without doing any checks at
	/// all.
	async fn unsafely_insert_extension(&self, extension: &'static Extension) {
		self.all_extensions.lock().await.insert(extension.name, extension);
		self.extension_id_to_version
			.lock()
			.await
			.insert(extension.name, extension.version);
	}

	/// Insert extension after validating that the extension can definitely be inserted.
	async fn try_insert_extension(&self, extension: &'static Extension) -> Result<()> {
		if *self.locked.lock().await {
			return Err(ExtensionInstallationError::Locked.into());
		}

		if let Some(version) = self.get_extension_version_for(extension.name).await {
			if version == extension.version {
				// If its the same extension and version, just don't worry about it
				return Ok(());
			}
			// If it's a different version, it's a bigger problem
			return Err(
				ExtensionInstallationError::ExtensionAlreadyAdded(extension.name, version, extension.version).into(),
			);
		}

		self.unsafely_insert_extension(extension).await;

		self.resolve(extension).await?;

		Ok(())
	}

	async fn resolve(&self, extension: &'static Extension) -> Result<()> {
		let mut all_names: HashSet<&'static str> = HashSet::new();

		let mut has_problems = false;
		let mut pending_dependencies: HashMap<&'static str, (&'static str, bool)> = HashMap::new();
		let mut solved_dependencies: HashSet<&'static str> = HashSet::new();
		let mut pending_dependency_names: Vec<&'static str> = Vec::new();

		for dependency in extension.dependencies {
			let (is_required, name, version_matcher) = match dependency {
				Dependency::Optional(name, version_matcher) => (false, *name, *version_matcher),
				Dependency::Required(name, version_matcher) => (true, *name, *version_matcher),
			};
			if all_names.contains(name) {
				return Err(
					ExtensionInstallationError::DuplicateDependency(extension.name, extension.version, name).into(),
				);
			}
			all_names.insert(name);

			if !self.activated_extensions.lock().await.contains(&name) {
				pending_dependencies.insert(name, (version_matcher, is_required));
				pending_dependency_names.push(name);
			} else {
				if let Some(received_version) = self.match_dependency(name, version_matcher).await? {
					if is_required {
						has_problems = true;
						self.version_mismatches.lock().await.push((
							extension.name,
							extension.version,
							name,
							received_version,
							version_matcher,
						));
						logs::warn!(
							"Extension '{}@{}' expected version '{}' from required dependency '{}' (but got '{}') - extension will not be initialized",
							extension.name,
							extension.version,
							version_matcher,
							name,
							received_version
						);
					} else {
						logs::warn!(
							"Extension '{}@{}' expected version '{}' from optional dependency '{}' (but got '{}')",
							extension.name,
							extension.version,
							version_matcher,
							name,
							received_version
						);
					}
				} else {
					solved_dependencies.insert(name);
				}
			}
		}

		{
			let mut reverse = self.extensions_dependents_expected.lock().await;
			for name in pending_dependency_names {
				if let Some(lookup) = reverse.get_mut(name) {
					lookup.push(extension.name);
				} else {
					reverse.insert(name, vec![extension.name]);
				}
			}
		}

		let has_pending = pending_dependencies.len() > 0;

		self.extension_dependencies_expected
			.lock()
			.await
			.insert(extension.name, pending_dependencies);

		self.extension_dependencies_resolved
			.lock()
			.await
			.insert(extension.name, solved_dependencies);

		if !has_pending && !has_problems {
			self.complete(extension).await?;
		}

		Ok(())
	}

	#[async_recursion(?Send)]
	async fn complete(&self, extension: &'static Extension) -> Result<()> {
		logs::debug!("[repository] Completing {}@{}", extension.name, extension.version);
		self.activate_extension(extension).await?;
		logs::debug!(
			"[repository] Initialized {}@{} - now resolving dependents",
			extension.name,
			extension.version
		);

		let mut extensions_to_complete: Vec<&'static Extension> = Vec::new();

		if let Some(dependents) = self.extensions_dependents_expected.lock().await.remove(extension.name) {
			for dependent in dependents {
				logs::debug!(
					"[repository] from {}@{} resolving {}",
					extension.name,
					extension.version,
					dependent
				);
				let mut has_problems = false;
				let mut should_complete = false;

				{
					let dependent_version = self.get_extension_version_for(dependent).await.unwrap();
					let mut expected = self.extension_dependencies_expected.lock().await;
					let deps_dependencies = expected.get_mut(dependent).unwrap();
					let size = deps_dependencies.len();
					let (version_matcher, is_required) = deps_dependencies.remove(extension.name).unwrap();

					if self.match_dependency(extension.name, version_matcher).await?.is_some() {
						if is_required {
							has_problems = true;
							self.version_mismatches.lock().await.push((
								dependent,
								dependent_version,
								extension.name,
								extension.version,
								version_matcher,
							));
						}
					} else {
						self.extension_dependencies_resolved
							.lock()
							.await
							.get_mut(dependent)
							.unwrap()
							.insert(extension.name);
						if size == 1 {
							should_complete = true;
						}
					}
				}

				if !has_problems && should_complete {
					extensions_to_complete.push(self.all_extensions.lock().await.get(dependent).unwrap());
				}
			}
		}

		for ext in extensions_to_complete {
			self.complete(ext).await?;
		}

		logs::debug!("[repository] Completed {}@{}", extension.name, extension.version);

		Ok(())
	}

	async fn match_dependency(
		&self,
		dependency_name: &'static str,
		version_match: &'static str,
	) -> Result<Option<&'static str>> {
		if let Some(received_version) = self.extension_id_to_version.lock().await.get(dependency_name) {
			let expected_version_match = VersionReq::parse(version_match)?;
			let received_version_semver = Version::parse(received_version)?;
			if expected_version_match.matches(&received_version_semver) {
				Ok(None)
			} else {
				Ok(Some(received_version))
			}
		} else {
			Err(ExtensionInstallationError::ExtensionNotFound(dependency_name).into())
		}
	}

	async fn activate_extension(&self, extension: &'static Extension) -> Result<()> {
		let extension_context = Arc::new(ExtensionContext::new(extension.name));
		(extension.init)(Arc::clone(&extension_context)).await?;
		self.activated_extensions.lock().await.push(extension.name);
		Ok(())
	}

	pub async fn lock(&self) -> Result<()> {
		{
			let mut locked = self.locked.lock().await;
			if *locked {
				return Ok(());
			}

			*locked = true;
		}
		// todo(James Bradlee): Fire the locked event.

		Ok(())
	}
}

#[derive(Error, Debug)]
pub enum ExtensionInstallationError {
	#[error("Repository is locked")]
	Locked,

	/// The extension has already been added.
	/// - The extension that has already been added
	/// - The added extension version
	/// - The version of the extension currently being
	///   attempted to install
	#[error("Extension '{0}' has already been added with version '{1}', trying to add '{2}'!")]
	ExtensionAlreadyAdded(&'static str, &'static str, &'static str),

	/// An extension requires another extension to be
	/// installed, but the other extension isn't.
	/// - The dependent, the extension that is missing the
	///   other extension
	/// - The dependent version
	/// - The dependency, the extension that is missed
	/// - The dependency version
	#[error("Extension '{0}@{1}' is missing a required dependency '{2}@{3}'")]
	MissingDependency(&'static str, &'static str, &'static str, &'static str),

	/// A dependency has a different version than expected by
	/// dependent.
	/// - The dependent, the extension that received a wrong
	///   version from dependency
	/// - The dependent version
	/// - The dependency, the extension with the unexpected
	///   version
	/// - The dependency version
	/// - The expected dependency version range(s)
	#[error("Extension '{0}@{1}' expected dependency '{2}' with a version in range(s) '{4}', but got '{3}'")]
	VersionMismatch(&'static str, &'static str, &'static str, &'static str, &'static str),

	/// Extension contains duplicate dependency of X.
	/// - The extension name
	/// - The extension version
	/// - The duplicate dependency name
	#[error("Extension '{0}@{1}' contains a duplicate dependency of '{2}'")]
	DuplicateDependency(&'static str, &'static str, &'static str),

	/// A given dependency name cannot be found in the repository.
	/// - The extension name
	///
	/// This should never happen! Panic.
	#[error("Extension '{0}' not found!")]
	ExtensionNotFound(&'static str),
}
