use super::*;

impl imp::KpWindow {
    pub(super) fn setup_stop_button(&self) {
        self.stop_button
            .connect_clicked(glib::clone!(@weak self as imp => move |_| {
                imp.ready(true);
            }));
    }

    pub(super) fn setup_ui_hiding(&self) {
        self.show_cursor.set(true);

        let device = self
            .obj()
            .display()
            .default_seat()
            .expect("display always has a default seat")
            .pointer()
            .expect("default seat has device");

        self.text_view.connect_typed_text_notify(
            glib::clone!(@weak self as imp, @strong device => move |_| {
                if imp.show_cursor.get() && imp.running.get() {
                    imp.header_bar_running.add_css_class("hide-controls");

                    imp.hide_cursor();
                }
            }),
        );

        let motion_ctrl = gtk::EventControllerMotion::new();
        motion_ctrl.connect_motion(glib::clone!(@weak self as imp, @strong device => move |_,_,_| {
            if !imp.show_cursor.get() && device.timestamp() > imp.cursor_hidden_timestamp.get() {
                imp.header_bar_running.remove_css_class("hide-controls");

                imp.show_cursor();
            }
        }));

        self.obj().add_controller(motion_ctrl);
    }

    pub(super) fn ready(&self, animate: bool) {
        self.running.set(false);
        self.header_bar_running.add_css_class("hide-controls");
        self.text_view.set_running(false);
        self.text_view.set_accepts_input(true);
        self.main_stack.set_visible_child_name("session");
        self.header_stack.set_visible_child_name("ready");
        self.ready_message.set_reveal_child(true);
        self.text_view.reset(animate);
        self.show_cursor();
        self.focus_text_view();

        self.update_original_text();
        self.update_time();
    }

    pub(super) fn start(&self) {
        self.running.set(true);
        self.start_time.set(Some(Instant::now()));
        self.main_stack.set_visible_child_name("session");
        self.header_stack.set_visible_child_name("running");
        self.ready_message.set_reveal_child(false);
        self.hide_cursor();
        self.header_bar_running.add_css_class("hide-controls");

        match self.text_type.get() {
            TextType::Simple | TextType::Advanced => self.start_timer(),
            TextType::Custom => (),
        }
    }

    pub(super) fn finish(&self) {
        self.running.set(false);
        self.text_view.set_running(false);
        self.text_view.set_accepts_input(false);
        self.main_stack.set_visible_child_name("results");
        self.show_cursor();
    }

    pub(super) fn hide_cursor(&self) {
        let device = self
            .obj()
            .display()
            .default_seat()
            .expect("display always has a default seat")
            .pointer()
            .expect("default seat has device");

        self.show_cursor.set(false);
        self.cursor_hidden_timestamp.set(device.timestamp());
        self.obj().set_cursor_from_name(Some("none"));
    }

    pub(super) fn show_cursor(&self) {
        self.show_cursor.set(true);
        self.obj().set_cursor_from_name(Some("default"));
    }

    pub(super) fn focus_text_view(&self) {
        self.obj().set_focus_widget(Some(&self.text_view.get()));
    }
}