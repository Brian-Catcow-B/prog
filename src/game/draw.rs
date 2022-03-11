use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand, Result};
use std::io::{Stdout, Write};
use std::ops::{BitAnd, BitOr};

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

pub trait Bitmask<Arg = Self> {
    fn is_set(self, flag: Arg) -> bool;
    fn set(&mut self, flag: Arg);
}

pub type DrawFlagsMask = u64;
#[repr(u64)]
pub enum DrawFlags {
    All = 0xFF_FF_FF_FF_FF_FF_FF_FF,
    Clear = 0x01,
    ConstPerifs = 0x02,
    ProgContents = 0x04,
}

impl BitAnd<DrawFlags> for DrawFlagsMask {
    type Output = DrawFlagsMask;

    fn bitand(self, rhs: DrawFlags) -> DrawFlagsMask {
        self & rhs as DrawFlagsMask
    }
}

impl BitOr<DrawFlags> for DrawFlagsMask {
    type Output = DrawFlagsMask;

    fn bitor(self, rhs: DrawFlags) -> DrawFlagsMask {
        self | rhs as DrawFlagsMask
    }
}

impl Bitmask<DrawFlags> for DrawFlagsMask {
    fn is_set(self, flag: DrawFlags) -> bool {
        self & flag > 0
    }

    fn set(&mut self, flag: DrawFlags) {
        (*self) = (*self) | flag;
    }
}

enum CutSecStyle {
    NoIntersect,
    IntersectBothEnds,
    IntersectLSV,
    IntersectMSV,
}

pub fn draw(stdout: &mut Stdout, input: &Input, flags: DrawFlagsMask) -> Result<()> {
    if flags == 0 {
        return Ok(());
    }

    if flags.is_set(DrawFlags::Clear) {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    }

    if flags.is_set(DrawFlags::ConstPerifs) {
        print_const_perifs(stdout)?;
    }

    if flags.is_set(DrawFlags::ProgContents) {
        for i in 0..=9 {
            if (input.num_key_bitwise & (0x01 << i)) > 0 {
                stdout
                    .queue(cursor::MoveTo(i + 1, 1))?
                    .queue(style::Print(format!("{}", i)))?;
            }
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

fn print_const_perifs(stdout: &mut Stdout) -> Result<()> {
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
