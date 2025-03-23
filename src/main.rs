mod app;
mod ui;

use crate::ui::ui;
use app::{App, CurrentScreen, CurrentlyEditing};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::Terminal;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::prelude::{Backend, CrosstermBackend};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    // You might notice that we are using stderr for our output.
    // This is because we want to allow the user to pipe their
    // completed json to other programs like ratatui-tutorial > output.json.
    // To do this, we are using the fact that stderr is
    // piped differently than stdout. We render output to stderr,
    // and print our completed json to stdout.
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::init();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // When an application exits without running this closing boilerplate,
    // the terminal will act very strange, and the user will usually have to
    // end the terminal session and start a new one.
    // Thus it is important that we handle our error in such a way that
    // we can call this last piece.
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        // `terminal` is the `Terminal<Backend>` that we take as an argument,
        // `draw` is the `ratatui` command to draw a `Frame` to the `terminal`.
        // `|f| ui(f, &app)` tells `draw` that we want to take `f: <Frame>`
        // and pass it to our function `ui`, and `ui` will draw to that Frame.
        // we also pass an immutable borrow of our application state to the `ui` function.
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not `KeyEventKind::Press`
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    // In this case, `KeyCode::Char('e')` changes the current screen to
                    // `CurrentScreen::Editing` and sets the `CurrentlyEditing` to a Some and
                    // notes that the user should be editing the `Key` value field, as opposed
                    // to the `Value` field.
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    // We would like the Enter key to serve two purposes.
                    // When the user is editing the `Key`, we want the enter key to switch the
                    // focus to editing the `Value`. However, if the Value is what is being
                    // currently edited, Enter will save the key-value pair, and return to the
                    // Main screen.
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    // When Backspace is pressed, we need to first determine if the user is
                    // editing a `Key` or a `Value`, then `pop()` the endings of those strings
                    // accordingly.
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    // When Tab is pressed, we want the currently editing selection to switch.
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    // if the user types a valid character, we want to capture that, and add it
                    // to the string that is the final key or value.
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
