use gpui::{App, AppContext, Context, Entity, Global, SharedString, Subscription};
use gpui_component::tab::Tab;
use smallvec::{smallvec, SmallVec};

pub fn init(cx: &mut App) {
    let state = cx.new(AppRoute::new);

    state.update(cx, |this, cx| {
        this.subscriptions.push(cx.observe(&state, |_this, _state, _cx| {
            // route change side effect
        }));
    });

    AppRoute::set_global(state, cx);
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SettingsSubRoute {
    General,
    Advanced,
    About,
}

impl From<SettingsSubRoute> for Tab {
    fn from(route: SettingsSubRoute) -> Self {
        Tab::new().label(match route {
            SettingsSubRoute::General => SharedString::new_static("基础设置"),
            SettingsSubRoute::Advanced => SharedString::new_static("高级设置"),
            SettingsSubRoute::About => SharedString::new_static("关于"),
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ToolsSubRoute {
    Copilot,
    Recruit,
    Gacha,
}

impl From<ToolsSubRoute> for Tab {
    fn from(route: ToolsSubRoute) -> Self {
        Tab::new().label(match route {
            ToolsSubRoute::Copilot => SharedString::new_static("自动战斗"),
            ToolsSubRoute::Recruit => SharedString::new_static("公招识别"),
            ToolsSubRoute::Gacha => SharedString::new_static("牛牛抽卡"),
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Route {
    Home,
    Tools(ToolsSubRoute),
    Tasks,
    Dashboard,
    Settings(SettingsSubRoute),
}

impl Route {
    pub fn id(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("home"),
            Route::Tools(sub_route) => match sub_route {
                ToolsSubRoute::Copilot => SharedString::new_static("tools-copliot"),
                ToolsSubRoute::Recruit => SharedString::new_static("tools-recruit"),
                ToolsSubRoute::Gacha => SharedString::new_static("tools-gacha"),
            },
            Route::Tasks => SharedString::new_static("tasks"),
            Route::Dashboard => SharedString::new_static("dashboard"),
            Route::Settings(sub_route) => match sub_route {
                SettingsSubRoute::General => SharedString::new_static("settings-general"),
                SettingsSubRoute::Advanced => SharedString::new_static("settings-advanced"),
                SettingsSubRoute::About => SharedString::new_static("settings-about"),
            },
        }
    }

    pub fn label(&self) -> SharedString {
        match self {
            Route::Home => SharedString::new_static("主页"),
            Route::Tools(_) => SharedString::new_static("工具"),
            Route::Tasks => SharedString::new_static("任务列表"),
            Route::Dashboard => SharedString::new_static("仪表盘"),
            Route::Settings(_) => SharedString::new_static("设置"),
        }
    }
}

impl From<Route> for Tab {
    fn from(route: Route) -> Self {
        Tab::new().label(route.label())
    }
}

struct GlobalAppRoute(Entity<AppRoute>);

impl Global for GlobalAppRoute {}

pub struct AppRoute {
    pub route: Route,
    #[allow(dead_code)]
    subscriptions: SmallVec<[Subscription; 1]>,
}

impl AppRoute {
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalAppRoute>().0.clone()
    }

    pub fn get_global(cx: &App) -> &Self {
        cx.global::<GlobalAppRoute>().0.read(cx)
    }

    pub fn global_mut(cx: &mut App) -> Entity<Self> {
        cx.global_mut::<GlobalAppRoute>().0.clone()
    }

    pub(crate) fn set_global(state: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalAppRoute(state));
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = smallvec![];

        subscriptions.push(cx.observe_new::<Self>(|_this, _window, _cx| {
            // initial side effect
        }));

        Self {
            route: Route::Home,
            subscriptions,
        }
    }
}
