#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {
    None = 0,
    LeftButton = 1,
    RightButton = 2,
    Cancel = 3,
    MiddleButton = 4,
    XButton1 = 5,
    XButton2 = 6,
    Back = 8,
    Tab = 9,
    Clear = 10,
    Return = 11,
    Shift = 12,
    Control = 13,
    Menu = 14,
    Pause = 15,
    Capital = 16,
    KanaHangelHangul = 17,
    Junja = 18,
    Final = 19,
    HanjaKanji = 20,
    Escape = 21,
    Convert = 22,
    NonConvert = 23,
    Accept = 24,
    ModeChange = 25,
    Space = 26,
    Prior = 27,
    Next = 28,
    End = 29,
    Home = 30,
    Left = 31,
    Up = 32,
    Right = 33,
    Down = 34,
    Select = 35,
    Print = 36,
    Execute = 37,
    Snapshot = 38,
    Insert = 39,
    Delete = 40,
    Help = 41,
    Key0 = 42,
    Key1 = 43,
    Key2 = 44,
    Key3 = 45,
    Key4 = 46,
    Key5 = 47,
    Key6 = 48,
    Key7 = 49,
    Key8 = 50,
    Key9 = 51,
    A = 52,
    B = 53,
    C = 54,
    D = 55,
    E = 56,
    F = 57,
    G = 58,
    H = 59,
    I = 60,
    J = 61,
    K = 62,
    L = 63,
    M = 64,
    N = 65,
    O = 66,
    P = 67,
    Q = 68,
    R = 69,
    S = 70,
    T = 71,
    U = 72,
    V = 73,
    W = 74,
    X = 75,
    Y = 76,
    Z = 77,
    LeftWin = 78,
    RightWin = 79,
    Apps = 80,
    Sleep = 81,
    NumPad0 = 82,
    NumPad1 = 83,
    NumPad2 = 84,
    NumPad3 = 85,
    NumPad4 = 86,
    NumPad5 = 87,
    NumPad6 = 88,
    NumPad7 = 89,
    NumPad8 = 90,
    NumPad9 = 91,
    Multiply = 92,
    Add = 93,
    Separator = 94,
    Subtract = 95,
    Decimal = 96,
    Divide = 97,
    F1 = 98,
    F2 = 99,
    F3 = 100,
    F4 = 101,
    F5 = 102,
    F6 = 103,
    F7 = 104,
    F8 = 105,
    F9 = 106,
    F10 = 107,
    F11 = 108,
    F12 = 109,
    F13 = 110,
    F14 = 111,
    F15 = 112,
    F16 = 113,
    F17 = 114,
    F18 = 115,
    F19 = 116,
    F20 = 117,
    F21 = 118,
    F22 = 119,
    F23 = 120,
    F24 = 121,
    NumLock = 122,
    Scroll = 123,
    LeftShift = 124,
    RightShift = 125,
    LeftControl = 126,
    RightControl = 127,
    LeftMenu = 128,
    RightMenu = 129,
    BrowserBack = 130,
    BrowserForward = 131,
    BrowserRefresh = 132,
    BrowserStop = 133,
    BrowserSearch = 134,
    BrowserFavorites = 135,
    BrowserHome = 136,
    VolumeMute = 137,
    VolumeDown = 138,
    VolumeUp = 139,
    MediaNextTrack = 140,
    MediaPreviousTrack = 141,
    MediaStop = 142,
    MediaPlayPause = 143,
    LaunchMail = 144,
    LaunchMediaSelect = 145,
    LaunchApp1 = 146,
    LaunchApp2 = 147,
    Oem1 = 148,
    Plus = 149,
    Comma = 150,
    Minus = 151,
    Period = 152,
    Oem2 = 153,
    Oem3 = 154,
    Oem4 = 155,
    Oem5 = 156,
    Oem6 = 157,
    Oem7 = 158,
    Oem8 = 159,
    ProcessKey = 160,
    ImeProcess = 161,
    Packet = 162,
    Attn = 163,
    CrSel = 164,
    ExSel = 165,
    EraseEof = 166,
    Play = 167,
    Zoom = 168,
    PA1 = 169,
    OemClear = 170,
}

impl Key {
    pub fn to_u32(&self) -> u32 {
        match self {
            Key::None => 0,
            Key::LeftButton => 1,
            Key::RightButton => 2,
            Key::Cancel => 3,
            Key::MiddleButton => 4,
            Key::XButton1 => 5,
            Key::XButton2 => 6,
            Key::Back => 8,
            Key::Tab => 9,
            Key::Clear => 10,
            Key::Return => 11,
            Key::Shift => 12,
            Key::Control => 13,
            Key::Menu => 14,
            Key::Pause => 15,
            Key::Capital => 16,
            Key::KanaHangelHangul => 17,
            Key::Junja => 18,
            Key::Final => 19,
            Key::HanjaKanji => 20,
            Key::Escape => 21,
            Key::Convert => 22,
            Key::NonConvert => 23,
            Key::Accept => 24,
            Key::ModeChange => 25,
            Key::Space => 26,
            Key::Prior => 27,
            Key::Next => 28,
            Key::End => 29,
            Key::Home => 30,
            Key::Left => 31,
            Key::Up => 32,
            Key::Right => 33,
            Key::Down => 34,
            Key::Select => 35,
            Key::Print => 36,
            Key::Execute => 37,
            Key::Snapshot => 38,
            Key::Insert => 39,
            Key::Delete => 40,
            Key::Help => 41,
            Key::Key0 => 42,
            Key::Key1 => 43,
            Key::Key2 => 44,
            Key::Key3 => 45,
            Key::Key4 => 46,
            Key::Key5 => 47,
            Key::Key6 => 48,
            Key::Key7 => 49,
            Key::Key8 => 50,
            Key::Key9 => 51,
            Key::A => 52,
            Key::B => 53,
            Key::C => 54,
            Key::D => 55,
            Key::E => 56,
            Key::F => 57,
            Key::G => 58,
            Key::H => 59,
            Key::I => 60,
            Key::J => 61,
            Key::K => 62,
            Key::L => 63,
            Key::M => 64,
            Key::N => 65,
            Key::O => 66,
            Key::P => 67,
            Key::Q => 68,
            Key::R => 69,
            Key::S => 70,
            Key::T => 71,
            Key::U => 72,
            Key::V => 73,
            Key::W => 74,
            Key::X => 75,
            Key::Y => 76,
            Key::Z => 77,
            Key::LeftWin => 78,
            Key::RightWin => 79,
            Key::Apps => 80,
            Key::Sleep => 81,
            Key::NumPad0 => 82,
            Key::NumPad1 => 83,
            Key::NumPad2 => 84,
            Key::NumPad3 => 85,
            Key::NumPad4 => 86,
            Key::NumPad5 => 87,
            Key::NumPad6 => 88,
            Key::NumPad7 => 89,
            Key::NumPad8 => 90,
            Key::NumPad9 => 91,
            Key::Multiply => 92,
            Key::Add => 93,
            Key::Separator => 94,
            Key::Subtract => 95,
            Key::Decimal => 96,
            Key::Divide => 97,
            Key::F1 => 98,
            Key::F2 => 99,
            Key::F3 => 100,
            Key::F4 => 101,
            Key::F5 => 102,
            Key::F6 => 103,
            Key::F7 => 104,
            Key::F8 => 105,
            Key::F9 => 106,
            Key::F10 => 107,
            Key::F11 => 108,
            Key::F12 => 109,
            Key::F13 => 110,
            Key::F14 => 111,
            Key::F15 => 112,
            Key::F16 => 113,
            Key::F17 => 114,
            Key::F18 => 115,
            Key::F19 => 116,
            Key::F20 => 117,
            Key::F21 => 118,
            Key::F22 => 119,
            Key::F23 => 120,
            Key::F24 => 121,
            Key::NumLock => 122,
            Key::Scroll => 123,
            Key::LeftShift => 124,
            Key::RightShift => 125,
            Key::LeftControl => 126,
            Key::RightControl => 127,
            Key::LeftMenu => 128,
            Key::RightMenu => 129,
            Key::BrowserBack => 130,
            Key::BrowserForward => 131,
            Key::BrowserRefresh => 132,
            Key::BrowserStop => 133,
            Key::BrowserSearch => 134,
            Key::BrowserFavorites => 135,
            Key::BrowserHome => 136,
            Key::VolumeMute => 137,
            Key::VolumeDown => 138,
            Key::VolumeUp => 139,
            Key::MediaNextTrack => 140,
            Key::MediaPreviousTrack => 141,
            Key::MediaStop => 142,
            Key::MediaPlayPause => 143,
            Key::LaunchMail => 144,
            Key::LaunchMediaSelect => 145,
            Key::LaunchApp1 => 146,
            Key::LaunchApp2 => 147,
            Key::Oem1 => 148,
            Key::Plus => 149,
            Key::Comma => 150,
            Key::Minus => 151,
            Key::Period => 152,
            Key::Oem2 => 153,
            Key::Oem3 => 154,
            Key::Oem4 => 155,
            Key::Oem5 => 156,
            Key::Oem6 => 157,
            Key::Oem7 => 158,
            Key::Oem8 => 159,
            Key::ProcessKey => 160,
            Key::ImeProcess => 161,
            Key::Packet => 162,
            Key::Attn => 163,
            Key::CrSel => 164,
            Key::ExSel => 165,
            Key::EraseEof => 166,
            Key::Play => 167,
            Key::Zoom => 168,
            Key::PA1 => 169,
            Key::OemClear => 170,
        }
    }
}