pub mod end_scene;
pub mod main_scene;
pub mod pause_scene;
pub mod start_scene;

#[derive(Eq, PartialEq, Hash, Debug)]
#[allow(dead_code)]
pub enum ActiveScene {
    Start,
    Main,
    Pause,
    End,
}

impl ActiveScene {
    pub fn change_to_main(&mut self) {
        *self = ActiveScene::Main;
    }
}

impl Default for ActiveScene {
    fn default() -> Self {
        ActiveScene::Start
    }
}
