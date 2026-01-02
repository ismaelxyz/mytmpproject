#[macro_use]
extern crate lazy_static;
extern crate pest;

use iced::widget::{column, container, row, scrollable, text, text_editor};
pub use iced::window::{Id, Position, Settings, close, gain_focus, open};
use iced::{Alignment, Element, Length, Theme};

mod convert_chart;
mod float_pretty_print;
mod parser;
mod test;

struct Formatter;

impl Formatter {
    fn number(value: f64) -> String {
        use float_pretty_print::PrettyPrintFloat;

        if value.is_nan() {
            return "-".to_string();
        }

        if value.fract() == 0.0 {
            return value.to_string().trim().to_string();
        }

        PrettyPrintFloat(value).to_string().trim().to_string()
    }
}

#[derive(Default, Clone)]
struct CalculationResult {
    output: String,
    total: f64,
}

struct CalculatorEngine;

impl CalculatorEngine {
    fn process_input(input: &str) -> CalculationResult {
        let mut output = String::new();
        let mut total = 0.0;

        for line in input.lines() {
            let parsed = parser::parse(line);
            if parsed.is_normal() {
                total += parsed;
            }
            output.push_str(&Formatter::number(parsed));
            output.push('\n');
        }

        CalculationResult { output, total }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    WindowOpened(Id),
    //  Event(window::Id, Event),
    // Tick(Instant),
}

fn app_background(theme: &Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();

    iced::widget::container::Style {
        background: Some(palette.background.weakest.color.into()),
        text_color: Some(palette.background.weakest.text),
        ..iced::widget::container::Style::default()
    }
}

fn panel_style(theme: &Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();

    iced::widget::container::Style {
        background: Some(palette.background.base.color.into()),
        text_color: Some(palette.background.base.text),
        border: iced::Border {
            width: 1.0,
            radius: 10.0.into(),
            color: palette.background.strong.color,
        },
        shadow: iced::Shadow {
            color: iced::Color::BLACK.scale_alpha(0.25),
            offset: iced::Vector::new(0.0, 3.0),
            blur_radius: 12.0,
        },
        ..iced::widget::container::Style::default()
    }
}

fn pill_style(theme: &Theme) -> iced::widget::container::Style {
    let palette = theme.extended_palette();

    iced::widget::container::Style {
        background: Some(palette.primary.weak.color.into()),
        text_color: Some(palette.primary.weak.text),
        border: iced::Border {
            width: 1.0,
            radius: 999.0.into(),
            color: palette.primary.base.color.scale_alpha(0.6),
        },
        ..iced::widget::container::Style::default()
    }
}

struct QubitApp {
    input: text_editor::Content,
    output: String,
    total: f64,
    theme: iced::Theme,
    // config: iced::Settings,
}

impl Default for QubitApp {
    fn default() -> Self {
        Self {
            input: text_editor::Content::new(),
            output: String::new(),
            theme: iced::Theme::Dark,
            total: 0.0,
        }
    }
}

impl QubitApp {
    fn recompute(&mut self) {
        let result = CalculatorEngine::process_input(&self.input.text());
        self.output = result.output;
        self.total = result.total;
    }
}

impl QubitApp {
    fn new() -> (Self, iced::Task<Message>) {
        let mut app = Self::default();
        app.recompute();

        let (_id, task) = open(Settings::default());
        (app, task.map(Message::WindowOpened))
    }

    fn scale_factor(&self, _window_id: Id) -> f32 {
        1.0 // f32::from(self.config.scale_factor)
    }

    fn theme(&self, _window_id: Id) -> Theme {
        self.theme.clone()
    }

    fn title(&self, _window_id: Id) -> String {
        "Qubit".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Edit(action) => {
                self.input.perform(action);
                self.recompute();
            }
            Message::WindowOpened(_id) => {}
        }

        iced::Task::none()
    }

    fn view(&self, _window_id: Id) -> Element<'_, Message> {
        let editor = text_editor(&self.input)
            .on_action(Message::Edit)
            .placeholder("2 + 2 + sin ( 90 )\n12 kg to g")
            .padding(12)
            .size(15)
            .height(Length::Fill);

        let mut results = column![].spacing(4).align_x(Alignment::End);
        for line in self.output.lines() {
            results = results.push(text(line));
        }

        let results = scrollable(results).height(Length::Fill).width(Length::Fill);

        let header = row![
            column![
                text("Qubit").size(28),
                text("Calculator & unit conversions").size(13),
            ]
            .spacing(2)
            .width(Length::Fill),
            container(text(format!("Total: {}", Formatter::number(self.total))))
                .padding([6, 12])
                .style(pill_style),
        ]
        .align_y(Alignment::Center)
        .spacing(12);

        let input_panel = container(column![text("Input").size(12), editor,].spacing(8))
            .padding(14)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .style(panel_style);

        let output_panel = container(
            column![text("Results").size(12), results,]
                .spacing(8)
                .align_x(Alignment::End),
        )
        .padding(14)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(panel_style);

        let content = column![
            header,
            row![input_panel, output_panel]
                .height(Length::Fill)
                .spacing(12),
        ]
        .spacing(14)
        .padding(16)
        .max_width(1100);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .style(app_background)
            .into()
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// fn main() -> iced::Result {
//     QubitApp::run(iced::Settings::default())
// }

// #[cfg(target_arch = "wasm32")]
// fn main() {
//     // For web builds (e.g. via Trunk), iced will hook into the browser event loop.
//     // `run` is available on wasm as well.
//     let _ = QubitApp::run(iced::Settings::default());
// }

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     QubitApp::run(iced::Settings::default()).unwrap();
    // }

    // #[cfg(target_arch = "wasm32")]
    // {
    //     // For web builds (e.g. via Trunk), iced will hook into the browser event loop.
    //     // `run` is available on wasm as well.
    //     let _ = QubitApp::run(iced::Settings::default());
    // }

    iced::daemon(QubitApp::new, QubitApp::update, QubitApp::view)
        .title(QubitApp::title)
        .theme(QubitApp::theme)
        .scale_factor(QubitApp::scale_factor)
        // .subscription(QubitApp::subscription)
        // .settings(settings)
        .run()?;
    // .inspect_err(|err| log::error!("{err}"))?;

    Ok(())
}
