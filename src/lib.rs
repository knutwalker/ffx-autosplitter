#![no_std]

use asr::{
    Address, Process,
    deep_pointer::DeepPointer,
    future::{next_tick, retry},
    settings::{Gui, gui::Title as Heading},
    signature::Signature,
    timer::{self, TimerState},
    watcher::{Pair, Watcher},
};
use bytemuck::CheckedBitPattern;
#[cfg(any(testing, debugging))]
use bytemuck::checked;
use core::{fmt, iter, ops::ControlFlow};
use num_enum::IntoPrimitive;
use strum::{EnumIter, IntoEnumIterator as _};

mod enum_set;

asr::async_main!(stable);
asr::panic_handler!();

#[macro_export]
macro_rules! log {
    ($format:literal $($arg:tt)*) => {{
        let mut buf = ::asr::arrayvec::ArrayString::<8192>::new();
        let _ = ::core::fmt::Write::write_fmt(
            &mut buf,
            ::core::format_args!(concat!("[FFX]: ", $format) $($arg)*),
        );
        ::asr::print_message(&buf);
    }};
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoPrimitive, EnumIter)]
#[repr(u8)]
enum Splits {
    Ammes,
    Klikk,
    Tros,
    Lagoon,
    BesaidVillage,
    Kimahri,
    BesaidRoad,
    Echuilles,
    Geneaux,
    Ifrit,
    Kilika,
    Oblitzerator,
    BeforeBlitzball,
    BlitzballDone,
    Garuda,
    MiihenRoad,
    ChocoboEater,
    OldRoad,
    Mrr,
    Gui,
    MrrSkip,
    DjoseRoad,
    Ixion,
    EnterMoonflow,
    MoonflowSouth,
    Extractor,
    Guadosalam,
    ThunderPlains,
    MacalaniaWoods,
    Spherimorph,
    Crawler,
    Seymour,
    Shiva,
    Wendigo,
    Bikanel,
    Home,
    Evrae,
    Guards,
    Bahamut,
    ViaPurifico,
    Isaaru,
    Natus,
    CalmLands,
    BiranYenke,
    Flux,
    SanctuaryKeeper,
    Zanarkand,
    Tetris,
    SpectralKeeper,
    Yunalesca,
    Core,
    Overdrive,
    Omnis,
    Eggs,
    Bfa,
    YuYevon,
    #[cfg(testing)]
    SinFin,
    #[cfg(testing)]
    Luca,
    #[cfg(testing)]
    Workers1,
    #[cfg(testing)]
    Workers2,
    #[cfg(testing)]
    Workers31,
    #[cfg(testing)]
    Workers32,
    #[cfg(testing)]
    Workers3,
    #[cfg(testing)]
    Sahagins1,
    #[cfg(testing)]
    Sahagins2,
    #[cfg(testing)]
    Sahagins,
    #[cfg(testing)]
    MoonflowNorth,
    #[cfg(testing)]
    OakaShop,
    #[cfg(testing)]
    CrawlerGrid,
    #[cfg(testing)]
    SeymourGrid,
    #[cfg(testing)]
    WendigoGrid,
    #[cfg(testing)]
    Crevasse,
    #[cfg(testing)]
    BikanelParty,
    #[cfg(testing)]
    BikanelYeet,
    #[cfg(testing)]
    Bombs,
    #[cfg(testing)]
    DualHorns,
    #[cfg(testing)]
    Chimeras,
    #[cfg(testing)]
    Guards1,
    #[cfg(testing)]
    Guards2,
    #[cfg(testing)]
    Guards3,
    #[cfg(testing)]
    Guards4,
    #[cfg(testing)]
    Altana,
    #[cfg(testing)]
    ViaUnderwater,
    #[cfg(testing)]
    Defender,
}

#[derive(Gui)]
pub struct Settings {
    /// Start the timer when a new run starts
    #[default = true]
    start: bool,

    /// Enable autosplitting. See below for detailed splits
    #[default = true]
    split: bool,

    /// Reset the timer on music selection
    #[default = false]
    reset: bool,

    /// Remove load times from Game Time
    #[default = true]
    remove_loads: bool,

    /// Count encounters, use Text layout to read the `encounter_count` value
    #[default = true]
    count_encounters: bool,

    /// Splits: Enable the settings that match your splits!
    _splits_heading1: Heading,

    /// You don't need to enable all, only what you want to split.
    _splits_heading2: Heading,

    /// Anything else not mentioned here can still be split manually.
    _splits_heading3: Heading,

    /// Sinspawn Ammes
    #[default = true]
    ammes: bool,

    /// Klikk
    #[default = true]
    klikk: bool,

    /// Tros
    #[default = true]
    tros: bool,

    /// Piranhas
    #[default = true]
    lagoon: bool,

    /// Besaid Village
    #[default = false]
    besaid_village: bool,

    /// Kimahri
    #[default = true]
    kimahri: bool,

    /// Leaving Besaid Road
    #[default = false]
    besaid_road: bool,

    /// Sinspawn Echuilles
    #[default = true]
    echuilles: bool,

    /// Sinspawn Geneaux
    #[default = true]
    geneaux: bool,

    /// Kilika Trials
    #[default = false]
    ifrit: bool,

    /// Kilika Woods
    #[default = true]
    kilika: bool,

    /// Oblitzerator
    #[default = true]
    oblitzerator: bool,

    /// Before Blitzball
    #[default = false]
    before_blitzball: bool,

    /// Blitzball Complete
    #[default = true]
    blitzball_done: bool,

    /// Garuda
    #[default = true]
    garuda: bool,

    /// Mi'ihen Highroad
    #[default = true]
    miihen_road: bool,

    /// Chocobo Eater
    #[default = false]
    chocobo_eater: bool,

    /// Old Road
    #[default = true]
    old_road: bool,

    /// Mushroom Rock Road (Not with MRR skip)
    #[default = true]
    mrr: bool,

    /// Sinspawn Gui (Not with MRR skip)
    #[default = true]
    gui: bool,

    /// MRR Skip
    #[default = false]
    mrr_skip: bool,

    /// Djose Highroad
    #[default = true]
    djose_road: bool,

    /// Djose Trials
    #[default = false]
    ixion: bool,

    /// Entering Moonflow
    #[default = false]
    enter_moonflow: bool,

    /// Moonflow South
    #[default = true]
    moonflow_south: bool,

    /// Extractor
    #[default = true]
    extractor: bool,

    /// Guadosalam
    #[default = true]
    guadosalam: bool,

    /// Thunder Plains
    #[default = true]
    thunder_plains: bool,

    /// Macalnia Woods
    #[default = false]
    macalania_woods: bool,

    /// Spherimorph
    #[default = true]
    spherimorph: bool,

    /// Crawler
    #[default = true]
    crawler: bool,

    /// Seymour
    #[default = true]
    seymour: bool,

    /// Macalania Trials
    #[default = false]
    shiva: bool,

    /// Wendigo
    #[default = true]
    wendigo: bool,

    /// Bikanel
    #[default = false]
    bikanel: bool,

    /// Home
    #[default = true]
    home: bool,

    /// Evrae
    #[default = true]
    evrae: bool,

    /// Bevelle Guards
    #[default = true]
    guards: bool,

    /// Bevelle Trials
    #[default = true]
    bahamut: bool,

    /// Via Purifico
    #[default = false]
    via_purifico: bool,

    /// Isaaru
    #[default = true]
    isaaru: bool,

    /// Seymour Natus
    #[default = true]
    natus: bool,

    /// Calm Lands
    #[default = false]
    calm_lands: bool,

    /// Biran & Yenke
    #[default = true]
    biran_yenke: bool,

    /// Seymour Flux
    #[default = true]
    flux: bool,

    /// Sanctuary Keeper
    #[default = true]
    sanctuary_keeper: bool,

    /// Zanarkand
    #[default = false]
    zanarkand: bool,

    /// Zanarkand Trials (Tetris)
    #[default = false]
    tetris: bool,

    /// Spectral Keeper
    #[default = false]
    spectral_keeper: bool,

    /// Yunalesca
    #[default = true]
    yunalesca: bool,

    /// Sin Core
    #[default = true]
    core: bool,

    /// Overdrive Sin
    #[default = true]
    overdrive: bool,

    /// Seymour Omnis
    #[default = true]
    omnis: bool,

    /// The Nucleus
    #[default = false]
    eggs: bool,

    /// Braska's Final Aeon
    #[default = true]
    bfa: bool,

    /// Yu Yevon
    #[default = true]
    yu_yevon: bool,

    #[cfg(testing)]
    /// FOR TESTING: The rest of the settings is for testing things
    _test_heading: Heading,

    #[cfg(testing)]
    /// START: Start timer when loading a save
    #[default = false]
    start_on_load: bool,

    #[cfg(testing)]
    /// STARFT: Start timer when a battle starts
    #[default = false]
    start_on_battle: bool,

    #[cfg(testing)]
    /// START: Start timer at the beginning of Bevelle guards
    #[default = false]
    start_on_guards: bool,

    #[cfg(testing)]
    /// RESET: Reset timer when loading a regular save
    #[default = false]
    reset_on_load: bool,

    #[cfg(testing)]
    /// RESET: Reset timer when loading the auto save
    #[default = false]
    reset_on_autosave_load: bool,

    #[cfg(testing)]
    /// Sin's Fin
    #[default = false]
    sinfin: bool,

    #[cfg(testing)]
    /// Yuna gets kidnapped in Luca
    #[default = false]
    luca: bool,

    #[cfg(testing)]
    /// First Workers
    #[default = false]
    workers1: bool,

    #[cfg(testing)]
    /// Second Workers
    #[default = false]
    workers2: bool,

    #[cfg(testing)]
    /// Third Workers, wave 1
    #[default = false]
    workers31: bool,

    #[cfg(testing)]
    /// Third Workers, wave 2
    #[default = false]
    workers32: bool,

    #[cfg(testing)]
    /// Third Workers
    #[default = false]
    workers3: bool,

    #[cfg(testing)]
    /// Sahagins wave 1
    #[default = false]
    sahagins1: bool,

    #[cfg(testing)]
    /// Sahagins wave 2
    #[default = false]
    sahagins2: bool,

    #[cfg(testing)]
    /// Sahagin Chiefs
    #[default = false]
    sahagins: bool,

    #[cfg(testing)]
    /// Moonflow North
    #[default = false]
    moonflow_north: bool,

    #[cfg(testing)]
    /// O'aka shop
    #[default = false]
    oaka_shop: bool,

    #[cfg(testing)]
    /// Pre-Crawler grid
    #[default = false]
    crawler_grid: bool,

    #[cfg(testing)]
    /// Pre-Seymour grid
    #[default = false]
    seymour_grid: bool,

    #[cfg(testing)]
    /// Pre-Wendigo grid
    #[default = false]
    wendigo_grid: bool,

    #[cfg(testing)]
    /// Crevasse
    #[default = false]
    crevasse: bool,

    #[cfg(testing)]
    /// Bikanel party members joined
    #[default = false]
    bikanel_party: bool,

    #[cfg(testing)]
    /// Bikanel Sandragora yeeted
    #[default = false]
    bikanel_yeet: bool,

    #[cfg(testing)]
    /// Home Bombs
    #[default = false]
    bombs: bool,

    #[cfg(testing)]
    /// Home Dual Horns
    #[default = false]
    dual_horns: bool,

    #[cfg(testing)]
    /// Home Chimeras
    #[default = false]
    chimeras: bool,

    #[cfg(testing)]
    /// Bevelle Guards #1
    #[default = false]
    guards1: bool,

    #[cfg(testing)]
    /// Bevelle Guards #2
    #[default = false]
    guards2: bool,

    #[cfg(testing)]
    /// Bevelle Guards #3
    #[default = false]
    guards3: bool,

    #[cfg(testing)]
    /// Bevelle Guards #4
    #[default = false]
    guards4: bool,

    #[cfg(testing)]
    /// Evrae Altana
    #[default = false]
    altana: bool,

    #[cfg(testing)]
    /// Escaped Via Purifico Underwater
    #[default = false]
    via_underwater: bool,

    #[cfg(testing)]
    /// Defender
    #[default = false]
    defender: bool,
}

impl Settings {
    fn filter(&self, split_on: Splits) -> bool {
        let Settings {
            start: _,
            split: _,
            reset: _,
            remove_loads: _,
            count_encounters: _,
            _splits_heading1,
            _splits_heading2,
            _splits_heading3,
            ammes,
            klikk,
            tros,
            lagoon,
            besaid_village,
            kimahri,
            besaid_road,
            echuilles,
            geneaux,
            ifrit,
            kilika,
            oblitzerator,
            before_blitzball,
            blitzball_done,
            garuda,
            miihen_road,
            chocobo_eater,
            old_road,
            mrr,
            gui,
            mrr_skip,
            djose_road,
            ixion,
            enter_moonflow,
            moonflow_south,
            extractor,
            guadosalam,
            thunder_plains,
            macalania_woods,
            spherimorph,
            crawler,
            seymour,
            shiva,
            wendigo,
            bikanel,
            home,
            evrae,
            guards,
            bahamut,
            via_purifico,
            isaaru,
            natus,
            calm_lands,
            biran_yenke,
            flux,
            sanctuary_keeper,
            zanarkand,
            tetris,
            spectral_keeper,
            yunalesca,
            core,
            overdrive,
            omnis,
            eggs,
            bfa,
            yu_yevon,
            #[cfg(testing)]
            _test_heading,
            #[cfg(testing)]
                start_on_load: _,
            #[cfg(testing)]
                start_on_battle: _,
            #[cfg(testing)]
                start_on_guards: _,
            #[cfg(testing)]
                reset_on_load: _,
            #[cfg(testing)]
                reset_on_autosave_load: _,
            #[cfg(testing)]
            sinfin,
            #[cfg(testing)]
            luca,
            #[cfg(testing)]
            workers1,
            #[cfg(testing)]
            workers2,
            #[cfg(testing)]
            workers31,
            #[cfg(testing)]
            workers32,
            #[cfg(testing)]
            workers3,
            #[cfg(testing)]
            sahagins1,
            #[cfg(testing)]
            sahagins2,
            #[cfg(testing)]
            sahagins,
            #[cfg(testing)]
            moonflow_north,
            #[cfg(testing)]
            oaka_shop,
            #[cfg(testing)]
            crawler_grid,
            #[cfg(testing)]
            seymour_grid,
            #[cfg(testing)]
            wendigo_grid,
            #[cfg(testing)]
            crevasse,
            #[cfg(testing)]
            bikanel_party,
            #[cfg(testing)]
            bikanel_yeet,
            #[cfg(testing)]
            bombs,
            #[cfg(testing)]
            dual_horns,
            #[cfg(testing)]
            chimeras,
            #[cfg(testing)]
            guards1,
            #[cfg(testing)]
            guards2,
            #[cfg(testing)]
            guards3,
            #[cfg(testing)]
            guards4,
            #[cfg(testing)]
            altana,
            #[cfg(testing)]
            via_underwater,
            #[cfg(testing)]
            defender,
        } = self;

        return *match split_on {
            Splits::Ammes => ammes,
            Splits::Klikk => klikk,
            Splits::Tros => tros,
            Splits::Lagoon => lagoon,
            Splits::BesaidVillage => besaid_village,
            Splits::Kimahri => kimahri,
            Splits::BesaidRoad => besaid_road,
            Splits::Echuilles => echuilles,
            Splits::Geneaux => geneaux,
            Splits::Ifrit => ifrit,
            Splits::Kilika => kilika,
            Splits::Oblitzerator => oblitzerator,
            Splits::BeforeBlitzball => before_blitzball,
            Splits::BlitzballDone => blitzball_done,
            Splits::Garuda => garuda,
            Splits::MiihenRoad => miihen_road,
            Splits::ChocoboEater => chocobo_eater,
            Splits::OldRoad => old_road,
            Splits::Mrr => mrr,
            Splits::Gui => gui,
            Splits::MrrSkip => mrr_skip,
            Splits::DjoseRoad => djose_road,
            Splits::Ixion => ixion,
            Splits::EnterMoonflow => enter_moonflow,
            Splits::MoonflowSouth => moonflow_south,
            Splits::Extractor => extractor,
            Splits::Guadosalam => guadosalam,
            Splits::ThunderPlains => thunder_plains,
            Splits::MacalaniaWoods => macalania_woods,
            Splits::Spherimorph => spherimorph,
            Splits::Crawler => crawler,
            Splits::Seymour => seymour,
            Splits::Shiva => shiva,
            Splits::Wendigo => wendigo,
            Splits::Bikanel => bikanel,
            Splits::Home => home,
            Splits::Evrae => evrae,
            Splits::Guards => guards,
            Splits::Bahamut => bahamut,
            Splits::ViaPurifico => via_purifico,
            Splits::Isaaru => isaaru,
            Splits::Natus => natus,
            Splits::CalmLands => calm_lands,
            Splits::BiranYenke => biran_yenke,
            Splits::Flux => flux,
            Splits::SanctuaryKeeper => sanctuary_keeper,
            Splits::Zanarkand => zanarkand,
            Splits::Tetris => tetris,
            Splits::SpectralKeeper => spectral_keeper,
            Splits::Yunalesca => yunalesca,
            Splits::Core => core,
            Splits::Overdrive => overdrive,
            Splits::Omnis => omnis,
            Splits::Eggs => eggs,
            Splits::Bfa => bfa,
            Splits::YuYevon => yu_yevon,
            #[cfg(testing)]
            Splits::SinFin => sinfin,
            #[cfg(testing)]
            Splits::Ifrit => ifrit,
            #[cfg(testing)]
            Splits::Luca => luca,
            #[cfg(testing)]
            Splits::Workers1 => workers1,
            #[cfg(testing)]
            Splits::Workers2 => workers2,
            #[cfg(testing)]
            Splits::Workers31 => workers31,
            #[cfg(testing)]
            Splits::Workers32 => workers32,
            #[cfg(testing)]
            Splits::Workers3 => workers3,
            #[cfg(testing)]
            Splits::Sahagins1 => sahagins1,
            #[cfg(testing)]
            Splits::Sahagins2 => sahagins2,
            #[cfg(testing)]
            Splits::Sahagins => sahagins,
            #[cfg(testing)]
            Splits::Ixion => ixion,
            #[cfg(testing)]
            Splits::MoonflowNorth => moonflow_north,
            #[cfg(testing)]
            Splits::OakaShop => oaka_shop,
            #[cfg(testing)]
            Splits::CrawlerGrid => crawler_grid,
            #[cfg(testing)]
            Splits::SeymourGrid => seymour_grid,
            #[cfg(testing)]
            Splits::WendigoGrid => wendigo_grid,
            #[cfg(testing)]
            Splits::Shiva => shiva,
            #[cfg(testing)]
            Splits::Crevasse => crevasse,
            #[cfg(testing)]
            Splits::BikanelParty => bikanel_party,
            #[cfg(testing)]
            Splits::BikanelYeet => bikanel_yeet,
            #[cfg(testing)]
            Splits::Bombs => bombs,
            #[cfg(testing)]
            Splits::DualHorns => dual_horns,
            #[cfg(testing)]
            Splits::Chimeras => chimeras,
            #[cfg(testing)]
            Splits::Guards1 => guards1,
            #[cfg(testing)]
            Splits::Guards2 => guards2,
            #[cfg(testing)]
            Splits::Guards3 => guards3,
            #[cfg(testing)]
            Splits::Guards4 => guards4,
            #[cfg(testing)]
            Splits::Altana => altana,
            #[cfg(testing)]
            Splits::ViaUnderwater => via_underwater,
            #[cfg(testing)]
            Splits::Defender => defender,
            #[cfg(testing)]
            Splits::SpectralKeeper => spectral_keeper,
        };
    }
}

struct Running {
    splits: SeenSplits,
    watchers: Watchers,
    #[cfg(testing)]
    workers3: u32,
    #[cfg(testing)]
    sahagins: u32,
    #[cfg(testing)]
    guards: u32,
}

struct NotRunning {
    watchers: Watchers,
    loading_frame_buffer: u32,
}

impl NotRunning {
    fn new() -> Self {
        Self {
            watchers: Watchers::new(),
            loading_frame_buffer: 0,
        }
    }
}

enum Timer {
    Running(Running),
    NotRunning(NotRunning),
}

impl Timer {
    fn new() -> Self {
        return Self::NotRunning(NotRunning::new());
    }

    fn get_or_start(&mut self) -> &mut Running {
        match self {
            Self::Running(running) => running,
            Self::NotRunning(_) => {
                let running = Running {
                    splits: SeenSplits::empty(),
                    watchers: Watchers::new(),
                    #[cfg(testing)]
                    workers3: 1,
                    #[cfg(testing)]
                    sahagins: 0,
                    #[cfg(testing)]
                    guards: 1,
                };
                *self = Self::Running(running);
                let Self::Running(running) = self else {
                    unreachable!()
                };
                running
            }
        }
    }

    fn stop(&mut self) -> &mut NotRunning {
        match self {
            Self::Running(_) => {
                let not_running = NotRunning::new();
                *self = Self::NotRunning(not_running);
                let Self::NotRunning(not_running) = self else {
                    unreachable!();
                };
                not_running
            }
            Self::NotRunning(not_running) => not_running,
        }
    }
}

struct Game {
    // option because of borrowck shenanigans
    process: Option<Process>,
    memory: Memory,
}

struct State<'s> {
    settings: &'s mut Settings,
    timer: Timer,
    game: Option<Game>,
}

async fn main() {
    asr::set_tick_rate(30.0);

    let mut settings = {
        let mut s = Settings::register();
        s.update();
        log!("Loaded settings: {:?}", s);
        s
    };

    let mut state = State {
        settings: &mut settings,
        timer: Timer::new(),
        game: None,
    };

    loop {
        retry(|| state.try_connect()).await;
        state.connected_loop().await;
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Nothing,
    Start,
    StartPaused,
    Reset,
    Split(Splits),
}

impl Action {
    fn send(self) {
        match self {
            Self::Nothing => {}
            Self::Start => {
                log!("Timer START!");
                timer::start();
            }
            Self::StartPaused => {
                log!("Timer START!");
                timer::start();
                timer::pause_game_time();
            }
            Self::Reset => {
                log!("Timer RESET!");
                timer::reset();
            }
            Self::Split(splits) => {
                log!("SPLIT! {:?}", splits);
                timer::split();
            }
        }
    }
}

impl State<'_> {
    fn try_connect(&mut self) -> Option<()> {
        if self.game.is_none() {
            if let Some(base_address) = find_process() {
                log!("attached to process");
                let memory = Memory::new(&base_address);
                let game = Game {
                    process: Some(base_address.process),
                    memory,
                };
                self.game = Some(game);
            }
        }
        return self.game.as_ref().map(|_| ());
    }

    async fn connected_loop(&mut self) {
        let Some(mut game) = self.game.take() else {
            return;
        };
        let Some(process) = game.process.take() else {
            return;
        };
        process
            .until_closes(self.main_loop(&process, &game.memory))
            .await
            .unwrap_or_default();

        #[cfg(testing)]
        if let Timer::Running(ref mut running) = self.timer {
            running.workers3 = 1;
            running.sahagins = 0;
            running.guards = 1;
        }
    }

    async fn main_loop(&mut self, process: &Process, memory: &Memory) {
        loop {
            let action = self.tick(process, memory);
            action.send();
            next_tick().await;
            self.settings.update();
        }
    }

    fn tick(&mut self, process: &Process, memory: &Memory) -> Action {
        let timer_state = timer::state();
        match timer_state {
            TimerState::Running | TimerState::Paused => {
                let running = self.timer.get_or_start();
                return running.update_game(self.settings, process, memory);
            }
            TimerState::NotRunning | TimerState::Ended => {
                let not_running = self.timer.stop();
                return not_running.update_game(self.settings, process, memory);
            }
            otherwise => {
                log!("Unexpected timer state: {:?}", otherwise);
                return Action::Nothing;
            }
        }
    }
}

impl NotRunning {
    fn update_game(&mut self, settings: &Settings, process: &Process, memory: &Memory) -> Action {
        let mut read = Read::new(&mut self.watchers, process, memory);

        #[cfg(debugging)]
        {
            let _ = *read.loading();
            let _ = *read.level();
            let _ = *read.select_screen();
            let _ = *read.cursor_position();
            let _ = *read.input();
            let _ = *read.story_progression();
            let _ = *read.loading_slot();
            let _ = *read.battle_state();
            let _ = *read.map_id();
            let _ = *read.formation_id();
            read.watchers.dump_all_vars();
        }

        let level = read.level();
        if level.new_game() {
            self.loading_frame_buffer = self.loading_frame_buffer.saturating_sub(1);

            if settings.reset {
                let select_screen = read.select_screen();
                if select_screen.changed_to(&6) {
                    return Action::Reset;
                }
            }

            if settings.start {
                let select_screen = read.select_screen();
                if (7..=8).contains(&select_screen.current) {
                    let cursor_position = read.cursor_position();
                    if (cursor_position.current >> 16) & 0xFF == 0 {
                        let input = read.input();
                        if input.confirm_pressed() {
                            return Action::Start;
                        }
                    }
                }
            }

            #[cfg(testing)]
            if settings.reset_on_load || settings.reset_on_autosave_load {
                let loading = read.loading();
                if loading.on_loading_screen() {
                    let input = read.input();
                    if input.changed() && input.confirm_pressed() {
                        let slot = read.loading_slot();
                        let is_autosave = slot.current == 0;
                        log!("{}ave loaded!", if is_autosave { "Autos" } else { "S" });
                        if settings.reset_on_autosave_load && is_autosave {
                            return Action::Reset;
                        }
                        if settings.reset_on_load && is_autosave == false {
                            return Action::Reset;
                        }
                        log!("Not resetting");
                    }
                }
            }

            #[cfg(testing)]
            if settings.start_on_load {
                let loading = read.loading();
                if loading.old.on_loading_screen() && loading.current.not_loading() {
                    // 10 frames to allow for loading to start
                    self.loading_frame_buffer = 10;
                }
            }
        } else if self.loading_frame_buffer > 0 {
            if read.loading().is_loading() {
                log!("Save loaded");
                return Action::StartPaused;
            }
        } else {
            #[cfg(testing)]
            if settings.start_on_battle || settings.start_on_guards {
                let story = *read.story_progression();

                if settings.start_on_battle && story.current.0 > 0 && read.map_id().changed() {
                    log!("Battle Start!");
                    return Action::Start;
                }

                if settings.start_on_guards && story.changed_to(&Progress(2080)) {
                    log!("Bevelle Guards test");
                    return Action::Start;
                }
            }
        }

        return Action::Nothing;
    }
}

impl Running {
    fn update_game(&mut self, settings: &Settings, process: &Process, memory: &Memory) -> Action {
        match self.find_split(settings, process, memory) {
            ControlFlow::Break(split) => self
                .try_split(settings, split)
                .map_or(Action::Nothing, Action::Split),
            ControlFlow::Continue(action) => action,
        }
    }

    fn find_split(&mut self, settings: &Settings, process: &Process, memory: &Memory) -> Splitter {
        let mut read = Read::new(&mut self.watchers, process, memory);

        #[cfg(debugging)]
        {
            let _ = *read.loading();
            let _ = *read.encounter_count();
            let _ = *read.level();
            let _ = *read.story_progression();
            let _ = *read.battle_state();
            let _ = *read.cutscene_type();
            let _ = *read.map_id();
            let _ = *read.formation_id();
            let _ = *read.yu_yevon();
            let _ = *read.hp_enemy_a();
            let _ = *read.loading_slot();
            read.watchers.dump_all_vars();
            timer::set_variable_int("workers3", self.workers3);
            timer::set_variable_int("sahagins", self.sahagins);
            timer::set_variable_int("guards", self.guards);
        }

        if settings.remove_loads {
            let loading = read.loading();
            if loading.changed() {
                if loading.is_loading() {
                    timer::pause_game_time();
                } else if loading.old.is_loading() {
                    timer::resume_game_time();
                }
            }
        }

        if settings.count_encounters {
            let encounters = read.encounter_count();
            if encounters.changed() {
                timer::set_variable_int("encounter_count", encounters.current);
            }
        }

        if settings.split == false {
            return NO_SPLIT;
        }

        let level = *read.level();
        level.split(level.old, &mut read)?;

        let battle_state = *read.battle_state();

        if battle_state.changed() {
            read.story_progression()
                .split_battle(battle_state, &mut read)?;
        }

        let story_progress = *read.story_progression();
        story_progress.split_advance(story_progress.old, &mut read)?;

        let yu_yevon = read.yu_yevon();
        if yu_yevon.changed_to(&1) {
            story_progress.split_yu_yevon(&mut read)?;
        }

        if settings.reset {
            if level.new_game() {
                let select_screen = read.select_screen();
                if select_screen.changed_to(&6) {
                    return RESET;
                }
            }
        }

        #[cfg(testing)]
        story_progress.split_workers(battle_state, &mut read)?;
        #[cfg(testing)]
        story_progress.split_sahagins(battle_state, &mut read)?;

        #[cfg(testing)]
        if settings.reset_on_load || settings.reset_on_autosave_load {
            let level = read.level();
            if level.new_game() {
                let loading = read.loading();
                if loading.on_loading_screen() {
                    let input = read.input();
                    if input.changed() && input.confirm_pressed() {
                        let slot = read.loading_slot();
                        let is_autosave = slot.current == 0;
                        log!("{}ave loaded!", if is_autosave { "Autos" } else { "S" });
                        if settings.reset_on_autosave_load && is_autosave {
                            return RESET;
                        }
                        if settings.reset_on_load && is_autosave == false {
                            return RESET;
                        }
                        log!("Not resetting");
                    }
                }
            }
        }

        return NO_SPLIT;
    }

    fn try_split(&mut self, settings: &Settings, split: Splits) -> Option<Splits> {
        log!("Potential split: {:?}", split);

        #[cfg(testing)]
        let split = match self.map_split(split) {
            Ok(s) => {
                log!("Fixed split from {:?} to {:?}", split, s);
                s
            }
            Err(UseSplit::Keep) => split,
            Err(UseSplit::Ignore) => return None,
        };

        if settings.filter(split) == false {
            log!("Ignoring disabled split: {:?}", split);
            return None;
        }

        if self.splits.insert(&split) == false {
            log!("Ignoring duplicated split: {:?}", split);
            return None;
        }

        return Some(split);
    }

    #[cfg(testing)]
    fn map_split(&mut self, split: Splits) -> Result<Splits, UseSplit> {
        Ok(match split {
            Splits::Workers31 => {
                if self.workers3 == 2 {
                    self.workers3 = 3;
                    Splits::Workers32
                } else {
                    if self.workers3 != 1 {
                        log!(
                            "Unexpected workers3 value: {}, fixing to workers3=1",
                            self.workers3
                        );
                    }
                    self.workers3 = 2;
                    return Err(UseSplit::Keep);
                }
            }
            Splits::Sahagins1 => {
                self.sahagins += 1;
                match self.sahagins {
                    3 => return Err(UseSplit::Keep),
                    9 => Splits::Sahagins2,
                    _ => return Err(UseSplit::Ignore),
                }
            }
            Splits::Guards1 => {
                if self.guards == 3 {
                    self.guards = 4;
                    Splits::Guards3
                } else {
                    if self.guards != 1 {
                        log!(
                            "Unexpected guards value: {}, fixing to guards=1",
                            self.guards
                        );
                    }
                    self.guards = 2;
                    return Err(UseSplit::Keep);
                }
            }
            Splits::Guards2 => {
                if self.guards == 4 {
                    self.guards = 5;
                    Splits::Guards4
                } else {
                    if self.guards != 2 {
                        log!(
                            "Unexpected guards value: {}, fixing to guards=2",
                            self.guards
                        );
                    }
                    self.guards = 3;
                    return Err(UseSplit::Keep);
                }
            }
            Splits::Guards => {
                if self.guards != 5 {
                    log!(
                        "Unexpected guards value: {}, fixing to guards=5",
                        self.guards
                    );
                }
                self.guards = 6;
                return Err(UseSplit::Keep);
            }
            _ => return Err(UseSplit::Keep),
        })
    }
}

#[cfg(testing)]
#[derive(Debug, Copy, Clone)]
enum UseSplit {
    Keep,
    Ignore,
}

fn find_process() -> Option<BaseAddress> {
    let process = Process::attach("FFX.exe")?;
    let start = find_entry_point(&process)?;
    log!("Found main module at {}", start);
    return Some(BaseAddress { process, start });
}

fn find_entry_point(process: &Process) -> Option<Address> {
    let main_module = process.get_module_range("FFX.exe").ok()?;
    let sig = Signature::<8>::new("58 0E 00 00 E9 00 00 00");

    let ranges = process.memory_ranges();
    let ranges = iter::once(main_module).chain(ranges.filter_map(|r| r.range().ok()));

    for module in ranges {
        if let Some(entry_point) = sig.scan_process_range(process, module) {
            log!(
                "Potential main module at {} with size {}",
                module.0,
                module.1
            );
            let entry_point = entry_point.value().saturating_sub(module.0.value());
            if entry_point == 0x5493c8 {
                return Some(module.0);
            }
        }
    }

    return None;
}

struct BaseAddress {
    process: Process,
    start: Address,
}

type Splitter = ControlFlow<Splits, Action>;
const NO_SPLIT: Splitter = ControlFlow::Continue(Action::Nothing);
const RESET: Splitter = ControlFlow::Continue(Action::Reset);

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
struct Loading(u32);

impl Loading {
    #[cfg(testing)]
    const SELECTION: u32 = 1;
    const LOADING: u32 = 2;

    #[cfg(testing)]
    fn not_loading(self) -> bool {
        return self.0 == 0;
    }

    fn is_loading(self) -> bool {
        return self.0 == Self::LOADING;
    }

    #[cfg(testing)]
    fn on_loading_screen(self) -> bool {
        return self.0 == Self::SELECTION;
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
struct Input(u32);

impl Input {
    const CONFIRM: u32 = 32;

    fn confirm_pressed(self) -> bool {
        self.0 & Self::CONFIRM == Self::CONFIRM
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
struct Level(u32);

impl Level {
    const BESAID_VILLAGE: u32 = 17;
    const KILIKA_WOODS: u32 = 18;
    const BESAID_BEACH: u32 = 20;
    const BESAID_END_ROAD: u32 = 22;
    const NEW_GAME: u32 = 23;
    const LAGOON: u32 = 41;
    const KILIKA_FAYTH: u32 = 45;
    const KILIKA_RESIDENTIAL_AREA: u32 = 46;
    const HIGHROAD_AGENCY: u32 = 58;
    const HIGHROAD_NORTH_END: u32 = 59;
    const BESAID_PROMONTORY: u32 = 67;
    const BESAID_VILLAGE_ROAD: u32 = 69;
    const MOONFLOW_SOUTH_BANK_ROAD: u32 = 75;
    const DJOSE_PILGRIMAGE_ROAD: u32 = 76;
    const KILIKA_TEMPLE: u32 = 78;
    const MUSHROOM_ROCK_ROAD: u32 = 79;
    #[cfg(testing)]
    const MACALNIA_ANTECHAMBER: u32 = 80;
    const DJOSE_OUTSIDE: u32 = 82;
    #[cfg(testing)]
    const LUCA_DOCK_1: u32 = 85;
    const LUCA_DOCK_5: u32 = 89;
    const DJOSE_FAYTH: u32 = 90;
    const DJOSE_HIGHROAD: u32 = 93;
    #[cfg(testing)]
    const MOONFLOW_NORTH: u32 = 97;
    #[cfg(testing)]
    const MACALNIA_LAKE: u32 = 102;
    const MOONFLOW_SOUTH_BANK: u32 = 105;
    const MACALANIA_TEMPLE: u32 = 106;
    const MACALANIA_WOODS_SOUTH: u32 = 110;
    const LUCA_MAIN: u32 = 123;
    const HIGHROAD_CENTRAL: u32 = 127;
    const HOME_ENTRANCE: u32 = 130;
    const MUSHROOM_ROCK_AFTERMATH: u32 = 131;
    const GUADOSALAM: u32 = 135;
    const BIKANEL_NORTH: u32 = 138;
    const THUNDERPLAINS_SOUTH: u32 = 140;
    const MACALANIA_TEMPLE_ROAD: u32 = 153;
    const THUNDERPLAINS_NORTH: u32 = 162;
    #[cfg(testing)]
    const MACALNIA_LAKE_SHOP: u32 = 164;
    #[cfg(testing)]
    const CREVASSE: u32 = 192;
    #[cfg(testing)]
    const HIGHBRIDGE: u32 = 208;
    #[cfg(testing)]
    const VIA_UNDERWATER: u32 = 209;
    const STADIUM_POOL: u32 = 212;
    const HOME_ENVIRONMENT_CONTROLS: u32 = 219;
    const MACALANIA_SPRING: u32 = 221;
    const ZANARKAND_ROAD: u32 = 222;
    const CALM_LANDS: u32 = 223;
    const BEVELLE_ANTECHAMBER: u32 = 226;
    #[cfg(testing)]
    const MACALNIA_HALLWAY: u32 = 239;
    const MACALANIA_WOODS_NORTH: u32 = 242;
    #[cfg(testing)]
    const MACALANIA_SPHERIMORPH: u32 = 248;
    const STADIUM_STANDS: u32 = 250;
    const CALM_LANDS_BRIDGE: u32 = 279;
    const HOME_MAIN_CORRIDOR: u32 = 280;
    const BEVELLE_TRIALS: u32 = 306;
    const ZANARKAND_DOME: u32 = 316;
    const NUCLEUS: u32 = 324;
    const DREAMS_END: u32 = 325;

    fn new_game(self) -> bool {
        self.0 == Self::NEW_GAME
    }

    fn split(self, old: Self, read: &mut Read<'_>) -> Splitter {
        if self == old {
            return NO_SPLIT;
        }
        ControlFlow::Break(match (old.0, self.0) {
            (Self::LAGOON, Self::BESAID_PROMONTORY | Self::BESAID_VILLAGE_ROAD) => Splits::Lagoon, // story 119->124
            (Self::BESAID_VILLAGE, Self::BESAID_VILLAGE_ROAD) => Splits::BesaidVillage, // story == 200
            (Self::BESAID_END_ROAD, Self::BESAID_BEACH) => Splits::BesaidRoad, // story == 217
            (Self::KILIKA_FAYTH, Self::KILIKA_TEMPLE) => Splits::Ifrit,        // story 346 -> 348
            (Self::KILIKA_WOODS, Self::KILIKA_RESIDENTIAL_AREA) => Splits::Kilika,
            #[cfg(testing)]
            (Self::LUCA_MAIN, Self::LUCA_DOCK_1) if read.is_at(Progress::WORKERS) => Splits::Luca,
            (Self::LUCA_DOCK_5, Self::LUCA_MAIN) if read.is_at(Progress::BLITZBALL) => {
                Splits::BeforeBlitzball
            }
            (Self::STADIUM_POOL, Self::STADIUM_STANDS) => Splits::BlitzballDone,
            (Self::HIGHROAD_CENTRAL, Self::HIGHROAD_AGENCY) => Splits::MiihenRoad,
            (Self::HIGHROAD_NORTH_END, Self::MUSHROOM_ROCK_ROAD) => Splits::OldRoad,
            (Self::MUSHROOM_ROCK_ROAD, Self::MUSHROOM_ROCK_AFTERMATH) => Splits::MrrSkip,
            (Self::DJOSE_HIGHROAD, Self::DJOSE_PILGRIMAGE_ROAD) => Splits::DjoseRoad,
            (Self::DJOSE_FAYTH, Self::DJOSE_OUTSIDE) => Splits::Ixion, // story == 998
            (Self::DJOSE_HIGHROAD, Self::MOONFLOW_SOUTH_BANK_ROAD) => Splits::EnterMoonflow, // story == 1030
            (Self::MOONFLOW_SOUTH_BANK_ROAD, Self::MOONFLOW_SOUTH_BANK) => Splits::MoonflowSouth,
            #[cfg(testing)]
            (Self::MOONFLOW_NORTH, Self::GUADOSALAM) => Splits::MoonflowNorth,
            (Self::GUADOSALAM, Self::THUNDERPLAINS_SOUTH) => Splits::Guadosalam,
            (Self::THUNDERPLAINS_NORTH, Self::MACALANIA_WOODS_SOUTH) => Splits::ThunderPlains,
            (Self::MACALANIA_WOODS_NORTH, Self::MACALANIA_SPRING) => Splits::MacalaniaWoods, // story == 1413
            #[cfg(testing)]
            (Self::MACALANIA_SPRING, Self::MACALANIA_SPHERIMORPH) => Splits::OakaShop,
            #[cfg(testing)]
            (Self::MACALANIA_SPRING, Self::MACALNIA_LAKE_SHOP) => Splits::CrawlerGrid, // story == 1470
            #[cfg(testing)]
            (Self::MACALANIA_TEMPLE_ROAD, Self::MACALANIA_TEMPLE) => Splits::SeymourGrid, // story == 1504
            #[cfg(testing)]
            (Self::MACALNIA_ANTECHAMBER, Self::MACALNIA_HALLWAY) => Splits::WendigoGrid,
            (Self::MACALANIA_TEMPLE, Self::MACALANIA_TEMPLE_ROAD) => Splits::Shiva, // story == 1557
            #[cfg(testing)]
            (Self::CREVASSE, Self::MACALNIA_LAKE) => Splits::Crevasse,
            (Self::BIKANEL_NORTH, Self::HOME_ENTRANCE) => Splits::Bikanel, // story 1720 -> 1800
            (Self::HOME_MAIN_CORRIDOR, Self::HOME_ENVIRONMENT_CONTROLS) => Splits::Home, // story = 1940
            (Self::BEVELLE_TRIALS, Self::BEVELLE_ANTECHAMBER) => Splits::Bahamut,
            #[cfg(testing)]
            (Self::VIA_UNDERWATER, Self::HIGHBRIDGE) => Splits::ViaUnderwater, // story = 2220,
            (Self::CALM_LANDS, Self::CALM_LANDS_BRIDGE) => Splits::CalmLands, // story = 2400
            (Self::ZANARKAND_ROAD, Self::ZANARKAND_DOME) => Splits::Zanarkand, // stgory = 2767
            (Self::NUCLEUS, Self::DREAMS_END) => Splits::Eggs,                // story == 3260
            _ => return NO_SPLIT,
        })
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
struct BattleState(u32);

#[allow(unused)]
impl BattleState {
    const ONGOING: u32 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
    const IS_OVER: u32 = 0b0000_0000_0000_0000_0000_0010_0000_0000;
    const ESCAPED: u32 = 0b0000_0000_0000_0000_0000_0001_0000_0000;
    const FANFARE: u32 = 0b0000_0000_0000_0001_0000_0000_0000_0000;
    const CHAINED: u32 = 0b0000_0001_0000_0000_0000_0000_0000_0000;

    const fn in_battle(self) -> bool {
        self.0 & Self::ONGOING == Self::ONGOING
    }

    const fn escaped(self) -> bool {
        self.0 & Self::ESCAPED == Self::ESCAPED
    }

    const fn is_over(self) -> bool {
        self.0 & Self::IS_OVER == Self::IS_OVER
    }

    const fn fanfare(self) -> bool {
        self.0 & Self::FANFARE == Self::FANFARE
    }

    const fn chained(self) -> bool {
        self.0 & Self::CHAINED == Self::CHAINED
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(transparent)]
struct Progress(u32);

impl Progress {
    const AMMES: u32 = 15;
    const KLIKK: u32 = 55;
    const TROS: u32 = 76;
    const LAGOON: u32 = 119;
    const KIMAHRI: u32 = 214;
    #[cfg(testing)]
    const SINFIN: u32 = 272;
    const ECHUILLES: u32 = 280;
    const GENEAUX: u32 = 322;
    #[cfg(testing)]
    const WORKERS: u32 = 492;
    const OBLITZERATOR: u32 = 502;
    const BLITZBALL: u32 = 514;
    #[cfg(testing)]
    const SAHAGINS: u32 = 583;
    const GARUDA: u32 = 600;
    const CHOCOBO_EATER: u32 = 770;
    const MRR: u32 = 835;
    const GUI: u32 = 865;
    const EXTRACTOR: u32 = 1060;
    const SPHERIMORPH: u32 = 1420;
    const CRAWLER: u32 = 1485;
    const SEYMOUR: u32 = 1540;
    const WENDIGO: u32 = 1570;
    #[cfg(testing)]
    const BIKANEL_KIMAHRI: u32 = 1718;
    #[cfg(testing)]
    const BIKANEL_RIKKU: u32 = 1720;
    #[cfg(testing)]
    const HOME: u32 = 1820;
    #[cfg(testing)]
    const HOME2: u32 = 1885;
    const EVRAE: u32 = 2040;
    const GUARDS: u32 = 2080;
    const ISAARU: u32 = 2220;
    const NATUS: u32 = 2280;
    #[cfg(testing)]
    const DEFENDER: u32 = 2400;
    const BIRAN_YENKE: u32 = 2510;
    const FLUX: u32 = 2530;
    const SANCTUARY_KEEPER: u32 = 2585;
    const TETRIS: u32 = 2767;
    const SPECTRAL_KEEPER: u32 = 2775;
    const YUNALESCA: u32 = 2815;
    const SIN_CORE: u32 = 3105;
    const OVERDRIVE_SIN: u32 = 3135;
    const OMNIS: u32 = 3205;
    const BFA: u32 = 3300;
    const YU_YEVON: u32 = 3380;

    fn split_battle(self, battle_state: Pair<BattleState>, read: &mut Read<'_>) -> Splitter {
        if battle_state.is_over() == false {
            return NO_SPLIT;
        }

        if battle_state.escaped() && self.0 != Self::CHOCOBO_EATER {
            return NO_SPLIT;
        }

        let mut is_encounter = |map_id, id1, id2| -> bool {
            return Self::is_encounter(read, map_id, id1, id2);
        };

        ControlFlow::Break(match self.0 {
            Self::AMMES => Splits::Ammes,
            Self::KLIKK if battle_state.fanfare() => Splits::Klikk,
            Self::TROS if is_encounter(4, 0, 0) => Splits::Tros,
            Self::KIMAHRI => Splits::Kimahri,
            #[cfg(testing)]
            Self::SINFIN => Splits::SinFin,
            Self::ECHUILLES => Splits::Echuilles,
            Self::GENEAUX => Splits::Geneaux,
            #[cfg(testing)]
            Self::WORKERS if is_encounter(12, 0, 0) => Splits::Workers1,
            #[cfg(testing)]
            Self::WORKERS if is_encounter(13, 0, 0) => Splits::Workers2,
            #[cfg(testing)]
            Self::WORKERS if is_encounter(14, 0, 0) => Splits::Workers3,
            Self::OBLITZERATOR => Splits::Oblitzerator,
            #[cfg(testing)]
            Self::SAHAGINS => Splits::Sahagins,
            Self::GARUDA if is_encounter(17, 0, 1) => Splits::Garuda,
            Self::CHOCOBO_EATER => Splits::ChocoboEater,
            Self::GUI => Splits::Gui,
            Self::EXTRACTOR => Splits::Extractor,
            Self::SPHERIMORPH => Splits::Spherimorph,
            Self::CRAWLER => Splits::Crawler,
            Self::SEYMOUR => Splits::Seymour,
            Self::WENDIGO if is_encounter(44, 0, 1) => Splits::Wendigo,
            #[cfg(testing)]
            Self::BIKANEL_RIKKU if is_encounter(48, 2, 0) => Splits::BikanelYeet,
            #[cfg(testing)]
            Self::HOME if is_encounter(87, 0, 0) => Splits::Bombs,
            #[cfg(testing)]
            Self::HOME if is_encounter(87, 0, 2) => Splits::DualHorns,
            #[cfg(testing)]
            Self::HOME2 if is_encounter(87, 0, 3) => Splits::Chimeras,
            Self::EVRAE => Splits::Evrae,
            #[cfg(testing)]
            Self::GUARDS if is_encounter(53, 0, 0) => Splits::Guards1, // story = 2080
            #[cfg(testing)]
            Self::GUARDS if is_encounter(53, 0, 1) => Splits::Guards2,
            Self::GUARDS if is_encounter(53, 0, 2) => Splits::Guards,
            Self::ISAARU if is_encounter(54, 2, 2) => Splits::Isaaru,
            #[cfg(testing)]
            Self::ISAARU if is_encounter(55, 1, 0) => Splits::Altana,
            Self::NATUS => Splits::Natus,
            #[cfg(testing)]
            Self::DEFENDER if is_encounter(61, 0, 0) => Splits::Defender,
            Self::BIRAN_YENKE if battle_state.fanfare() => Splits::BiranYenke,
            Self::FLUX if is_encounter(65, 0, 0) => Splits::Flux,
            Self::SANCTUARY_KEEPER if is_encounter(68, 0, 0) => Splits::SanctuaryKeeper,
            Self::SPECTRAL_KEEPER => Splits::SpectralKeeper,
            Self::YUNALESCA => Splits::Yunalesca,
            Self::SIN_CORE => Splits::Core,
            Self::OVERDRIVE_SIN => Splits::Overdrive,
            Self::OMNIS if is_encounter(78, 0, 0) => Splits::Omnis,
            Self::BFA => Splits::Bfa,
            _ => return NO_SPLIT,
        })
    }

    fn split_advance(self, old: Self, read: &mut Read<'_>) -> Splitter {
        if self.0 == Self::ISAARU {
            if read.cutscene_type().changed_from_to(&18, &257) {
                if Self::is_encounter(read, 54, 1, 0) {
                    return ControlFlow::Break(Splits::ViaPurifico);
                }
            }
        }

        if self <= old {
            return NO_SPLIT;
        }

        ControlFlow::Break(match old.0 {
            Self::LAGOON if read.cutscene_type().either(73) => Splits::Lagoon,
            Self::MRR if read.cutscene_type().either(940) => Splits::Mrr,
            #[cfg(testing)]
            Self::BIKANEL_KIMAHRI if self.0 == Self::BIKANEL_RIKKU => Splits::BikanelParty,
            Self::TETRIS if self.0 == Self::SPECTRAL_KEEPER => Splits::Tetris,
            _ => return NO_SPLIT,
        })
    }

    fn split_yu_yevon(self, read: &mut Read<'_>) -> Splitter {
        if self.0 == Self::YU_YEVON {
            if read.hp_enemy_a().current == 0 {
                return ControlFlow::Break(Splits::YuYevon);
            }
        }
        NO_SPLIT
    }

    fn is_encounter(read: &mut Read<'_>, map_id: u16, id1: u8, id2: u8) -> bool {
        return read.map_id().current == map_id && read.formation_id().is(id1, id2);
    }

    fn is(self, progress: u32) -> bool {
        return self.0 == progress;
    }

    #[cfg(testing)]
    fn split_workers(self, battle_state: Pair<BattleState>, read: &mut Read<'_>) -> Splitter {
        if self.0 == Self::WORKERS
            && battle_state.is_over() == false
            && read.map_id().current == 14
            && read.cutscene_type().current == 27
            && read.formation_id().is(0, 0)
            && read.hp_enemy_a().changed_from_to(&0, &300)
        {
            return ControlFlow::Break(Splits::Workers31);
        }

        return NO_SPLIT;
    }

    #[cfg(testing)]
    fn split_sahagins(self, battle_state: Pair<BattleState>, read: &mut Read<'_>) -> Splitter {
        if self.0 == Self::SAHAGINS
            && battle_state.is_over() == false
            && read.map_id().current == 16
            && read.cutscene_type().current == 24
        {
            if read.hp_enemy_a().changed_to(&0)
                || read.hp_enemy_b().changed_to(&0)
                || read.hp_enemy_c().changed_to(&0)
                || read.hp_enemy_d().changed_to(&0)
            {
                return ControlFlow::Break(Splits::Sahagins1);
            }
        }

        return NO_SPLIT;
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
struct Formation(u16);

impl Formation {
    const fn is(self, id1: u8, id2: u8) -> bool {
        let value = u16::from_ne_bytes([id1, id2]);
        return self.0 == value;
    }
}

#[derive(CheckedBitPattern, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct Hp(u32);

impl Default for Hp {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

struct Memory {
    is_loading: DeepPointer<2>,
    encounter_counter: DeepPointer<1>,
    current_level: DeepPointer<1>,
    story_progression: DeepPointer<1>,
    battle_state: DeepPointer<2>,
    cutscene_type: DeepPointer<1>,
    map_id: DeepPointer<1>,
    formation_id: DeepPointer<1>,
    yu_yevon: DeepPointer<1>,
    hp_enemy_a: DeepPointer<2>,
    cursor_position: DeepPointer<1>,
    input: DeepPointer<1>,
    select_screen: DeepPointer<1>,
    #[cfg(any(testing, debugging))]
    loading_slot: DeepPointer<1>,
    #[cfg(any(testing, debugging))]
    hp_enemy_b: DeepPointer<2>,
    #[cfg(any(testing, debugging))]
    hp_enemy_c: DeepPointer<2>,
    #[cfg(any(testing, debugging))]
    hp_enemy_d: DeepPointer<2>,
}

impl Memory {
    fn new(base: &BaseAddress) -> Memory {
        return Memory {
            is_loading: DeepPointer::new_32bit(base.start, &[0x8CC898, 0x123A4]),
            encounter_counter: DeepPointer::new_32bit(base.start, &[0xD307A4]),
            current_level: DeepPointer::new_32bit(base.start, &[0x8CB990]),
            story_progression: DeepPointer::new_32bit(base.start, &[0x84949C]),
            battle_state: DeepPointer::new_32bit(base.start, &[0x390D90, 0x4]),
            cutscene_type: DeepPointer::new_32bit(base.start, &[0xD27C88]),
            map_id: DeepPointer::new_32bit(base.start, &[0xD2C256]),
            formation_id: DeepPointer::new_32bit(base.start, &[0xD2C258]),
            yu_yevon: DeepPointer::new_32bit(base.start, &[0xD2A8E8]),
            hp_enemy_a: DeepPointer::new_32bit(base.start, &[0xD34460, 0x5D0]),
            cursor_position: DeepPointer::new_32bit(base.start, &[0x1467808]),
            input: DeepPointer::new_32bit(base.start, &[0x8CB170]),
            select_screen: DeepPointer::new_32bit(base.start, &[0xF25B30]),
            #[cfg(any(testing, debugging))]
            loading_slot: DeepPointer::new_32bit(base.start, &[0x8E72DC]),
            #[cfg(any(testing, debugging))]
            hp_enemy_b: DeepPointer::new_32bit(base.start, &[0xD34460, 0x1560]),
            #[cfg(any(testing, debugging))]
            hp_enemy_c: DeepPointer::new_32bit(base.start, &[0xD34460, 0x24F0]),
            #[cfg(any(testing, debugging))]
            hp_enemy_d: DeepPointer::new_32bit(base.start, &[0xD34460, 0x3480]),
        };
    }
}

struct Watch<T>(Watcher<T>);

impl<T> Watch<T> {
    fn new() -> Self
    where
        T: Default,
    {
        Self(Watcher {
            pair: Some(Pair {
                old: T::default(),
                current: T::default(),
            }),
        })
    }

    fn update<const N: usize>(&mut self, process: &Process, memory: &DeepPointer<N>) -> &Pair<T>
    where
        T: Clone + CheckedBitPattern,
    {
        if let Ok(value) = memory.deref(process) {
            return self.0.update_infallible(value);
        } else {
            return self.0.pair.as_ref().unwrap();
        }
    }

    #[cfg(debugging)]
    fn current(&self) -> T
    where
        T: Clone,
    {
        return self.0.pair.as_ref().unwrap().current.clone();
    }
}

struct Watchers {
    is_loading: Watch<Loading>,
    encounter_counter: Watch<u32>,
    current_level: Watch<Level>,
    story_progression: Watch<Progress>,
    battle_state: Watch<BattleState>,
    cutscene_type: Watch<u32>,
    map_id: Watch<u16>,
    formation_id: Watch<Formation>,
    yu_yevon: Watch<u32>,
    hp_enemy_a: Watch<u32>,
    select_screen: Watch<u32>,
    cursor_position: Watch<u32>,
    input: Watch<Input>,
    #[cfg(any(testing, debugging))]
    loading_slot: Watch<u64>,
    #[cfg(any(testing, debugging))]
    hp_enemy_b: Watch<u32>,
    #[cfg(any(testing, debugging))]
    hp_enemy_c: Watch<u32>,
    #[cfg(any(testing, debugging))]
    hp_enemy_d: Watch<u32>,
}

impl Watchers {
    fn new() -> Watchers {
        return Watchers {
            is_loading: Watch::new(),
            encounter_counter: Watch::new(),
            current_level: Watch::new(),
            story_progression: Watch::new(),
            battle_state: Watch::new(),
            cutscene_type: Watch::new(),
            map_id: Watch::new(),
            formation_id: Watch::new(),
            yu_yevon: Watch::new(),
            hp_enemy_a: Watch::new(),
            select_screen: Watch::new(),
            cursor_position: Watch::new(),
            input: Watch::new(),
            #[cfg(any(testing, debugging))]
            loading_slot: Watch::new(),
            #[cfg(any(testing, debugging))]
            hp_enemy_b: Watch::new(),
            #[cfg(any(testing, debugging))]
            hp_enemy_c: Watch::new(),
            #[cfg(any(testing, debugging))]
            hp_enemy_d: Watch::new(),
        };
    }

    fn loading(&mut self, process: &Process, memory: &Memory) -> &Pair<Loading> {
        return self.is_loading.update(process, &memory.is_loading);
    }

    fn encounter_count(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        return self
            .encounter_counter
            .update(process, &memory.encounter_counter);
    }

    fn level(&mut self, process: &Process, memory: &Memory) -> &Pair<Level> {
        return self.current_level.update(process, &memory.current_level);
    }

    fn story_progression(&mut self, process: &Process, memory: &Memory) -> &Pair<Progress> {
        return self
            .story_progression
            .update(process, &memory.story_progression);
    }

    fn battle_state(&mut self, process: &Process, memory: &Memory) -> &Pair<BattleState> {
        return self.battle_state.update(process, &memory.battle_state);
    }

    fn cutscene_type(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        return self.cutscene_type.update(process, &memory.cutscene_type);
    }

    fn map_id(&mut self, process: &Process, memory: &Memory) -> &Pair<u16> {
        return self.map_id.update(process, &memory.map_id);
    }

    fn formation_id(&mut self, process: &Process, memory: &Memory) -> &Pair<Formation> {
        return self.formation_id.update(process, &memory.formation_id);
    }

    fn yu_yevon(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        return self.yu_yevon.update(process, &memory.yu_yevon);
    }

    fn hp_enemy_a(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        let value = memory.hp_enemy_a.deref(process).map_or(Hp::default(), Hp);
        return self.hp_enemy_a.0.update_infallible(value.0);
    }

    fn select_screen(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        return self.select_screen.update(process, &memory.select_screen);
    }

    fn cursor_position(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        return self
            .cursor_position
            .update(process, &memory.cursor_position);
    }

    fn input(&mut self, process: &Process, memory: &Memory) -> &Pair<Input> {
        return self.input.update(process, &memory.input);
    }

    #[cfg(any(testing, debugging))]
    fn loading_slot(&mut self, process: &Process, memory: &Memory) -> Pair<u32> {
        return self
            .loading_slot
            .update(process, &memory.loading_slot)
            .map(|long| {
                let [idx, off] = checked::cast::<_, [u32; 2]>(long);
                idx.saturating_add(off)
            });
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_b(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        let value = memory.hp_enemy_b.deref(process).map_or(Hp::default(), Hp);
        return self.hp_enemy_b.0.update_infallible(value.0);
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_c(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        let value = memory.hp_enemy_c.deref(process).map_or(Hp::default(), Hp);
        return self.hp_enemy_c.0.update_infallible(value.0);
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_d(&mut self, process: &Process, memory: &Memory) -> &Pair<u32> {
        let value = memory.hp_enemy_d.deref(process).map_or(Hp::default(), Hp);
        return self.hp_enemy_d.0.update_infallible(value.0);
    }

    #[cfg(debugging)]
    fn dump_all_vars(&self) {
        timer::set_variable_int("is_loading", self.is_loading.current().0);
        timer::set_variable_int("current_level", self.current_level.current().0);
        timer::set_variable_int("story_progression", self.story_progression.current().0);
        timer::set_variable_int("battle_state", self.battle_state.current().0);
        timer::set_variable_int("cutscene_type", self.cutscene_type.current());
        timer::set_variable_int("map_id", self.map_id.current());
        timer::set_variable_int("formation_id", self.formation_id.current().0);
        timer::set_variable_int("yu_yevon", self.yu_yevon.current());
        timer::set_variable_int("hp_enemy_a", self.hp_enemy_a.current());
        timer::set_variable_int("hp_enemy_b", self.hp_enemy_b.current());
        timer::set_variable_int("hp_enemy_c", self.hp_enemy_c.current());
        timer::set_variable_int("hp_enemy_d", self.hp_enemy_d.current());
        timer::set_variable_int("select_screen", self.select_screen.current());
        timer::set_variable_int("cursor_position", self.cursor_position.current());
        timer::set_variable_int("input", self.input.current().0);
        timer::set_variable_int("loading_slot", self.loading_slot.current());
    }
}

struct Read<'a> {
    watchers: &'a mut Watchers,
    process: &'a Process,
    memory: &'a Memory,
    is_loading: Option<Pair<Loading>>,
    encounter_counter: Option<Pair<u32>>,
    current_level: Option<Pair<Level>>,
    story_progression: Option<Pair<Progress>>,
    battle_state: Option<Pair<BattleState>>,
    cutscene_type: Option<Pair<u32>>,
    map_id: Option<Pair<u16>>,
    formation_id: Option<Pair<Formation>>,
    yu_yevon: Option<Pair<u32>>,
    hp_enemy_a: Option<Pair<u32>>,
    select_screen: Option<Pair<u32>>,
    cursor_position: Option<Pair<u32>>,
    input: Option<Pair<Input>>,
    #[cfg(any(testing, debugging))]
    loading_slot: Option<Pair<u32>>,
    #[cfg(any(testing, debugging))]
    hp_enemy_b: Option<Pair<u32>>,
    #[cfg(any(testing, debugging))]
    hp_enemy_c: Option<Pair<u32>>,
    #[cfg(any(testing, debugging))]
    hp_enemy_d: Option<Pair<u32>>,
}

impl<'a> Read<'a> {
    fn new(watchers: &'a mut Watchers, process: &'a Process, memory: &'a Memory) -> Self {
        Self {
            watchers,
            process,
            memory,
            is_loading: None,
            encounter_counter: None,
            current_level: None,
            story_progression: None,
            battle_state: None,
            cutscene_type: None,
            map_id: None,
            formation_id: None,
            yu_yevon: None,
            hp_enemy_a: None,
            select_screen: None,
            cursor_position: None,
            input: None,
            #[cfg(any(testing, debugging))]
            loading_slot: None,
            #[cfg(any(testing, debugging))]
            hp_enemy_b: None,
            #[cfg(any(testing, debugging))]
            hp_enemy_c: None,
            #[cfg(any(testing, debugging))]
            hp_enemy_d: None,
        }
    }

    fn loading(&mut self) -> &Pair<Loading> {
        self.is_loading
            .get_or_insert_with(|| *self.watchers.loading(self.process, self.memory))
    }

    fn encounter_count(&mut self) -> &Pair<u32> {
        self.encounter_counter
            .get_or_insert_with(|| *self.watchers.encounter_count(self.process, self.memory))
    }

    fn level(&mut self) -> &Pair<Level> {
        self.current_level
            .get_or_insert_with(|| *self.watchers.level(self.process, self.memory))
    }

    fn story_progression(&mut self) -> &Pair<Progress> {
        self.story_progression
            .get_or_insert_with(|| *self.watchers.story_progression(self.process, self.memory))
    }

    fn battle_state(&mut self) -> &Pair<BattleState> {
        self.battle_state
            .get_or_insert_with(|| *self.watchers.battle_state(self.process, self.memory))
    }

    fn cutscene_type(&mut self) -> &Pair<u32> {
        self.cutscene_type
            .get_or_insert_with(|| *self.watchers.cutscene_type(self.process, self.memory))
    }

    fn hp_enemy_a(&mut self) -> &Pair<u32> {
        self.hp_enemy_a
            .get_or_insert_with(|| *self.watchers.hp_enemy_a(self.process, self.memory))
    }

    fn yu_yevon(&mut self) -> &Pair<u32> {
        self.yu_yevon
            .get_or_insert_with(|| *self.watchers.yu_yevon(self.process, self.memory))
    }

    fn map_id(&mut self) -> &Pair<u16> {
        self.map_id
            .get_or_insert_with(|| *self.watchers.map_id(self.process, self.memory))
    }

    fn formation_id(&mut self) -> &Pair<Formation> {
        self.formation_id
            .get_or_insert_with(|| *self.watchers.formation_id(self.process, self.memory))
    }

    fn select_screen(&mut self) -> &Pair<u32> {
        self.select_screen
            .get_or_insert_with(|| *self.watchers.select_screen(self.process, self.memory))
    }

    fn cursor_position(&mut self) -> &Pair<u32> {
        self.cursor_position
            .get_or_insert_with(|| *self.watchers.cursor_position(self.process, self.memory))
    }

    fn input(&mut self) -> &Pair<Input> {
        self.input
            .get_or_insert_with(|| *self.watchers.input(self.process, self.memory))
    }

    fn is_at(&mut self, story: u32) -> bool {
        return self.story_progression().is(story);
    }

    #[cfg(any(testing, debugging))]
    fn loading_slot(&mut self) -> &Pair<u32> {
        self.loading_slot
            .get_or_insert_with(|| self.watchers.loading_slot(self.process, self.memory))
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_b(&mut self) -> &Pair<u32> {
        self.hp_enemy_b
            .get_or_insert_with(|| *self.watchers.hp_enemy_b(self.process, self.memory))
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_c(&mut self) -> &Pair<u32> {
        self.hp_enemy_c
            .get_or_insert_with(|| *self.watchers.hp_enemy_c(self.process, self.memory))
    }

    #[cfg(any(testing, debugging))]
    fn hp_enemy_d(&mut self) -> &Pair<u32> {
        self.hp_enemy_d
            .get_or_insert_with(|| *self.watchers.hp_enemy_d(self.process, self.memory))
    }
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut enabled = SeenSplits::empty();
        for split in Splits::iter() {
            if self.filter(split) {
                enabled.insert(&split);
            }
        }
        let enabled = enabled.inner();

        let Settings {
            start,
            split,
            reset,
            remove_loads,
            count_encounters,
            _splits_heading1,
            _splits_heading2,
            _splits_heading3,
            #[cfg(testing)]
            _test_heading,
            #[cfg(testing)]
            start_on_load,
            #[cfg(testing)]
            reset_on_load,
            ..
        } = self;

        let mut dbg = f.debug_struct("Settings");
        dbg.field("start", start)
            .field("split", split)
            .field("reset", reset)
            .field("remove_loads", remove_loads)
            .field("count_encounters", count_encounters)
            .field("splits", &DebugAsHex(enabled));

        #[cfg(testing)]
        dbg.field("start_on_load", start_on_load)
            .field("reset_on_load", reset_on_load);

        dbg.finish()
    }
}

impl enum_set::EnumSetMember for Splits {
    fn ordinal(&self) -> Option<u8> {
        Some(u8::from(*self))
    }
}

type SeenSplits = enum_set::EnumSet<Splits>;

trait PairExt<T> {
    fn either(&self, value: T) -> bool
    where
        T: PartialEq;
}

impl<T> PairExt<T> for Pair<T> {
    fn either(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        self.current == value || self.old == value
    }
}

struct DebugAsHex<T>(T);

impl<T: core::fmt::UpperHex> core::fmt::Debug for DebugAsHex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}
