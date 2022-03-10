use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::io::{Stdout, Write};

use crate::game::Input;

const H_CUT_SEC_CHAR: char = '-';
const V_CUT_SEC_CHAR: char = '|';
const CUT_SEC_INTERSECT_CHAR: char = '+';

const X_MAX: u16 = 80;
const Y_MAX: u16 = 40;

const TOP_MENU_PROG_U_INDEX: u16 = 0;
const TOP_MENU_PROG_HEIGHT: u16 = 4;
const TOP_MENU_PROG_D_INDEX: u16 = TOP_MENU_PROG_U_INDEX + TOP_MENU_PROG_HEIGHT + 1;

const LEFT_MENU_HELP_L_INDEX: u16 = 0;
const LEFT_MENU_HELP_WIDTH: u16 = 8;
const LEFT_MENU_HELP_R_INDEX: u16 = LEFT_MENU_HELP_L_INDEX + LEFT_MENU_HELP_WIDTH + 1;

enum CutSecStyle {
    NoIntersect,
    IntersectBothEnds,
    IntersectLSV,
    IntersectMSV,
}

pub fn draw(stdout: &mut Stdout, input: &Input) -> Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // top/bottom edges
    h_cut_section(stdout, 0, X_MAX - 1, 0, CutSecStyle::NoIntersect)?;
    h_cut_section(stdout, 0, X_MAX - 1, Y_MAX - 1, CutSecStyle::NoIntersect)?;

    // left/right edges
    v_cut_section(stdout, 0, Y_MAX - 1, 0, CutSecStyle::IntersectBothEnds)?;
    v_cut_section(
        stdout,
        0,
        Y_MAX - 1,
        X_MAX - 1,
        CutSecStyle::IntersectBothEnds,
    )?;

    // prog menu separator
    h_cut_section(
        stdout,
        0,
        X_MAX - 1,
        TOP_MENU_PROG_D_INDEX,
        CutSecStyle::IntersectBothEnds,
    )?;

    // help menu separator
    v_cut_section(
        stdout,
        TOP_MENU_PROG_D_INDEX,
        Y_MAX - 1,
        LEFT_MENU_HELP_R_INDEX,
        CutSecStyle::IntersectBothEnds,
    )?;

    // help menu contents
    print_controls(
        stdout,
        LEFT_MENU_HELP_L_INDEX + 1,
        TOP_MENU_PROG_D_INDEX + 1,
    )?;

    for i in 0..=9 {
        if (input.num_key_bitwise & (0x01 << i)) > 0 {
            stdout
                .queue(cursor::MoveTo(i + 1, 1))?
                .queue(style::Print(format!("{}", i)))?;
        }
    }

    stdout.flush()?;

    Ok(())
}

fn h_cut_section(
    stdout: &mut Stdout,
    x_first: u16,
    x_last: u16,
    y: u16,
    style: CutSecStyle,
) -> Result<()> {
    // intersect char or not
    match style {
        CutSecStyle::IntersectBothEnds | CutSecStyle::IntersectLSV => {
            stdout
                .queue(cursor::MoveTo(x_first, y))?
                .queue(style::Print(CUT_SEC_INTERSECT_CHAR))?;
        }
        CutSecStyle::IntersectMSV | CutSecStyle::NoIntersect => {
            stdout
                .queue(cursor::MoveTo(x_first, y))?
                .queue(style::Print(H_CUT_SEC_CHAR))?;
        }
    }

    // mid section
    for x in (x_first + 1)..=(x_last - 1) {
        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(H_CUT_SEC_CHAR))?;
    }

    // intersect char or not
    match style {
        CutSecStyle::IntersectBothEnds | CutSecStyle::IntersectMSV => {
            stdout
                .queue(cursor::MoveTo(x_last, y))?
                .queue(style::Print(CUT_SEC_INTERSECT_CHAR))?;
        }
        CutSecStyle::IntersectLSV | CutSecStyle::NoIntersect => {
            stdout
                .queue(cursor::MoveTo(x_last, y))?
                .queue(style::Print(H_CUT_SEC_CHAR))?;
        }
    }

    Ok(())
}

fn v_cut_section(
    stdout: &mut Stdout,
    y_first: u16,
    y_last: u16,
    x: u16,
    style: CutSecStyle,
) -> Result<()> {
    // intersect char or not
    match style {
        CutSecStyle::IntersectBothEnds | CutSecStyle::IntersectLSV => {
            stdout
                .queue(cursor::MoveTo(x, y_first))?
                .queue(style::Print(CUT_SEC_INTERSECT_CHAR))?;
        }
        CutSecStyle::IntersectMSV | CutSecStyle::NoIntersect => {
            stdout
                .queue(cursor::MoveTo(x, y_first))?
                .queue(style::Print(V_CUT_SEC_CHAR))?;
        }
    }

    // mid section
    for y in (y_first + 1)..=(y_last - 1) {
        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(V_CUT_SEC_CHAR))?;
    }

    // intersect char or not
    match style {
        CutSecStyle::IntersectBothEnds | CutSecStyle::IntersectMSV => {
            stdout
                .queue(cursor::MoveTo(x, y_last))?
                .queue(style::Print(CUT_SEC_INTERSECT_CHAR))?;
        }
        CutSecStyle::IntersectLSV | CutSecStyle::NoIntersect => {
            stdout
                .queue(cursor::MoveTo(x, y_last))?
                .queue(style::Print(V_CUT_SEC_CHAR))?;
        }
    }

    Ok(())
}

fn print_controls(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(style::Print("CONTROLS"))?
        .queue(cursor::MoveTo(x, y + 1))?
        .queue(style::Print("test?"))?;

    Ok(())
}
