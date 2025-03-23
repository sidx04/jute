use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    // Widgets are constructed and then drawn onto the screen using a `Frame`, which is placed
    // within a specified `Rect`. Now, envision a scenario where we wish to divide our
    // renderable `Rect` area into three distinct areas. For this, we can use the `Layout`
    // functionality in `ratatui`.

    // Take the area `f.area()` (which is a rectangle), and cut it into three vertical pieces
    // (making horizontal cuts).
    // The first section will be 3 lines tall
    // The second section should never be smaller than one line tall, but can expand if needed.
    // The final section should also be 3 lines tall

    // ```
    // +-------------------------------------------------+
    // | Top segment always remains 3 lines              | Constraint :: Length=3
    // +-------------------------------------------------+
    // |                                                 |
    // | Middle segment maintains a minimum height of 1  | Constraint :: Length>=1
    // | line, but can expand if additional space is     |
    // | present.                                        |
    // |                                                 |
    // +-------------------------------------------------+
    // | Bottom segment is consistently 3 lines          | Constraint :: Length=3
    // +-------------------------------------------------+
    // ```
    //
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // To create our title, we are going to use a `Paragraph` widget
    // (which is used to display only text), and we are going to tell that
    // `Paragraph` we want a border all around it by giving it a `Block`
    // with borders enabled
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "JUTE: Create JSON keypairs in your terminal!",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    frame.render_widget(title, chunks[0]);

    // `List` is what it sounds like - it creates a new line of text for each
    // `ListItem`, and it supports passing in a state so you can implement
    // selecting items on the list with little extra work.
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items).style(Style::default().bg(Color::Rgb(15, 15, 15)));

    frame.render_widget(list, chunks[1]);

    // Bottom Navbar
    // Two bars, and another layout.
    // These two bars will contain information on
    //    1) the current screen (Main, Editing, and Exiting)
    //    2) what keybinds are available.

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    //  we are also going to make a hint in the navigation bar with available keys
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel / (Tab) to switch boxes / (ENTER) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    //
    // +-------------------------------------------------+
    // | Length = 50%         |      Length = 50%        | Constraint :: Length=3|
    // +-------------------------------------------------+
    //

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // `Block` that will contain the popup for editing screen.
    // We will give this Block a title to display as well to
    // explain to the user what it is.
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        // create split the `Rect` given to us by `centered_rect`, and create a layout from it.
        // Note the use of `margin(1)`, which gives a 1 space margin around any layout block,
        // meaning our new blocks and widgets don’t overwrite anything
        // from the first popup block.
        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // what to display
        let mut key_block = Block::default()
            .title("Key")
            .borders(Borders::ALL)
            .italic()
            .bold();
        let mut value_block = Block::default()
            .title("Value")
            .borders(Borders::ALL)
            .italic()
            .bold();

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);

        // Note that we are declaring the blocks as variables, and then adding
        // extra styling to the block the user is currently editing.
        // Then we create the `Paragraph` widgets, and assign the blocks
        // with those variables. Also note how we used the `popup_chunks` layout
        // instead of the `popup_block` layout to render these widgets into.
    }

    // In this screen, we are asking the user if they want to output
    // the key-value pairs they have entered in the stdout pipe,
    // or close without outputting anything.
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn

        let popup_block = Block::bordered()
            .title("Exit")
            .title_style(Style::default().bold())
            .title_alignment(Alignment::Center)
            .border_set(symbols::border::ROUNDED)
            .style(Style::new().fg(Color::LightCyan).bg(Color::Rgb(123, 3, 35)))
            .padding(Padding::uniform(2));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::White),
        );

        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain
/// percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
