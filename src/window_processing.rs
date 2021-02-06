use std::error::Error;
use x11rb::xcb_ffi::XCBConnection;
use x11rb::generated::xproto::{ConnectionExt, Atom, GetPropertyReply, ATOM, WINDOW};
use x11rb::wrapper::LazyAtom;
use x11rb::connection::Connection;
use x11rb::x11_utils::TryParse;

use crate::stream_info;

//Taken from 'https://www.reddit.com/r/rust/comments/f7yrle/get_information_about_current_window_xorg/fijrbyr'
pub fn linux_query_window(stream_info: &mut stream_info::_StreamInfo) -> Result<(), Box<dyn Error>> {
    // Set up our state
    let (conn, screen) = XCBConnection::connect(None)?;
    let root = conn.setup().roots[screen].root;
    let mut net_active_window = LazyAtom::new(&conn, false, b"_NET_ACTIVE_WINDOW");
    let mut net_wm_name = LazyAtom::new(&conn, false, b"_NET_WM_NAME");
    let mut utf8_string = LazyAtom::new(&conn, false, b"UTF8_STRING");

    let focus = find_active_window(&conn, root, net_active_window.atom()?)?;

    // Collect the replies to the atoms
    let (net_wm_name, utf8_string) = (net_wm_name.atom()?, utf8_string.atom()?);
    let (wm_class, string) = (Atom::WM_CLASS.into(), Atom::STRING.into());

    // Get the property from the window that we need
    let name = conn.get_property(false, focus, net_wm_name, utf8_string, 0, u32::max_value())?;
    let class = conn.get_property(false, focus, wm_class, string, 0, u32::max_value())?;
    let (name, class) = (name.reply()?, class.reply()?);

    // Print out the result
    let (instance, class) = parse_wm_class(&class);
    
    if class == "GStreamer"{
        stream_info.current_window = true
    } else {
        stream_info.current_window = false
    }
    Ok(())
}

fn find_active_window(conn: &impl Connection, root: WINDOW, net_active_window: ATOM) -> Result<WINDOW, Box<dyn Error>> {
    let window = Atom::WINDOW.into();
    let active_window = conn.get_property(false, root, net_active_window, window, 0, 1)?.reply()?;
    if active_window.format == 32 && active_window.length == 1 {
        // Things will be so much easier with the next release:
        // This does active_window.value32().next().unwrap()
        Ok(u32::try_parse(&active_window.value)?.0)
    } else {
        // Query the input focus
        Ok(conn.get_input_focus()?.reply()?.focus)
    }
}

fn parse_string_property(property: &GetPropertyReply) -> &str {
    std::str::from_utf8(&property.value).unwrap_or("Invalid utf8")
}

fn parse_wm_class(property: &GetPropertyReply) -> (&str, &str) {
    if property.format != 8 {
        return ("Malformed property: wrong format", "Malformed property: wrong format");
    }
    let value = &property.value;
    // The property should contain two null-terminated strings. Find them.
    if let Some(middle) = value.iter().position(|&b| b == 0) {
        let (instance, class) = value.split_at(middle);
        // Skip the null byte at the beginning
        let mut class = &class[1..];
        // Remove the last null byte from the class, if it is there.
        if class.last() == Some(&0) {
            class = &class[..class.len() - 1];
        }
        let instance = std::str::from_utf8(instance);
        let class = std::str::from_utf8(class);
        (instance.unwrap_or("Invalid utf8"), class.unwrap_or("Invalid utf8"))
    } else {
        ("Missing null byte", "Missing null byte")
    }
}
