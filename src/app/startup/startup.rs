use crate::app::app::App;

impl App<'_> {
    /// Method called before running the app
    pub fn startup(&mut self) -> &mut Self {
        self.set_collections_from_file();

        self
    }
}