use winit::window::CursorIcon;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitUserEvent {
    /// The virtual machine sends
    WakeUp,
}

impl Default for WinitUserEvent {
    fn default() -> Self {
        Self::WakeUp
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitCursorIcon {
    /// The platform-dependent default cursor.
    Default,
    /// A simple crosshair.
    Crosshair,
    /// A hand (often used to indicate links in web browsers).
    Hand,
    /// Self explanatory.
    Arrow,
    /// Indicates something is to be moved.
    Move,
    /// Indicates text that may be selected or edited.
    Text,
    /// Program busy indicator.
    Wait,
    /// Help indicator (often rendered as a "?")
    Help,
    /// Progress indicator. Shows that processing is being done. But in contrast
    /// with "Wait" the user may still interact with the program. Often rendered
    /// as a spinning beach ball, or an arrow with a watch or hourglass.
    Progress,

    /// Cursor showing that something cannot be done.
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,

    /// Indicate that some edge is to be moved. For example, the 'SeResize' cursor
    /// is used when the movement starts from the south-east corner of the box.
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl From<WinitCursorIcon> for CursorIcon {
    fn from(cursor_icon: WinitCursorIcon) -> CursorIcon {
        match cursor_icon {
            WinitCursorIcon::Default => CursorIcon::Default,
            WinitCursorIcon::Crosshair => CursorIcon::Crosshair,
            WinitCursorIcon::Hand => CursorIcon::Hand,
            WinitCursorIcon::Arrow => CursorIcon::Arrow,
            WinitCursorIcon::Move => CursorIcon::Move,
            WinitCursorIcon::Text => CursorIcon::Text,
            WinitCursorIcon::Wait => CursorIcon::Wait,
            WinitCursorIcon::Help => CursorIcon::Help,
            WinitCursorIcon::Progress => CursorIcon::Progress,
            WinitCursorIcon::NotAllowed => CursorIcon::NotAllowed,
            WinitCursorIcon::ContextMenu => CursorIcon::ContextMenu,
            WinitCursorIcon::Cell => CursorIcon::Cell,
            WinitCursorIcon::VerticalText => CursorIcon::VerticalText,
            WinitCursorIcon::Alias => CursorIcon::Alias,
            WinitCursorIcon::Copy => CursorIcon::Copy,
            WinitCursorIcon::NoDrop => CursorIcon::NoDrop,
            WinitCursorIcon::Grab => CursorIcon::Grab,
            WinitCursorIcon::Grabbing => CursorIcon::Grabbing,
            WinitCursorIcon::AllScroll => CursorIcon::AllScroll,
            WinitCursorIcon::ZoomIn => CursorIcon::ZoomIn,
            WinitCursorIcon::ZoomOut => CursorIcon::ZoomOut,
            WinitCursorIcon::EResize => CursorIcon::EResize,
            WinitCursorIcon::NResize => CursorIcon::NResize,
            WinitCursorIcon::NeResize => CursorIcon::NeResize,
            WinitCursorIcon::NwResize => CursorIcon::NwResize,
            WinitCursorIcon::SResize => CursorIcon::SResize,
            WinitCursorIcon::SeResize => CursorIcon::SeResize,
            WinitCursorIcon::SwResize => CursorIcon::SwResize,
            WinitCursorIcon::WResize => CursorIcon::WResize,
            WinitCursorIcon::EwResize => CursorIcon::EwResize,
            WinitCursorIcon::NsResize => CursorIcon::NsResize,
            WinitCursorIcon::NeswResize => CursorIcon::NeswResize,
            WinitCursorIcon::NwseResize => CursorIcon::NwseResize,
            WinitCursorIcon::ColResize => CursorIcon::ColResize,
            WinitCursorIcon::RowResize => CursorIcon::RowResize,
        }
    }
}
