use gtk4::prelude::*;
use gtk4::{self as gtk};
use libadwaita as adw;
use adw::prelude::*;

fn main() {
    let app = adw::Application::builder()
        .application_id("com.omma.IncubatorApp")
        .build();

    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });
    app.run();
}

fn load_css() {
    let provider = gtk::CssProvider::new();

    // Load CSS directly as string for reliability
    let css = r#"
        /* Omma GTK App Styling - Dark Mode */

        @define-color omma_green #10b981;
        @define-color omma_green_dark #059669;
        @define-color omma_emerald #10b981;
        @define-color omma_purple #a855f7;

        window {
            background: linear-gradient(135deg, #111827 0%, #1f2937 50%, #111827 100%);
            color: #e5e7eb;
        }

        .toolbar {
            background-color: rgba(17, 24, 39, 0.95);
            border-bottom: 1px solid rgba(55, 65, 81, 0.5);
        }

        .card {
            background-color: rgba(31, 41, 55, 0.5);
            border: 1px solid rgba(55, 65, 81, 0.5);
            border-radius: 16px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
            margin: 8px 0;
            color: #e5e7eb;
        }

        label {
            color: #e5e7eb;
        }

        entry {
            background-color: rgba(31, 41, 55, 0.8);
            color: #e5e7eb;
            border: 1px solid rgba(55, 65, 81, 0.5);
        }

        button {
            color: #e5e7eb;
        }

        .omma-progress trough {
            min-height: 1px;
            background-color: rgba(55, 65, 81, 0.5);
            border-radius: 0;
        }

        .omma-progress progress {
            background-image: linear-gradient(to right, @omma_green, @omma_emerald);
            border-radius: 0;
            min-height: 1px;
        }

        .badge {
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 600;
            border: 1px solid;
        }

        .badge.purple {
            background-color: rgba(168, 85, 247, 0.2);
            color: #c084fc;
            border-color: rgba(168, 85, 247, 0.2);
        }

        .badge.dim {
            background-color: rgba(107, 114, 128, 0.2);
            color: #9ca3af;
            border-color: rgba(107, 114, 128, 0.2);
        }

        .suggested-action {
            background-image: linear-gradient(to bottom, @omma_green, @omma_green_dark);
            color: white;
            border: none;
            border-radius: 12px;
            padding: 8px 16px;
            font-weight: 600;
        }

        .suggested-action:hover {
            background-image: linear-gradient(to bottom, @omma_green_dark, @omma_green_dark);
        }

        .pill {
            background-color: rgba(249, 250, 251, 0.9);
            border: 1px solid rgba(209, 213, 219, 0.5);
            border-radius: 20px;
            padding: 6px 12px;
            font-size: 12px;
        }

        .dim-label {
            color: #6b7280;
            font-size: 12px;
        }

        .caption {
            font-size: 10px;
        }

        .title-3 {
            font-size: 16px;
            font-weight: 600;
        }
    "#;

    provider.load_from_string(css);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    // Create main window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Omma")
        .default_width(1200)
        .default_height(900)
        .build();

    // Note: macOS dock icons with GTK apps require .app bundle packaging
    // For now, the dock will show the default GTK icon
    // This would work properly on Linux/Pi where GTK is native

    // Quit app when window is closed
    let app_clone = app.clone();
    window.connect_close_request(move |_| {
        app_clone.quit();
        gtk::glib::Propagation::Stop
    });

    // Main vertical container
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Add header bar with window controls (red/yellow/green buttons)
    let header_bar = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(Some("Omma")))
        .build();
    main_box.append(&header_bar);

    // Top navigation bar with logo and breadcrumbs
    let top_nav = create_top_navigation();
    main_box.append(&top_nav);

    // Date picker and search section
    let date_search = create_date_search_section();
    main_box.append(&date_search);

    // Scrollable content area
    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .build();

    let content = create_content_area();
    scrolled.set_child(Some(&content));
    main_box.append(&scrolled);

    window.set_content(Some(&main_box));
    window.present();
}

fn create_top_navigation() -> gtk::Box {
    let nav_bar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(20)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(20)
        .margin_end(20)
        .css_classes(vec!["toolbar"])
        .valign(gtk::Align::Center)
        .build();

    // Omma logo - load and scale to exact size
    let logo_path = "/Users/dongyiuwu/SoftwareDevelopment/omma-linux/logo_full.png";
    let logo = if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(
        logo_path,
        140,  // width
        36,   // height - reduced for more compact nav
        true  // preserve aspect ratio
    ) {
        // Use Texture instead of deprecated from_pixbuf
        let texture = gtk::gdk::Texture::for_pixbuf(&pixbuf);
        let picture = gtk::Picture::for_paintable(&texture);
        picture.set_valign(gtk::Align::Center);
        picture
    } else {
        // Fallback if image fails to load
        gtk::Picture::new()
    };

    nav_bar.append(&logo);

    // Breadcrumbs
    let breadcrumbs = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .valign(gtk::Align::Center)
        .build();

    let crumb1 = gtk::Button::builder()
        .label("Summerberry ▾")
        .valign(gtk::Align::Center)
        .build();
    breadcrumbs.append(&crumb1);

    let sep1 = gtk::Label::builder()
        .label("/")
        .valign(gtk::Align::Center)
        .build();
    breadcrumbs.append(&sep1);

    let crumb2 = gtk::Button::builder()
        .label("Groves Farm ▾")
        .valign(gtk::Align::Center)
        .build();
    breadcrumbs.append(&crumb2);

    let sep2 = gtk::Label::builder()
        .label("/")
        .valign(gtk::Align::Center)
        .build();
    breadcrumbs.append(&sep2);

    let crumb3 = gtk::Button::builder()
        .label("Summerberry ▾")
        .valign(gtk::Align::Center)
        .build();
    breadcrumbs.append(&crumb3);

    nav_bar.append(&breadcrumbs);

    // Spacer
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    nav_bar.append(&spacer);

    // Right side buttons
    let harvest_btn = gtk::Button::builder()
        .label("Harvest")
        .css_classes(vec!["suggested-action"])
        .valign(gtk::Align::Center)
        .build();
    nav_bar.append(&harvest_btn);

    let list_btn = gtk::Button::builder()
        .icon_name("view-list-symbolic")
        .valign(gtk::Align::Center)
        .build();
    nav_bar.append(&list_btn);

    let grid_btn = gtk::Button::builder()
        .icon_name("view-grid-symbolic")
        .valign(gtk::Align::Center)
        .build();
    nav_bar.append(&grid_btn);

    nav_bar
}

fn create_date_search_section() -> gtk::Box {
    let section = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(16)
        .margin_top(8)
        .margin_bottom(12)
        .margin_start(20)
        .margin_end(20)
        .build();

    // Date navigation
    let date_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    let prev_btn = gtk::Button::builder()
        .icon_name("go-previous-symbolic")
        .build();
    date_box.append(&prev_btn);

    let date_btn = gtk::Button::builder()
        .label("Oct 16, 2025")
        .build();
    date_box.append(&date_btn);

    let next_btn = gtk::Button::builder()
        .icon_name("go-next-symbolic")
        .build();
    date_box.append(&next_btn);

    section.append(&date_box);

    // Search bar
    let search_entry = gtk::SearchEntry::builder()
        .placeholder_text("Search incubators...")
        .hexpand(true)
        .build();
    section.append(&search_entry);

    // Today button
    let today_btn = gtk::Button::builder()
        .label("Today")
        .css_classes(vec!["suggested-action"])
        .build();
    section.append(&today_btn);

    section
}

fn create_content_area() -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .margin_top(8)
        .margin_bottom(16)
        .margin_start(20)
        .margin_end(20)
        .build();

    // Create 10 incubator cards
    let incubators = vec![
        ("Incubator Z", "Orius", "Empty", "1/2", vec![("Harvest Adults", true, true), ("Feed Insects", false, true)], "Population: 0", "ID: b0b18929..."),
        ("Incubator AJ", "Orius", "Empty", "1/2", vec![("Harvest Adults", true, true), ("Feed Insects", false, true)], "Population: 0", "ID: d9f39c5c..."),
        ("Incubator B", "Macrolophus", "Full", "2/2", vec![("Harvest Adults", false, false), ("Feed Insects", true, false), ("Clean Tank", false, false)], "Population: 150", "ID: a3b42d1e..."),
        ("Incubator C", "Orius", "Half", "1/2", vec![("Harvest Adults", true, false), ("Check Temperature", false, false)], "Population: 75", "ID: c8e91f2a..."),
        ("Incubator D", "Nesidiocoris", "Empty", "0/3", vec![("Harvest Adults", false, true), ("Feed Insects", false, false), ("Water Change", false, false)], "Population: 0", "ID: f1d84c7b..."),
        ("Incubator E", "Orius", "Full", "3/3", vec![("Harvest Adults", true, false), ("Feed Insects", true, false), ("Clean Tank", true, false)], "Population: 200", "ID: b7a39d8e..."),
        ("Incubator F", "Macrolophus", "Half", "2/3", vec![("Harvest Adults", true, false), ("Feed Insects", true, false), ("Monitor Growth", false, false)], "Population: 120", "ID: d4c28f1a..."),
        ("Incubator G", "Orius", "Empty", "0/2", vec![("Harvest Adults", false, true), ("Feed Insects", false, true)], "Population: 0", "ID: e9b17a3c..."),
        ("Incubator H", "Nesidiocoris", "Full", "2/2", vec![("Harvest Adults", true, false), ("Feed Insects", true, false)], "Population: 180", "ID: a2f64d8b..."),
        ("Incubator I", "Orius", "Half", "1/2", vec![("Harvest Adults", false, false), ("Feed Insects", true, false)], "Population: 90", "ID: c5e92b7d..."),
    ];

    for (name, species, status, progress, tasks, population, id) in incubators {
        let card = create_incubator_card(name, species, status, progress, tasks, population, id);
        content.append(&card);
    }

    content
}

fn create_incubator_card(
    name: &str,
    species: &str,
    status: &str,
    progress: &str,
    tasks: Vec<(&str, bool, bool)>,
    population: &str,
    id: &str,
) -> gtk::Frame {
    let frame = gtk::Frame::builder()
        .css_classes(vec!["card"])
        .build();

    let card_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .build();

    // Green progress bar at top
    let progress_bar = gtk::ProgressBar::builder()
        .fraction(0.5)
        .css_classes(vec!["omma-progress"])
        .build();
    card_box.append(&progress_bar);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    // Header row
    let header_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    let title_label = gtk::Label::builder()
        .label(name)
        .halign(gtk::Align::Start)
        .css_classes(vec!["title-3"])
        .build();
    header_row.append(&title_label);

    // Spacer
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Status badges
    let badges_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    let species_badge = gtk::Label::builder()
        .label(species)
        .css_classes(vec!["badge", "purple"])
        .build();
    badges_box.append(&species_badge);

    let status_badge = gtk::Label::builder()
        .label(status)
        .css_classes(vec!["badge", "dim"])
        .build();
    badges_box.append(&status_badge);

    let progress_badge = gtk::Label::builder()
        .label(progress)
        .css_classes(vec!["badge"])
        .build();
    badges_box.append(&progress_badge);

    header_row.append(&badges_box);
    content_box.append(&header_row);

    // Tasks list
    let tasks_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .build();

    for (task_label, checked, locked) in tasks {
        let task_row = create_task_row(task_label, checked, locked);
        tasks_box.append(&task_row);
    }

    content_box.append(&tasks_box);

    // Footer row
    let footer_row = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let population_label = gtk::Label::builder()
        .label(population)
        .halign(gtk::Align::Start)
        .css_classes(vec!["dim-label"])
        .build();
    footer_row.append(&population_label);

    let footer_spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    footer_spacer.set_hexpand(true);
    footer_row.append(&footer_spacer);

    let id_label = gtk::Label::builder()
        .label(id)
        .halign(gtk::Align::End)
        .css_classes(vec!["dim-label", "caption"])
        .build();
    footer_row.append(&id_label);

    content_box.append(&footer_row);
    card_box.append(&content_box);
    frame.set_child(Some(&card_box));
    frame
}

fn create_task_row(label: &str, initial_checked: bool, cancelled: bool) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);

    let checkbox = gtk::CheckButton::builder()
        .active(initial_checked)
        .build();
    row.append(&checkbox);

    let task_label = gtk::Label::builder()
        .label(label)
        .halign(gtk::Align::Start)
        .build();

    if cancelled {
        task_label.add_css_class("dim-label");
    }

    task_label.set_hexpand(true);
    row.append(&task_label);

    if cancelled {
        let cancelled_badge = gtk::Label::builder()
            .label("CANCELLED")
            .css_classes(vec!["caption", "dim-label"])
            .build();
        row.append(&cancelled_badge);
    }

    let more_btn = gtk::Button::builder()
        .icon_name("view-more-symbolic")
        .build();
    row.append(&more_btn);

    row
}
