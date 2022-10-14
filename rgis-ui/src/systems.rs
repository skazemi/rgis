use bevy::prelude::*;
use bevy_egui::egui;

fn render_bottom_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mouse_pos: Res<rgis_mouse::MousePos>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mut open_change_crs_window_event_writer: bevy::ecs::event::EventWriter<
        rgis_events::OpenChangeCrsWindow,
    >,
    mut bottom_panel_height: ResMut<crate::BottomPanelHeight>,
) {
    crate::bottom_panel::BottomPanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        mouse_pos: &mouse_pos,
        rgis_settings: &rgis_settings,
        open_change_crs_window_event_writer: &mut open_change_crs_window_event_writer,
        bottom_panel_height: &mut bottom_panel_height,
    }
    .render();
}

fn render_side_panel(
    mut manage_layer_window_state: ResMut<crate::ManageLayerWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut events: crate::side_panel::Events,
    mut side_panel_width: ResMut<crate::SidePanelWidth>,
) {
    crate::side_panel::SidePanel {
        egui_ctx: bevy_egui_ctx.ctx_mut(),
        manage_layer_window_state: &mut manage_layer_window_state,
        layers: &layers,
        events: &mut events,
        side_panel_width: &mut side_panel_width,
    }
    .render();
}

fn handle_open_file_task(
    mut finished_tasks: bevy_jobs::FinishedJobs,
    mut selected_file: ResMut<crate::add_layer_window::SelectedFile>,
) {
    while let Some(outcome) = finished_tasks.take_next::<crate::add_layer_window::OpenFileTask>() {
        if let Some(outcome) = outcome {
            selected_file.0 = Some(outcome);
        }
    }
}

fn render_manage_layer_window(
    mut state: ResMut<crate::ManageLayerWindowState>, // TODO: change this to Local?
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    layers: Res<rgis_layers::Layers>,
    mut color_events: ResMut<bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>>,
) {
    crate::manage_layer_window::ManageLayerWindow {
        state: &mut state,
        layers: &layers,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        color_events: &mut color_events,
    }
    .render();
}

struct IsVisible(bool);

impl Default for IsVisible {
    fn default() -> Self {
        IsVisible(true)
    }
}

fn render_add_layer_window(
    mut is_visible: Local<IsVisible>,
    mut selected_file: ResMut<crate::add_layer_window::SelectedFile>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut task_spawner: bevy_jobs::JobSpawner,
    mut state: Local<crate::add_layer_window::State>,
    mut events: crate::add_layer_window::Events,
) {
    if !events.show_add_layer_window_event_reader.is_empty() {
        is_visible.0 = true;
    }

    if !events.hide_add_layer_window_events.is_empty() {
        state.reset();
        is_visible.0 = false;
    }

    crate::add_layer_window::AddLayerWindow {
        state: &mut state,
        selected_file: &mut selected_file,
        is_visible: &mut is_visible.0,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        task_spawner: &mut task_spawner,
        events: &mut events,
    }
    .render();
}

fn render_change_crs_window(
    mut is_visible: Local<bool>,
    mut open_change_crs_window_event_reader: bevy::ecs::event::EventReader<
        rgis_events::OpenChangeCrsWindow,
    >,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut text_field_value: Local<String>,
    mut change_crs_event_writer: bevy::ecs::event::EventWriter<rgis_events::ChangeCrsEvent>,
) {
    if open_change_crs_window_event_reader.iter().next().is_some() {
        *is_visible = true;
    }

    crate::change_crs_window::ChangeCrsWindow {
        is_visible: &mut is_visible,
        bevy_egui_ctx: &mut bevy_egui_ctx,
        text_field_value: &mut text_field_value,
        change_crs_event_writer: &mut change_crs_event_writer,
        rgis_settings: &rgis_settings,
    }
    .render();
}

fn render_feature_properties_window(
    mut state: Local<crate::FeaturePropertiesWindowState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut render_message_events: ResMut<
        bevy::ecs::event::Events<rgis_events::RenderFeaturePropertiesEvent>,
    >,
) {
    if let Some(event) = render_message_events.drain().last() {
        state.is_visible = true;
        state.properties = Some(event.0);
    }
    crate::feature_properties_window::FeaturePropertiesWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();
}

fn render_message_window(
    mut state: Local<crate::MessageWindowState>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut render_message_events: ResMut<bevy::ecs::event::Events<rgis_events::RenderMessageEvent>>,
) {
    if let Some(event) = render_message_events.drain().last() {
        state.message = Some(event.0);
        state.is_visible = true;
    }
    crate::message_window::MessageWindow {
        state: &mut state,
        bevy_egui_ctx: &mut bevy_egui_ctx,
    }
    .render();
}

fn render_in_progress(
    query: Query<&bevy_jobs::InProgressJob>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    let mut task_name_iter = query.iter().map(|task| &task.name).peekable();

    if task_name_iter.peek().is_none() {
        return;
    }

    egui::Window::new("Running tasks")
        .open(&mut true)
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5., -5.])
        .show(bevy_egui_ctx.ctx_mut(), |ui| {
            for task_name in task_name_iter {
                ui.horizontal(|ui| {
                    ui.add(egui::Spinner::new());
                    ui.label(format!("Running '{}'", task_name));
                });
            }
        });
}

fn render_top_panel(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut app_exit_events: ResMut<bevy::ecs::event::Events<bevy::app::AppExit>>,
    mut windows: ResMut<Windows>,
    mut app_settings: ResMut<rgis_settings::RgisSettings>,
    mut top_panel_height: ResMut<crate::TopPanelHeight>,
) {
    crate::top_panel::TopPanel {
        bevy_egui_ctx: &mut bevy_egui_ctx,
        app_exit_events: &mut app_exit_events,
        windows: &mut windows,
        app_settings: &mut app_settings,
        top_panel_height: &mut top_panel_height,
    }
    .render();
}

fn set_egui_theme(
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    mut clear_color: ResMut<ClearColor>,
) {
    let egui_visuals = match dark_light::detect() {
        dark_light::Mode::Dark => egui::Visuals::dark(),
        dark_light::Mode::Light => egui::Visuals::light(),
    };
    // Set the background color of the map
    clear_color.0 = egui_color_to_bevy_color(egui_visuals.extreme_bg_color);
    // Set the egui theme
    bevy_egui_ctx.ctx_mut().set_visuals(egui_visuals);
}

fn egui_color_to_bevy_color(egui_color: bevy_egui::egui::Color32) -> bevy::render::color::Color {
    bevy::render::color::Color::rgb_u8(egui_color.r(), egui_color.g(), egui_color.b())
}

pub fn startup_system_set() -> SystemSet {
    SystemSet::new().with_system(set_egui_theme)
}

pub fn system_sets() -> [SystemSet; 2] {
    [
        SystemSet::new()
            .label("top_bottom_panels")
            .after(render_message_window)
            .with_system(render_top_panel)
            .with_system(render_bottom_panel),
        SystemSet::new()
            .with_system(handle_open_file_task)
            .with_system(render_message_window)
            .with_system(render_side_panel.after("top_bottom_panels"))
            .with_system(render_manage_layer_window.after(render_side_panel))
            .with_system(render_add_layer_window.after(render_manage_layer_window))
            .with_system(render_change_crs_window.after(render_add_layer_window))
            .with_system(render_feature_properties_window.after(render_add_layer_window))
            .with_system(render_in_progress.after("top_bottom_panels")),
    ]
}
