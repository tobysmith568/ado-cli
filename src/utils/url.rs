use open::that;

pub struct Url {
    path: String,
}

impl Url {
    pub fn from(path: String) -> Url {
        Url { path }
    }

    pub fn open_in_browser(&self) {
        _ = that(&self.path);
    }
}
