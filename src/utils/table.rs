use tabled::{
    settings::{
        object::{ Columns, Rows },
        style::BorderColor,
        themes::Colorization,
        Color,
        Style,
        Width,
    },
    Table,
};
use terminal_size::{ terminal_size, Width as TermWidth };

pub fn render_table<T: tabled::Tabled>(data: Vec<T>) -> String {
    let term_width = terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(80);

    /* Define max wrap width (leave some padding) */
    let wrap_width = term_width.saturating_sub(10);

    let mut table = Table::new(data);

    table.with(Style::rounded());
    table.with(Width::truncate(wrap_width).suffix("..."));

    /* Colors the first row and the first column */
    table.with(Colorization::exact([Color::FG_CYAN], Rows::first()));
    table.with(Colorization::exact([Color::FG_CYAN], Columns::first()));
    table.modify(Rows::single(0), BorderColor::filled(Color::FG_CYAN));
    table.modify(Columns::single(0), BorderColor::filled(Color::FG_CYAN));

    return table.to_string();
}
