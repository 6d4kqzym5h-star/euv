use super::*;

/// Implements cache query and management for `AssetCache`.
impl AssetCache {
    /// Returns the state of the asset with the given URL, or `None` if not cached.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The asset URL.
    ///
    /// # Returns
    ///
    /// - `Option<AssetState>` - The asset state, or `None`.
    pub fn get_state<U>(&self, url: U) -> Option<AssetState>
    where
        U: AsRef<str>,
    {
        self.get_entries()
            .get(url.as_ref())
            .map(|entry: &AssetEntry| entry.get_state())
    }

    /// Returns the loaded image for the given URL, or `None` if not loaded.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The asset URL.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The loaded image, or `None`.
    pub fn get_image<U>(&self, url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        let entry: &AssetEntry = self.get_entries().get(url.as_ref())?;
        if entry.get_state() != AssetState::Loaded {
            return None;
        }
        entry.get_image()
    }

    /// Returns `true` if all assets in the cache have finished loading.
    ///
    /// # Returns
    ///
    /// - `bool` - True if no assets are in the `Loading` state.
    pub fn is_all_loaded(&self) -> bool {
        self.get_entries()
            .values()
            .all(|entry: &AssetEntry| entry.get_state() != AssetState::Loading)
    }

    /// Returns the number of assets that have been successfully loaded.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of loaded assets.
    pub fn loaded_count(&self) -> usize {
        self.get_entries()
            .values()
            .filter(|entry: &&AssetEntry| entry.get_state() == AssetState::Loaded)
            .count()
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.get_mut_entries().clear();
    }
}

/// Implements `Default` for `AssetCache` as a new empty cache.
impl Default for AssetCache {
    fn default() -> AssetCache {
        AssetCache::new()
    }
}

/// Implements asynchronous asset loading for `AssetLoader`.
impl AssetLoader {
    /// Begins loading an image asset from the given URL.
    ///
    /// Creates an `HtmlImageElement`, sets its `src`, and registers `onload`/`onerror`
    /// callbacks to update the shared cache state. The image loads asynchronously.
    ///
    /// # Arguments
    ///
    /// - `String` - The URL of the image to load.
    pub fn load_image(&mut self, url: String) {
        let image: HtmlImageElement = HtmlImageElement::new().expect("should create image element");
        let entry: AssetEntry = AssetEntry::new(
            AssetType::Image,
            AssetState::Loading,
            Some(image.clone()),
            url.clone(),
        );
        self.get_cache()
            .get_mut()
            .get_mut_entries()
            .insert(url.clone(), entry);
        *self.get_mut_pending_count() += 1;
        let cache_clone: Rc<EngineCell<AssetCache>> = self.get_cache().clone();
        let url_for_onload: String = url.clone();
        let url_for_onerror: String = url.clone();
        let onload_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let cache_ref: &mut AssetCache = cache_clone.get_mut();
            if let Some(mut entry) = cache_ref.get_entries().get(&url_for_onload).cloned() {
                entry.set_state(AssetState::Loaded);
                cache_ref
                    .get_mut_entries()
                    .insert(url_for_onload.clone(), entry);
            }
        }));
        let cache_clone_err: Rc<EngineCell<AssetCache>> = self.get_cache().clone();
        let onerror_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let cache_ref: &mut AssetCache = cache_clone_err.get_mut();
            if let Some(mut entry) = cache_ref.get_entries().get(&url_for_onerror).cloned() {
                entry.set_state(AssetState::Error);
                cache_ref
                    .get_mut_entries()
                    .insert(url_for_onerror.clone(), entry);
            }
        }));
        image.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        image.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
        image.set_src(&url);
        self.get_closures().get_mut().push(onload_closure);
        self.get_closures().get_mut().push(onerror_closure);
    }

    /// Returns whether all requested assets have finished loading.
    ///
    /// # Returns
    ///
    /// - `bool` - True if no assets are pending.
    pub fn is_all_loaded(&self) -> bool {
        self.get_cache().get().is_all_loaded()
    }

    /// Returns the loaded image for the given URL.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The asset URL.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The loaded image, or `None`.
    pub fn get_image<U>(&self, url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        self.get_cache().get().get_image(url.as_ref())
    }

    /// Returns the progress ratio of loaded assets.
    ///
    /// # Returns
    ///
    /// - `f64` - The ratio in the range 0.0 to 1.0.
    pub fn progress(&self) -> f64 {
        let cache_ref: &AssetCache = self.get_cache().get();
        let total: usize = cache_ref.get_entries().len();
        if total == 0 {
            return 1.0;
        }
        cache_ref.loaded_count() as f64 / total as f64
    }
}

/// Implements `Default` for `AssetLoader` as a new empty loader.
impl Default for AssetLoader {
    fn default() -> AssetLoader {
        AssetLoader::new()
    }
}

/// Implements static asset creation utilities for `AssetLoader`.
impl AssetLoader {
    /// Creates an `HtmlImageElement` from the given URL without caching.
    ///
    /// The image loads asynchronously. Returns immediately with the image element
    /// whose `src` is set but may not have finished loading yet.
    ///
    /// # Arguments
    ///
    /// - `U: AsRef<str>` - The image URL.
    ///
    /// # Returns
    ///
    /// - `Option<HtmlImageElement>` - The image element, or `None` if creation failed.
    pub fn create_image_element<U>(url: U) -> Option<HtmlImageElement>
    where
        U: AsRef<str>,
    {
        let image: HtmlImageElement = HtmlImageElement::new().ok()?;
        image.set_src(url.as_ref());
        Some(image)
    }
}
