use iced::{button, Column, Element};

#[derive(Clone)]
pub struct RootSteps {
    steps: Vec<RootStep>,
    current: usize,
}

impl RootSteps {
    pub fn new() -> RootSteps {
        RootSteps {
            steps: vec![
                RootStep::Intro,
                RootStep::DownloadAndInstallMagisk,
                RootStep::PromptPlugInPhone,
                RootStep::StartADBServer,
                RootStep::UnlockBootloader,
                RootStep::GetLatestFactoryImage,
                RootStep::PatchBootImage,
                RootStep::Flash,
                RootStep::End,
            ],
            current: 0,
        }
    }

    pub fn update(&mut self, msg: RootStepMessage) {
        self.steps[self.current].update(msg);
    }

    pub fn view(&mut self) -> Element<RootStepMessage> {
        self.steps[self.current].view()
    }

    pub fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    pub fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    pub fn has_previous(&self) -> bool {
        self.current > 0
    }

    pub fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    pub fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

#[derive(Clone, Copy)]
pub enum RootStep {
    Intro,
    DownloadAndInstallMagisk,
    PromptPlugInPhone,
    StartADBServer,
    UnlockBootloader,
    GetLatestFactoryImage,
    PatchBootImage,
    Flash,
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum RootStepMessage {}

impl<'a> RootStep {
    fn update(&mut self, msg: RootStepMessage) {
        match msg {}
    }

    fn title(&self) -> &str {
        match self {
            Self::Intro => "Introduction",
            Self::DownloadAndInstallMagisk => "Download Magisk",
            Self::PromptPlugInPhone => "Plug in your phone",
            Self::StartADBServer => "Start ADB server",
            Self::UnlockBootloader => "Unlock bootloader",
            Self::GetLatestFactoryImage => "Download image",
            Self::PatchBootImage => "Patch image",
            Self::Flash => "Flash to device",
            Self::End => "Finish",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Self::Intro => true,
            Self::DownloadAndInstallMagisk => true,
            Self::PromptPlugInPhone => true,
            Self::StartADBServer => true,
            Self::UnlockBootloader => true,
            Self::GetLatestFactoryImage => true,
            Self::PatchBootImage => true,
            Self::Flash => true,
            Self::End => true,
        }
    }

    fn view(&mut self) -> Element<RootStepMessage> {
        match self {
            Self::Intro => Self::intro(),
            Self::DownloadAndInstallMagisk => Self::download_and_install_magisk(),
            Self::PromptPlugInPhone => Self::prompt_plug_in_phone(),
            Self::StartADBServer => Self::start_adb_server(),
            Self::UnlockBootloader => Self::unlock_bootloader(),
            Self::GetLatestFactoryImage => Self::get_latest_factory_image(),
            Self::PatchBootImage => Self::patch_boot_image(),
            Self::Flash => Self::flash(),
            Self::End => Self::end(),
        }
        .into()
    }

    fn intro() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn download_and_install_magisk() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn prompt_plug_in_phone() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn start_adb_server() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn unlock_bootloader() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn get_latest_factory_image() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn patch_boot_image() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn flash() -> Column<'a, RootStepMessage> {
        Column::new()
    }

    fn end() -> Column<'a, RootStepMessage> {
        Column::new()
    }
}
