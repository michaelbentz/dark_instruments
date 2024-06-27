//!
//! Android key codes
//! See [link](https://android.googlesource.com/platform/frameworks/base/+/master/core/java/android/view/KeyEvent.java)
//!
#[repr(u32)]
pub enum KeyCode {
    /// Key code constant: Unknown key code.
    Unknown = 0,
    /// Key code constant: Soft Left key.
    ///
    /// Usually situated below the display on phones and used as a multi-function feature key for
    /// selecting a software defined function shown on the bottom left of the display.
    SoftLeft = 1,
    /// Key code constant: Soft Right key.
    ///
    /// Usually situated below the display on phones and used as a multi-function feature key for
    /// selecting a software defined function shown on the bottom right of the display.
    SoftRight = 2,
    /// Key code constant: Home key.
    ///
    /// This key is handled by the framework and is never delivered to applications.
    Home = 3,
    /// Key code constant: Back key.
    Back = 4,
    /// Key code constant: Call key.
    Call = 5,
    /// Key code constant: End Call key.
    EndCall = 6,
    /// Key code constant: '0' key.
    _0 = 7,
    /// Key code constant: '1' key.
    _1 = 8,
    /// Key code constant: '2' key.
    _2 = 9,
    /// Key code constant: '3' key.
    _3 = 10,
    /// Key code constant: '4' key.
    _4 = 11,
    /// Key code constant: '5' key.
    _5 = 12,
    /// Key code constant: '6' key.
    _6 = 13,
    /// Key code constant: '7' key.
    _7 = 14,
    /// Key code constant: '8' key.
    _8 = 15,
    /// Key code constant: '9' key.
    _9 = 16,
    /// Key code constant: '*' key.
    Star = 17,
    /// Key code constant: '#' key.
    Found = 18,
    /// Key code constant: Directional Pad Up key.
    ///
    /// May also be synthesized from trackball motions.
    DpadUp = 19,
    /// Key code constant: Directional Pad Down key.
    ///
    /// May also be synthesized from trackball motions.
    DpadDown = 20,
    /// Key code constant: Directional Pad Left key.
    ///
    /// May also be synthesized from trackball motions.
    DpadLeft = 21,
    /// Key code constant: Directional Pad Right key.
    ///
    /// May also be synthesized from trackball motions.
    DpadRight = 22,
    /// Key code constant: Directional Pad Center key.
    ///
    /// May also be synthesized from trackball motions.
    DpadCenter = 23,
    /// Key code constant: Volume Up key.
    ///
    /// Adjusts the speaker volume up.
    VolumeUp = 24,
    /// Key code constant: Volume Down key.
    ///
    /// Adjusts the speaker volume down.
    VolumeDown = 25,
    /// Key code constant: Power key.
    Power = 26,
    /// Key code constant: Camera key.
    ///
    /// Used to launch a camera application or take pictures.
    Camera = 27,
    /// Key code constant: Clear key.
    CLEAR = 28,
    /// Key code constant: 'A' key.
    A = 29,
    /// Key code constant: 'B' key.
    B = 30,
    /// Key code constant: 'C' key.
    C = 31,
    /// Key code constant: 'D' key.
    D = 32,
    /// Key code constant: 'E' key.
    E = 33,
    /// Key code constant: 'F' key.
    F = 34,
    /// Key code constant: 'G' key.
    G = 35,
    /// Key code constant: 'H' key.
    H = 36,
    /// Key code constant: 'I' key.
    I = 37,
    /// Key code constant: 'J' key.
    J = 38,
    /// Key code constant: 'K' key.
    K = 39,
    /// Key code constant: 'L' key.
    L = 40,
    /// Key code constant: 'M' key.
    M = 41,
    /// Key code constant: 'N' key.
    N = 42,
    /// Key code constant: 'O' key.
    O = 43,
    /// Key code constant: 'P' key.
    P = 44,
    /// Key code constant: 'Q' key.
    Q = 45,
    /// Key code constant: 'R' key.
    R = 46,
    /// Key code constant: 'S' key.
    S = 47,
    /// Key code constant: 'T' key.
    T = 48,
    /// Key code constant: 'U' key.
    U = 49,
    /// Key code constant: 'V' key.
    V = 50,
    /// Key code constant: 'W' key.
    W = 51,
    /// Key code constant: 'X' key.
    X = 52,
    /// Key code constant: 'Y' key.
    Y = 53,
    /// Key code constant: 'Z' key.
    Z = 54,
    /// Key code constant: ',' key.
    Comma = 55,
    /// Key code constant: '.' key.
    Period = 56,
    /// Key code constant: Left Alt modifier key.
    AltLeft = 57,
    /// Key code constant: Right Alt modifier key.
    AltRight = 58,
    /// Key code constant: Left Shift modifier key.
    ShiftLeft = 59,
    /// Key code constant: Right Shift modifier key.
    ShiftRight = 60,
    /// Key code constant: Tab key.
    Tab = 61,
    /// Key code constant: Space key.
    Space = 62,
    /// Key code constant: Symbol modifier key.
    ///
    /// Used to enter alternate symbols.
    Sym = 63,
    /// Key code constant: Explorer special function key.
    ///
    /// Used to launch a browser application.
    Explorer = 64,
    /// Key code constant: Envelope special function key.
    ///
    /// Used to launch a mail application.
    Envelope = 65,
    /// Key code constant: Enter key.
    Enter = 66,
    /// Key code constant: Backspace key.
    ///
    /// Deletes characters before the insertion point, unlike [ForwardDel].
    Del = 67,
    /// Key code constant: '`' (backtick) key.
    Grave = 68,
    /// Key code constant: '-'.
    Minus = 69,
    /// Key code constant: '=' key.
    Equals = 70,
    /// Key code constant: '[' key.
    LeftBracket = 71,
    /// Key code constant: ']' key.
    RightBracket = 72,
    /// Key code constant: '\' key.
    Backslash = 73,
    /// Key code constant: ',' key.
    Semicolon = 74,
    /// Key code constant: ''' (apostrophe) key.
    Apostrophe = 75,
    /// Key code constant: '/' key.
    Slash = 76,
    /// Key code constant: '@' key.
    At = 77,
    /// Key code constant: Number modifier key.
    ///
    /// Used to enter numeric symbols. This key is not Num Lock, it is more like [AltLeft]
    /// and is interpreted as an ALT key by android.text.method.MetaKeyKeyListener.
    Num = 78,
    /// Key code constant: Headset Hook key.
    ///
    /// Used to hang up calls and stop media.
    HeadsetHook = 79,
    /// Key code constant: Camera Focus key.
    ///
    /// Used to focus the camera.
    Focus = 80,
    /// Key code constant: '+' key.
    Plus = 81,
    /// Key code constant: Menu key.
    Menu = 82,
    /// Key code constant: Notification key.
    Notification = 83,
    /// Key code constant: Search key.
    Search = 84,
    /// Key code constant: Play/Pause media key.
    MediaPlayPause = 85,
    /// Key code constant: Stop media key.
    MediaStop = 86,
    /// Key code constant: Play Next media key.
    MediaNext = 87,
    /// Key code constant: Play Previous media key.
    MediaPrevious = 88,
    /// Key code constant: Rewind media key.
    MediaRewind = 89,
    /// Key code constant: Fast Forward media key.
    MediaFastForward = 90,
    /// Key code constant: Mute key.
    ///
    /// Mute key for the microphone (unlike [VolumeMute], which is the speaker mute key).
    Mute = 91,
    /// Key code constant: Page Up key.
    PageUp = 92,
    /// Key code constant: Page Down key.
    PageDown = 93,
    /// Key code constant: Picture Symbols modifier key.
    ///
    /// Used to switch symbol sets (Emoji, Kao-moji).
    PictSymbols = 94,
    /// Key code constant: Switch Charset modifier key.
    ///
    /// Used to switch character sets (Kanji, Katakana).
    SwitchCharset = 95,
    /// Key code constant: A Button key.
    ///
    /// On a game controller, the A button should be either the button labeled A or the first
    /// button on the bottom row of controller buttons.
    ButtonA = 96,
    /// Key code constant: B Button key.
    ///
    /// On a game controller, the B button should be either the button labeled B or the second
    /// button on the bottom row of controller buttons.
    ButtonB = 97,
    /// Key code constant: C Button key.
    ///
    /// On a game controller, the C button should be either the button labeled C or the third
    /// button on the bottom row of controller buttons.
    ButtonC = 98,
    /// Key code constant: X Button key.
    ///
    /// On a game controller, the X button should be either the button labeled X or the first
    /// button on the upper row of controller buttons.
    ButtonX = 99,
    /// Key code constant: Y Button key.
    ///
    /// On a game controller, the Y button should be either the button labeled Y or the second
    /// button on the upper row of controller buttons.
    ButtonY = 100,
    /// Key code constant: Z Button key.
    ///
    /// On a game controller, the Z button should be either the button labeled Z or the third
    /// button on the upper row of controller buttons.
    ButtonZ = 101,
    /// Key code constant: L1 Button key.
    ///
    /// On a game controller, the L1 button should be either the button labeled L1 (or L) or the
    /// top left trigger button.
    ButtonL1 = 102,
    /// Key code constant: R1 Button key.
    ///
    /// On a game controller, the R1 button should be either the button labeled R1 (or R) or the
    /// top right trigger button.
    ButtonR1 = 103,
    /// Key code constant: L2 Button key.
    ///
    /// On a game controller, the L2 button should be either the button labeled L2 or the bottom
    /// left trigger button.
    ButtonL2 = 104,
    /// Key code constant: R2 Button key.
    ///
    /// On a game controller, the R2 button should be either the button labeled R2 or the bottom
    /// right trigger button.
    ButtonR2 = 105,
    /// Key code constant: Left Thumb Button key.
    ///
    /// On a game controller, the left thumb button indicates that the left (or only) joystick
    /// is pressed.
    ButtonThumbL = 106,
    /// Key code constant: Right Thumb Button key.
    ///
    /// On a game controller, the right thumb button indicates that the right joystick is
    /// pressed.
    ButtonThumbR = 107,
    /// Key code constant: Start Button key.
    ///
    /// On a game controller, the button labeled Start.
    ButtonStart = 108,
    /// Key code constant: Select Button key.
    ///
    /// On a game controller, the button labeled Select.
    ButtonSelect = 109,
    /// Key code constant: Mode Button key.
    ///
    /// On a game controller, the button labeled Mode.
    ButtonMode = 110,
    /// Key code constant: Escape key.
    Escape = 111,
    /// Key code constant: Forward Delete key.
    ///
    /// Deletes characters ahead of the insertion point, unlike [Del].
    ForwardDel = 112,
    /// Key code constant: Left Control modifier key.
    CtrlLeft = 113,
    /// Key code constant: Right Control modifier key.
    CtrlRight = 114,
    /// Key code constant: Caps Lock key.
    CapsLock = 115,
    /// Key code constant: Scroll Lock key.
    ScrollLock = 116,
    /// Key code constant: Left Meta modifier key.
    MetaLeft = 117,
    /// Key code constant: Right Meta modifier key.
    MetaRight = 118,
    /// Key code constant: Function modifier key.
    Function = 119,
    /// Key code constant: System Request / Print Screen key.
    SysRq = 120,
    /// Key code constant: Break / Pause key.
    BREAK = 121,
    /// Key code constant: Home Movement key.
    ///
    /// Used for scrolling or moving the cursor around to the start of a line or to the top
    /// of a list.
    MoveHome = 122,
    /// Key code constant: End Movement key.
    ///
    /// Used for scrolling or moving the cursor around to the end of a line or to the bottom
    /// of a list.
    MoveEnd = 123,
    /// Key code constant: Insert key.
    ///
    /// Toggles insert / overwrite edit mode.
    Insert = 124,
    /// Key code constant: Forward key.
    ///
    /// Navigates forward in the history stack. Complement of [Back].
    Forward = 125,
    /// Key code constant: Play media key.
    MediaPlay = 126,
    /// Key code constant: Pause media key.
    MediaPause = 127,
    /// Key code constant: Close media key.
    ///
    /// May be used to close a CD tray, for example.
    MediaClose = 128,
    /// Key code constant: Eject media key.
    ///
    /// May be used to eject a CD tray, for example.
    MediaEject = 129,
    /// Key code constant: Record media key.
    MediaRecord = 130,
    /// Key code constant: F1 key.
    F1 = 131,
    /// Key code constant: F2 key.
    F2 = 132,
    /// Key code constant: F3 key.
    F3 = 133,
    /// Key code constant: F4 key.
    F4 = 134,
    /// Key code constant: F5 key.
    F5 = 135,
    /// Key code constant: F6 key.
    F6 = 136,
    /// Key code constant: F7 key.
    F7 = 137,
    /// Key code constant: F8 key.
    F8 = 138,
    /// Key code constant: F9 key.
    F9 = 139,
    /// Key code constant: F10 key.
    F10 = 140,
    /// Key code constant: F11 key.
    F11 = 141,
    /// Key code constant: F12 key.
    F12 = 142,
    /// Key code constant: Num Lock key.
    ///
    /// This is the Num Lock key, it is different from [Num]. This key alters the
    /// behavior of other keys on the numeric keypad.
    NumLock = 143,
    /// Key code constant: Numeric keypad '0' key.
    Numpad0 = 144,
    /// Key code constant: Numeric keypad '1' key.
    Numpad1 = 145,
    /// Key code constant: Numeric keypad '2' key.
    Numpad2 = 146,
    /// Key code constant: Numeric keypad '3' key.
    Numpad3 = 147,
    /// Key code constant: Numeric keypad '4' key.
    Numpad4 = 148,
    /// Key code constant: Numeric keypad '5' key.
    Numpad5 = 149,
    /// Key code constant: Numeric keypad '6' key.
    Numpad6 = 150,
    /// Key code constant: Numeric keypad '7' key.
    Numpad7 = 151,
    /// Key code constant: Numeric keypad '8' key.
    Numpad8 = 152,
    /// Key code constant: Numeric keypad '9' key.
    Numpad9 = 153,
    /// Key code constant: Numeric keypad '/' key (for division).
    NumpadDivide = 154,
    /// Key code constant: Numeric keypad '*' key (for multiplication).
    NumpadMultiply = 155,
    /// Key code constant: Numeric keypad '-' key (for subtraction).
    NumpadSubtract = 156,
    /// Key code constant: Numeric keypad '+' key (for addition).
    NumpadAdd = 157,
    /// Key code constant: Numeric keypad '.' key (for decimals or digit grouping).
    NumpadDot = 158,
    /// Key code constant: Numeric keypad ',' key (for decimals or digit grouping).
    NumpadComma = 159,
    /// Key code constant: Numeric keypad Enter key.
    NumpadEnter = 160,
    /// Key code constant: Numeric keypad '=' key.
    NumpadEquals = 161,
    /// Key code constant: Numeric keypad '(' key.
    NumpadLeftParen = 162,
    /// Key code constant: Numeric keypad ')' key.
    NumpadRightParen = 163,
    /// Key code constant: Volume Mute key.
    ///
    /// Mute key for speaker (unlike [Mute], which is the mute key for the microphone).
    /// This key should normally be implemented as a toggle such that the first press mutes the
    /// speaker and the second press restores the original volume.
    VolumeMute = 164,
    /// Key code constant: Info key.
    ///
    /// Common on TV remotes to show additional information related to what is currently being viewed.
    Info = 165,
    /// Key code constant: Channel up key.
    ///
    /// On TV remotes, increments the television channel.
    ChannelUp = 166,
    /// Key code constant: Channel down key.
    ///
    /// On TV remotes, decrements the television channel.
    ChannelDown = 167,
    /// Key code constant: Zoom in key.
    ZoomIn = 168,
    /// Key code constant: Zoom out key.
    ZoomOut = 169,
    /// Key code constant: TV key.
    ///
    /// On TV remotes, switches to viewing live TV.
    Tv = 170,
    /// Key code constant: Window key.
    ///
    /// On TV remotes, toggles picture-in-picture mode or other windowing functions. On Android
    /// Wear devices, triggers a display offset.
    Window = 171,
    /// Key code constant: Guide key.
    ///
    /// On TV remotes, shows a programming guide.
    Guide = 172,
    /// Key code constant: DVR key.
    ///
    /// On some TV remotes, switches to a DVR mode for recorded shows.
    Dvr = 173,
    /// Key code constant: Bookmark key.
    ///
    /// On some TV remotes, bookmarks content or web pages.
    Bookmark = 174,
    /// Key code constant: Toggle captions key.
    ///
    /// Switches the mode for closed-captioning text, for example during television shows.
    Captions = 175,
    /// Key code constant: Settings key.
    ///
    /// Starts the system settings activity.
    Settings = 176,
    /// Key code constant: TV power key.
    ///
    /// On HDMI TV panel devices and Android TV devices that don't support HDMI, toggles the power
    /// state of the device. On HDMI source devices, toggles the power state of the HDMI-connected
    /// TV via HDMI-CEC and makes the source device follow this power state.
    TvPower = 177,
    /// Key code constant: TV input key.
    ///
    /// On TV remotes, switches the input on a television screen.
    TvInput = 178,
    /// Key code constant: Set-top-box power key.
    ///
    /// On TV remotes, toggles the power on an external Set-top-box.
    StbPower = 179,
    /// Key code constant: Set-top-box input key.
    ///
    /// On TV remotes, switches the input mode on an external Set-top-box.
    StbInput = 180,
    /// Key code constant: A/V Receiver power key.
    ///
    /// On TV remotes, toggles the power on an external A/V Receiver.
    AvrPower = 181,
    /// Key code constant: A/V Receiver input key.
    ///
    /// On TV remotes, switches the input mode on an external A/V Receiver.
    AvrInput = 182,
    /// Key code constant: Red "programmable" key.
    ///
    /// On TV remotes, acts as a contextual/programmable key.
    ProgRed = 183,
    /// Key code constant: Green "programmable" key.
    ///
    /// On TV remotes, actsas a contextual/programmable key.
    ProgGreen = 184,
    /// Key code constant: Yellow "programmable" key.
    ///
    /// On TV remotes, acts as a contextual/programmable key.
    ProgYellow = 185,
    /// Key code constant: Blue "programmable" key.
    ///
    /// On TV remotes, acts as a contextual/programmable key.
    ProgBlue = 186,
    /// Key code constant: App switch key.
    ///
    /// Should bring up the application switcher dialog.
    AppSwitch = 187,
    /// Key code constant: Generic Game Pad Button #1.
    Button1 = 188,
    /// Key code constant: Generic Game Pad Button #2.
    Button2 = 189,
    /// Key code constant: Generic Game Pad Button #3.
    Button3 = 190,
    /// Key code constant: Generic Game Pad Button #4.
    Button4 = 191,
    /// Key code constant: Generic Game Pad Button #5.
    Button5 = 192,
    /// Key code constant: Generic Game Pad Button #6.
    Button6 = 193,
    /// Key code constant: Generic Game Pad Button #7.
    Button7 = 194,
    /// Key code constant: Generic Game Pad Button #8.
    Button8 = 195,
    /// Key code constant: Generic Game Pad Button #9.
    Button9 = 196,
    /// Key code constant: Generic Game Pad Button #10.
    Button10 = 197,
    /// Key code constant: Generic Game Pad Button #11.
    Button11 = 198,
    /// Key code constant: Generic Game Pad Button #12.
    Button12 = 199,
    /// Key code constant: Generic Game Pad Button #13.
    Button13 = 200,
    /// Key code constant: Generic Game Pad Button #14.
    Button14 = 201,
    /// Key code constant: Generic Game Pad Button #15.
    Button15 = 202,
    /// Key code constant: Generic Game Pad Button #16.
    Button16 = 203,
    /// Key code constant: Language Switch key.
    ///
    /// Toggles the current input language such as switching between English and Japanese on
    /// a QWERTY keyboard.  On some devices, the same function may be performed by pressing
    /// Shift+Spacebar.
    LanguageSwitch = 204,
    /// Key code constant: Manner Mode key.
    ///
    /// Toggles silent or vibrate mode on and off to make the device behave more politely
    /// in certain settings such as on a crowded train.  On some devices, the key may only
    /// operate when long-pressed.
    MannerMode = 205,
    /// Key code constant: 3D Mode key.
    ///
    /// Toggles the display between 2D and 3D mode.
    _3dMode = 206,
    /// Key code constant: Contacts special function key.
    ///
    /// Used to launch an address book application.
    Contacts = 207,
    /// Key code constant: Calendar special function key.
    ///
    /// Used to launch a calendar application.
    Calendar = 208,
    /// Key code constant: Music special function key.
    ///
    /// Used to launch a music player application.
    Music = 209,
    /// Key code constant: Calculator special function key.
    ///
    /// Used to launch a calculator application.
    Calculator = 210,
    /// Key code constant: Japanese full-width / half-width key.
    ZenkakuHankaku = 211,
    /// Key code constant: Japanese alphanumeric key.
    Eisu = 212,
    /// Key code constant: Japanese non-conversion key.
    Muhenkan = 213,
    /// Key code constant: Japanese conversion key.
    Henkan = 214,
    /// Key code constant: Japanese katakana / hiragana key.
    KatakanaHiragana = 215,
    /// Key code constant: Japanese Yen key.
    Yen = 216,
    /// Key code constant: Japanese Ro key.
    Ro = 217,
    /// Key code constant: Japanese kana key.
    Kana = 218,
    /// Key code constant: Assist key.
    ///
    /// Launches the global assist activity. **Not delivered to applications.**
    Assist = 219,
    /// Key code constant: Brightness Down key.
    ///
    /// Adjusts the screen brightness down.
    BrightnessDown = 220,
    /// Key code constant: Brightness Up key.
    ///
    /// Adjusts the screen brightness up.
    BrightnessUp = 221,
    /// Key code constant: Audio Track key.
    /// Switches the audio tracks.
    MediaAudioTrack = 222,
    /// Key code constant: Sleep key.
    ///
    /// Puts the device to sleep. Behaves somewhat like [Power] but it has no effect if
    /// the device is already asleep.
    Sleep = 223,
    /// Key code constant: Wakeup key
    /// .
    /// Wakes up the device. Behaves somewhat like [Power] but it has no effect if the
    /// device is already awake.
    Wakeup = 224,
    /// Key code constant: Pairing key.
    ///
    /// Initiates peripheral pairing mode. Useful for pairing remote control devices or game
    /// controllers, especially if no other input mode is available.
    Pairing = 225,
    /// Key code constant: Media Top Menu key.
    ///
    /// Goes to the top of media menu.
    MediaTopMenu = 226,
    /// Key code constant: '11' key.
    _11 = 227,
    /// Key code constant: '12' key.
    _12 = 228,
    /// Key code constant: Last Channel key.
    ///
    /// Goes to the last viewed channel.
    LastChannel = 229,
    /// Key code constant: TV data service key.
    ///
    /// Displays data services like weather, sports.
    TvDataService = 230,
    /// Key code constant: Voice Assist key.
    ///
    /// Launches the global voice assist activity. Not delivered to applications.
    VoiceAssist = 231,
    /// Key code constant: Radio key.
    ///
    /// Toggles TV service / Radio service.
    TvRadioService = 232,
    /// Key code constant: Teletext key.
    ///
    /// Displays Teletext service.
    TvTeletext = 233,
    /// Key code constant: Number entry key.
    ///
    /// Initiates to enter multi-digit channel number when each digit key is assigned for
    /// selecting separate channel. Corresponds to Number Entry Mode (0x1D) of CEC User Control Code.
    TvNumberEntry = 234,
    /// Key code constant: Analog Terrestrial key.
    ///
    /// Switches to analog terrestrial broadcast service.
    TvTerrestrialAnalog = 235,
    /// Key code constant: Digital Terrestrial key.
    ///
    /// Switches to digital terrestrial broadcast service.
    TvTerrestrialDigital = 236,
    /// Key code constant: Satellite key.
    ///
    /// Switches to digital satellite broadcast service.
    TvSatellite = 237,
    /// Key code constant: BS key.
    /// Switches to BS digital satellite broadcasting service available in Japan.
    TvSatelliteBs = 238,
    /// Key code constant: CS key.
    /// Switches to CS digital satellite broadcasting service available in Japan.
    TvSatelliteCs = 239,
    /// Key code constant: BS/CS key.
    ///
    /// Toggles between BS and CS digital satellite services.
    TvSatelliteService = 240,
    /// Key code constant: Toggle Network key.
    ///
    /// Toggles selecting broadcast services.
    TvNetwork = 241,
    /// Key code constant: Antenna/Cable key.
    ///
    /// Toggles broadcast input source between antenna and cable.
    TvAntennaCable = 242,
    /// Key code constant: HDMI #1 key.
    ///
    /// Switches to HDMI input #1.
    TvInputHdmi1 = 243,
    /// Key code constant: HDMI #2 key.
    ///
    /// Switches to HDMI input #2.
    TvInputHdmi2 = 244,
    /// Key code constant: HDMI #3 key.
    ///
    /// Switches to HDMI input #3.
    TvInputHdmi3 = 245,
    /// Key code constant: HDMI #4 key.
    ///
    /// Switches to HDMI input #4.
    TvInputHdmi4 = 246,
    /// Key code constant: Composite #1 key.
    ///
    /// Switches to composite video input #1.
    TvInputComposite1 = 247,
    /// Key code constant: Composite #2 key.
    ///
    /// Switches to composite video input #2.
    TvInputComposite2 = 248,
    /// Key code constant: Component #1 key.
    ///
    /// Switches to component video input #1.
    TvInputComponent1 = 249,
    /// Key code constant: Component #2 key.
    ///
    /// Switches to component video input #2.
    TvInputComponent2 = 250,
    /// Key code constant: VGA #1 key.
    ///
    /// Switches to VGA (analog RGB) input #1.
    TvInputVga1 = 251,
    /// Key code constant: Audio description key.
    ///
    /// Toggles audio description off / on.
    TvAudioDescription = 252,
    /// Key code constant: Audio description mixing volume up key.
    ///
    /// Louden audio description volume as compared with normal audio volume.
    TvAudioDescriptionMixUp = 253,
    /// Key code constant: Audio description mixing volume down key.
    ///
    /// Lessen audio description volume as compared with normal audio volume.
    TvAudioDescriptionMixDown = 254,
    /// Key code constant: Zoom mode key.
    ///
    /// Changes Zoom mode (Normal, Full, Zoom, Wide-zoom, etc.)
    TvZoomMode = 255,
    /// Key code constant: Contents menu key.
    ///
    /// Goes to the title list. Corresponds to Contents Menu (0x0B) of CEC User Control Code
    TvContentsMenu = 256,
    /// Key code constant: Media context menu key.
    ///
    /// Goes to the context menu of media contents. Corresponds to Media Context-sensitive
    /// Menu (0x11) of CEC User Control Code.
    TvMediaContextMenu = 257,
    /// Key code constant: Timer programming key.
    ///
    /// Goes to the timer recording menu. Corresponds to Timer Programming (0x54) of CEC
    /// User Control Code.
    TvTimerProgramming = 258,
    /// Key code constant: Help key.
    Help = 259,
    /// Key code constant: Navigate to previous key.
    ///
    /// Goes backward by one item in an ordered collection of items.
    NavigatePrevious = 260,
    /// Key code constant: Navigate to next key.
    ///
    /// Advances to the next item in an ordered collection of items.
    NavigateNext = 261,
    /// Key code constant: Navigate in key.
    ///
    /// Activates the item that currently has focus or expands to the next level of a navigation
    /// hierarchy.
    NavigateIn = 262,
    /// Key code constant: Navigate out key.
    ///
    /// Backs out one level of a navigation hierarchy or collapses the item that currently has
    /// focus.
    NavigateOut = 263,
    /// Key code constant: Primary stem key for Wear
    ///
    /// Main power/reset button on watch.
    StemPrimary = 264,
    /// Key code constant: Generic stem key 1 for Wear
    Stem1 = 265,
    /// Key code constant: Generic stem key 2 for Wear
    Stem2 = 266,
    /// Key code constant: Generic stem key 3 for Wear
    Stem3 = 267,
    /// Key code constant: Directional Pad Up-Left
    DpadUpLeft = 268,
    /// Key code constant: Directional Pad Down-Left
    DpadDownLeft = 269,
    /// Key code constant: Directional Pad Up-Right
    DpadUpRight = 270,
    /// Key code constant: Directional Pad Down-Right
    DpadDownRight = 271,
    /// Key code constant: Skip forward media key.
    MediaSkipForward = 272,
    /// Key code constant: Skip backward media key.
    MediaSkipBackward = 273,
    /// Key code constant: Step forward media key.
    ///
    /// Steps media forward, one frame at a time.
    MediaStepForward = 274,
    /// Key code constant: Step backward media key.
    ///
    /// Steps media backward, one frame at a time.
    MediaStepBackward = 275,
    /// Key code constant: put device to sleep unless a wakelock is held.
    SoftSleep = 276,
    /// Key code constant: Cut key.
    Cut = 277,
    /// Key code constant: Copy key.
    Copy = 278,
    /// Key code constant: Paste key.
    Paste = 279,
    /// Key code constant: Consumed by the system for navigation up
    SystemNavigationUp = 280,
    /// Key code constant: Consumed by the system for navigation down
    SystemNavigationDown = 281,
    /// Key code constant: Consumed by the system for navigation left
    SystemNavigationLeft = 282,
    /// Key code constant: Consumed by the system for navigation right
    SystemNavigationRight = 283,
    /// Key code constant: Show all apps
    AllApps = 284,
    /// Key code constant: Refresh key.
    Refresh = 285,
    /// Key code constant: Thumbs up key. Apps can use this to let user upvote content.
    ThumbsUp = 286,
    /// Key code constant: Thumbs down key. Apps can use this to let user downvote content.
    ThumbsDown = 287,
    /// Key code constant:
    ///
    /// Used to switch current Account that is consuming content.
    /// May be consumed by system to set account globally.
    ProfileSwitch = 288,
    /// Key code constant: Video Application key #1.
    VideoApp1 = 289,
    /// Key code constant: Video Application key #2.
    VideoApp2 = 290,
    /// Key code constant: Video Application key #3.
    VideoApp3 = 291,
    /// Key code constant: Video Application key #4.
    VideoApp4 = 292,
    /// Key code constant: Video Application key #5.
    VideoApp5 = 293,
    /// Key code constant: Video Application key #6.
    VideoApp6 = 294,
    /// Key code constant: Video Application key #7.
    VideoApp7 = 295,
    /// Key code constant: Video Application key #8.
    VideoApp8 = 296,
    /// Key code constant: Featured Application key #1.
    FeaturedApp1 = 297,
    /// Key code constant: Featured Application key #2.
    FeaturedApp2 = 298,
    /// Key code constant: Featured Application key #3.
    FeaturedApp3 = 299,
    /// Key code constant: Featured Application key #4.
    FeaturedApp4 = 300,
    /// Key code constant: Demo Application key #1.
    DemoApp1 = 301,
    /// Key code constant: Demo Application key #2.
    DemoApp2 = 302,
    /// Key code constant: Demo Application key #3.
    DemoApp3 = 303,
    /// Key code constant: Demo Application key #4.
    DemoApp4 = 304,
    /// Key code constant: Keyboard backlight down
    KeyboardBacklightDown = 305,
    /// Key code constant: Keyboard backlight up
    KeyboardBacklightUp = 306,
    /// Key code constant: Keyboard backlight toggle
    KeyboardBacklightToggle = 307,
    ///Key code constant: The primary button on the barrel of a stylus.
    ///
    ///This is usually the button closest to the tip of the stylus.
    StylusButtonPrimary = 308,
    ///Key code constant: The secondary button on the barrel of a stylus.
    ///
    ///This is usually the second button from the tip of the stylus.
    StylusButtonSecondary = 309,
    ///Key code constant: The tertiary button on the barrel of a stylus.
    ///
    ///This is usually the third button from the tip of the stylus.
    StylusButtonTertiary = 310,
    ///Key code constant: A button on the tail end of a stylus.
    ///
    ///The use of this button does not usually correspond to the function of an eraser.
    StylusButtonTail = 311,
    ///Key code constant: To open recent apps view (a.k.a. Overview).
    ///
    ///This key is handled by the framework and is never delivered to applications.
    RecentApps = 312,
    /// Key code constant: A button whose usage can be customized by the user through the system.
    ///
    /// User customizable key #1.
    Macro1 = 313,
    /// Key code constant: A button whose usage can be customized by the user through the system.
    ///
    /// User customizable key #2.
    Macro2 = 314,
    /// Key code constant: A button whose usage can be customized by the user through the system.
    ///
    /// User customizable key #3.
    Macro3 = 315,
    /// Key code constant: A button whose usage can be customized by the user through the system.
    ///
    /// User customizable key #4.
    Macro4 = 316,
}