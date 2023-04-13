use super::data::Asset;

enum StringTableEntries
{
    // TEMP_NOT_IMPLEMENTED
    TempNotImplemented,
    // PRESS_START
    PressStart,
    // CONTINUE
    Continue,
    // LANGUAGE_ENGLISH_UK
    LanguageEnglishUk,
    // LANGUAGE_ENGLISH_US
    LanguageEnglishUs,
    // LANGUAGE_FRENCH
    LanguageFrench,
    // LANGUAGE_ITALIAN
    LanguageItalian,
    // LANGUAGE_GERMAN
    LanguageGerman,
    // LANGUAGE_SPANISH
    LanguageSpanish,
    // MAIN_START
    MainMenuStart,
    // MAIN_OPTIONS
    MainMenuOptions,
    // MAIN_GAME_SERVICES
    MainMenuGameServices,
    // MAIN_HIGH_SCORES
    MainMenuHighScores,
    // MAIN_MULTIPLAYER
    MainMenuMultiplayer,
    // MAIN_QUIT
    MainMenuQuit,
    // START_NEW_GAME
    StartNewGame,
    // START_LOAD_GAME
    StartLoadGame,
    // DIFFICULTY_EASY
    DifficultyEasy,
    // DIFFICULTY_NORMAL
    DifficultyNormal,
    // DIFFICULTY_HARD
    DifficultyHard,
    // OPTIONS_WEAPON_SWITCH
    OptionsWeaponSwitch,
    // OPTIONS_CONTRAST_OVERRIDE
    OptionsContrastOverride,
    // OPTIONS_SFX_VOLUME
    OptionsSfxVolume,
    // OPTIONS_MUSIC_VOLUME
    OptionsMusicVolume,
    // OPTIONS_GAMMA
    OptionsGamma,
    // OPTIONS_CROSSHAIR
    OptionsCrosshair,
    // OPTIONS_CONTROLS
    OptionsControls,
    // OPTIONS_CREDITS
    OptionsCredits,
    // OPTIONS_MUTE_IN_CALL
    OptionsMuteInCall,
    // OPTIONS_CONTROLLER_TYPE
    OptionsControllerType,
    // OPTIONS_AUDIO_VIDEO
    OptionsAudioVideo,
    // GAME_LEVEL1
    GameLevel1,
    // GAME_LEVEL2
    GameLevel2,
    // GAME_LEVEL3
    GameLevel3,
    // GAME_LEVEL4
    GameLevel4,
    // GAME_LEVEL5
    GameLevel5,
    // GAME_LEVEL6
    GameLevel6,
    // GAME_LEVEL7
    GameLevel7,
    // GAME_LEVEL8
    GameLevel8,
    // GAME_LEVELDM1
    GameLevelDm1,
    // GAME_LEVELDM2
    GameLevelDm2,
    // GAME_LEVELDM3
    GameLevelDm3,
    // GAME_LEVELDM4
    GameLevelDm4,
    // CHEAT_GOD
    CheatGod,
    // CHEAT_AMMO
    CheatAmmo,
    // HIGHSCORES_VIEW
    HighscoresView,
    // HIGHSCORES_SEND
    HighscoresSend,
    // HIGHSCORES_CLEAR
    HighscoresClear,
    // GAME_TOTAL
    GameTotal,
    // SLOT_EMPTY
    SlotEmpty,
    // MULTI_HOST_GAME
    MultiHostGame,
    // MULTI_JOIN_GAME
    MultiJoinGame,
    // HOST_LAUNCH
    HostLaunch,
    // HOST_LAUNCH_TEXT1
    HostLaunchText1,
    // HOST_LAUNCH_TEXT2
    HostLaunchText2,
    // SELECT_HOST
    SelectHost,
    // CLIENT_JOIN
    ClientJoin,
    // CLIENT_CONNECTING
    ClientConnecting,
    // CLIENT_WAITING_TEXT1
    ClientWaitingText1,
    // CLIENT_WAITING_TEXT2
    ClientWaitingText2,
    // BLUETOOTH_TEXT
    BluetoothText,
    // YES
    Yes,
    // NO
    No,
    // OK
    Ok,
    // CANCEL
    Cancel,
    // TEAM_SELECT
    TeamSelect,
    // TEAM_COLOUR
    TeamColour,
    // REMAP_FORWARD
    RemapForward,
    // REMAP_BACKWARD
    RemapBackward,
    // REMAP_STRAFE_LEFT
    RemapStrafeLeft,
    // REMAP_STRAFE_RIGHT
    RemapStrafeRight,
    // REMAP_LEFT
    RemapLeft,
    // REMAP_RIGHT
    RemapRight,
    // REMAP_LOOK_UP
    RemapLookUp,
    // REMAP_LOOK_DOWN
    RemapLookDown,
    // REMAP_NEXT_WEAPON
    RemapNextWeapon,
    // REMAP_PREVIOUS_WEAPON
    RemapPreviousWeapon,
    // REMAP_JUMP
    RemapJump,
    // REMAP_FIRE
    RemapFire,
    // REMAP_GHOST_VISION
    RemapGhostVision,
    // REMAP_RELOAD
    RemapReload,
    // CONTROL_SENSITIVITY
    ControlSensitivity,
    // CONTROL_INVERT
    ControlInvert,
    // CONTROL_CONFIGURATION
    ControlConfiguration,
    // CONTROL_USER_DEFINED
    ControlUserDefined,
    // CONTROL_TYPEA
    ControlTypeA,
    // CONTROL_TYPEB
    ControlTypeB,
    // CONTROL_TYPEC
    ControlTypeC,
    // KEY_DPAD_LEFT
    KeyDpadLeft,
    // KEY_DPAD_RIGHT
    KeyDpadRight,
    // KEY_DPAD_UP
    KeyDpadUp,
    // KEY_DPAD_DOWN
    KeyDpadDown,
    // KEY_DPAD_BUTTON
    KeyDpadButton,
    // KEY_1
    Key1,
    // KEY_2
    Key2,
    // KEY_3
    Key3,
    // KEY_4
    Key4,
    // KEY_5
    Key5,
    // KEY_6
    Key6,
    // KEY_7
    Key7,
    // KEY_8
    Key8,
    // KEY_9
    Key9,
    // KEY_STAR
    KeyStar,
    // KEY_HASH
    KeyHash,
    // KEY_EMPTY
    KeyEmpty,
    // KEY_WAITING
    KeyWaiting,
    // REMAP_INVALID1
    RemapInvalid1,
    // REMAP_INVALID2
    RemapInvalid2,
    // POPUP_MISSION_UPDATED
    PopupMissionUpdated,
    // POPUP_VANESSA_DIED
    PopupVanessaDied,
    // JOURNAL_INTRO1
    JournalIntro1,
    // JOURNAL_INTRO2
    JournalIntro2,
    // JOURNAL_INTRO3
    JournalIntro3,
    // JOURNAL_INTRO4
    JournalIntro4,
    // JOURNAL_INTRO5
    JournalIntro5,
    // JOURNAL_INTRO6
    JournalIntro6,
    // JOURNAL_INTRO7
    JournalIntro7,
    // JOURNAL_INTRO8
    JournalIntro8,
    // JOURNAL_INTRO9
    JournalIntro9,
    // JOURNAL_INTRO10
    JournalIntro10,
    // JOURNAL_INTRO11
    JournalIntro11,
    // JOURNAL_INTRO12
    JournalIntro12,
    // JOURNAL_1A_F
    Journal1aF,
    // JOURNAL_1B_D
    Journal1bD,
    // JOURNAL_1C_D2
    Journal1cD2,
    // JOURNAL_1C_H
    Journal1cH,
    // JOURNAL_1D_F
    Journal1dF,
    // JOURNAL_1E_H
    Journal1eH,
    // JOURNAL_1F_H
    Journal1fH,
    // OBJECTIVE_1A
    Objective1a,
    // OBJECTIVE_1B
    Objective1b,
    // OBJECTIVE_1C
    Objective1c,
    // JOURNAL_2A_F
    Journal2aF,
    // JOURNAL_2B_H
    Journal2bH,
    // OBJECTIVE_2A
    Objective2a,
    // OBJECTIVE_2B
    Objective2b,
    // OBJECTIVE_2C
    Objective2c,
    // JOURNAL_3A_F
    Journal3aF,
    // JOURNAL_3B_D
    Journal3bD,
    // JOURNAL_3B_D2
    Journal3bD2,
    // JOURNAL_3C_D
    Journal3cD,
    // JOURNAL_3D_D
    Journal3dD,
    // JOURNAL_3D_D2
    Journal3dD2,
    // OBJECTIVE_3A
    Objective3a,
    // OBJECTIVE_3B
    Objective3b,
    // JOURNAL_4A_D
    Journal4aD,
    // JOURNAL_4B_D
    Journal4bD,
    // JOURNAL_4C_D
    Journal4cD,
    // OBJECTIVE_4A
    Objective4a,
    // OBJECTIVE_4B
    Objective4b,
    // OBJECTIVE_4C
    Objective4c,
    // JOURNAL_5A_D
    Journal5aD,
    // JOURNAL_5B_D
    Journal5bD,
    // JOURNAL_5C_D
    Journal5cD,
    // OBJECTIVE_5A
    Objective5a,
    // OBJECTIVE_5B
    Objective5b,
    // OBJECTIVE_5C
    Objective5c,
    // OBJECTIVE_5D
    Objective5d,
    // JOURNAL_6A_D
    Journal6aD,
    // JOURNAL_6B_D
    Journal6bD,
    // JOURNAL_6C_D
    Journal6cD,
    // JOURNAL_6D_D
    Journal6dD,
    // JOURNAL_6E_D
    Journal6eD,
    // OBJECTIVE_6A
    Objective6a,
    // OBJECTIVE_6B
    Objective6b,
    // OBJECTIVE_6C
    Objective6c,
    // JOURNAL_7A_D
    Journal7aD,
    // JOURNAL_7B_D
    Journal7bD,
    // JOURNAL_7C_D
    Journal7cD,
    // JOURNAL_7D_D
    Journal7dD,
    // OBJECTIVE_7A
    Objective7a,
    // OBJECTIVE_7B
    Objective7b,
    // JOURNAL_8A_D
    Journal8aD,
    // JOURNAL_8B_D
    Journal8bD,
    // JOURNAL_8C_D
    Journal8cD,
    // JOURNAL_8D_H
    Journal8dH,
    // JOURNAL_8E_D
    Journal8eD,
    // JOURNAL_8F_D
    Journal8fD,
    // JOURNAL_8G_D
    Journal8gD,
    // OBJECTIVE_8A
    Objective8a,
    // OBJECTIVE_8B
    Objective8b,
    // OBJECTIVE_8C
    Objective8c,
    // ENDING_CUTSCENE1
    EndingCutscene1,
    // ENDING_CUTSCENE2
    EndingCutscene2,
    // JOURNAL_ENDING_CUTSCENE3
    JournalEndingCutscene3,
    // JOURNAL_ENDING_CUTSCENE4
    JournalEndingCutscene4,
    // JOURNAL_ENDING_CUTSCENE5
    JournalEndingCutscene5,
    // JOURNAL_ENDING_CUTSCENE6
    JournalEndingCutscene6,
    // ACQUIRED
    Acquired,
    // AMMO
    Ammo,
    // PISTOL
    Pistol,
    // PISTOL_AMMO
    PistolAmmo,
    // SHOTGUN
    Shotgun,
    // SHOTGUN_AMMO
    ShotgunAmmo,
    // DUAL_PISTOL
    DualPistol,
    // DUAL_PISTOL_AMMO
    DualPistolAmmo,
    // MACHINEGUN
    MachineGun,
    // MACHINEGUN_AMMO
    MachineGunAmmo,
    // GATLINGGUN
    GatlingGun,
    // GATLINGGUN_AMMO
    GatlingGunAmmo,
    // ALIENGUN
    AlienGun,
    // SNIPERRIFLE
    SniperRifle,
    // SNIPERRIFLE_AMMO
    SniperRifleAmmo,
    // GRENADE_LAUNCHER
    GrenadeLauncher,
    // ROCKET_AMMO
    RocketAmmo,
    // GRENADE_AMMO
    GrenadeAmmo,
    // GHOSTVISION_GOGGLES
    GhostVisionGoggles,
    // MISSION_FAILED
    MissionFailed,
    // RESTARTING
    Restarting,
    // MAXIMUM
    Maximum,
    // AMMO_FULL
    AmmoFull,
    // DHOLDER_TEXT1
    Select,
    // DHOLDER_TEXT2
    Back,
    // DHOLDER_TEXT3
    MainMenu,
    // DHOLDER_TEXT4
    StartMenu,
    // DHOLDER_TEXT5
    MissionSelect,
    // DHOLDER_TEXT6
    Difficulty,
    // DHOLDER_TEXT7
    PauseGame,
    // DHOLDER_TEXT8
    SaveGame,
    // DHOLDER_TEXT9
    Objectives,
    // DHOLDER_TEXT10
    GameOver,
    // DHOLDER_TEXT11
    LevelComplete,
    // DHOLDER_TEXT12
    ControlsStyleA,
    // DHOLDER_TEXT13
    ControlsStyleB,
    // DHOLDER_TEXT14
    ControlsStyleC,
    // DHOLDER_TEXT15
    ControlsUserDefined,
    // DHOLDER_TEXT16
    LevelHighScore,
    // DHOLDER_TEXT17
    Statistics,
    // DHOLDER_TEXT18
    CheatMenu,
    // DHOLDER_TEXT19
    Loading,
    // DHOLDER_TEXT20
    AudioVideo,
    // DHOLDER_TEXT21
    ControllerType,
    // DHOLDER_TEXT22
    GameHighScore,
    // LEVEL_NAME1
    LevelName1,
    // LEVEL_NAME2
    LevelName2,
    // LEVEL_NAME3
    LevelName3,
    // LEVEL_NAME4
    LevelName4,
    // LEVEL_NAME5
    LevelName5,
    // LEVEL_NAME6
    LevelName6,
    // LEVEL_NAME7
    LevelName7,
    // LEVEL_NAME8
    LevelName8,
    // PLAYER_DEAD
    PlayerDead,
    // SCORE_TALLY
    ScoreTally,
    // MISSION_TIME_PLAYER
    MissionTimePlayer,
    // MISSION_TIME_PAR
    MissionTimePar,
    // FIRING_ACCURACY
    FiringAccuracy,
    // TOTAL_SCORE
    TotalScore,
    // HIGH_SCORER
    HighScorer,
    // ENTER_INITIALS
    EnterInitials,
    // FLAK_JACKET
    FlakJacket,
    // HISCORE_SPACE
    HiscoreSpace,
    // HISCORE_DELETE
    HiscoreDelete,
    // HISCORE_DONE
    HiscoreDone,
    // HISCORE_CHAR_A
    HiscoreCharA,
    // HISCORE_CHAR_B
    HiscoreCharB,
    // HISCORE_CHAR_C
    HiscoreCharC,
    // HISCORE_CHAR_D
    HiscoreCharD,
    // HISCORE_CHAR_E
    HiscoreCharE,
    // HISCORE_CHAR_F
    HiscoreCharF,
    // HISCORE_CHAR_G
    HiscoreCharG,
    // HISCORE_CHAR_H
    HiscoreCharH,
    // HISCORE_CHAR_I
    HiscoreCharI,
    // HISCORE_CHAR_J
    HiscoreCharJ,
    // HISCORE_CHAR_K
    HiscoreCharK,
    // HISCORE_CHAR_L
    HiscoreCharL,
    // HISCORE_CHAR_M
    HiscoreCharM,
    // HISCORE_CHAR_N
    HiscoreCharN,
    // HISCORE_CHAR_O
    HiscoreCharO,
    // HISCORE_CHAR_P
    HiscoreCharP,
    // HISCORE_CHAR_Q
    HiscoreCharQ,
    // HISCORE_CHAR_R
    HiscoreCharR,
    // HISCORE_CHAR_S
    HiscoreCharS,
    // HISCORE_CHAR_T
    HiscoreCharT,
    // HISCORE_CHAR_U
    HiscoreCharU,
    // HISCORE_CHAR_V
    HiscoreCharV,
    // HISCORE_CHAR_W
    HiscoreCharW,
    // HISCORE_CHAR_X
    HiscoreCharX,
    // HISCORE_CHAR_Y
    HiscoreCharY,
    // HISCORE_CHAR_Z
    HiscoreCharZ,
    // SERVER_SETUP
    ServerSetup,
    // GAME_TYPE
    GameType,
    // GAME_MODE_DEATHMATCH
    GameModeDeathmatch,
    // GAME_MODE_DEATHMATCHTEAM
    GameModeTeamDeathmatch,
    // GAME_MODE_SURVIVOR
    GameModeSurvivor,
    // GAME_MODE_ESCAPE
    GameModeEscape,
    // MULTIPLAYER_MAP
    MultiplayerMap,
    // SET_LIMIT
    SetLimit,
    // LIMIT_UNLIMITED
    LimitUnlimited,
    // LIMIT_5MINS
    Limit5mins,
    // LIMIT_10MINS
    Limit10mins,
    // LIMIT_20MINS
    Limit20mins,
    // LIMIT_10KILLS
    Limit10kills,
    // LIMIT_20KILLS
    Limit20kills,
    // LIMIT_30KILLS
    Limit30kills,
    // WAITING_FOR_PLAYERS
    WaitingForPlayers,
    // QUIT_GAME
    QuitGame,
    // OVERWRITE_SAVE_GAME
    OverwriteSaveGame,
    // RESET_HIGH_SCORES
    ResetHighScores,
    // EPILOGUE
    Epilogue,
    // OBJ_INCOMPLETE
    ObjectiveIncomplete,
    // PULSE_GUN
    PulseGun,
    // CHAPTER_COMPLETE
    ChapterComplete,
    // PLAYER
    Player,
    // HAS_DIED
    HasDied,
    // HAS_KILLED
    HasKilled,
    // GIBBED
    Gibbed,
    // NO_KILLS
    NoKills,
    // NO_FRAGS
    NoFrags,
    // NO_DEATHS
    NoDeaths,
    // CONNECT_GAMESERVICES
    ConnectGameservices,
    // RED
    Red,
    // GREEN
    Green,
    // BLUE
    Blue,
    // YELLOW
    Yellow,
    // OUTRO_1A
    Outro1a,
    // OUTRO_1B
    Outro1b,
    // OUTRO_1C
    Outro1c,
    // OUTRO_1D
    Outro1d,
    // CREDITS_LIST
    CreditsList,
    // DEMO_MODE
    DemoMode,
    // LOGIN
    Login,
    // USER_NAME
    UserName,
    // PASSWORD
    Password,
    // REMEMBER_PASSWORD
    RememberPassword,
    // RETRY
    Retry,
    // LOCAL_SCORES
    LocalScores,
    // WORLD_SCORES
    WorldScores,
    // SUBMIT_SCORES
    SubmitScores,
    // UPLOADING
    Uploading,
    // NEW_ACCOUNT
    NewAccount,
    // NEW_ACCOUNT_PROMPT
    NewAccountPrompt,
    // FIRST_NAME
    FirstName,
    // LAST_NAME
    LastName,
    // COUNTRY
    Country,
    // EMAIL
    Email,
    // BIRTHDATE
    Birthdate,
    // PASSWORD_QUESTION
    PasswordQuestion,
    // PASSWORD_ANSWER
    PasswordAnswer,
    // TRYAGAIN
    TryAgain,
    // NGAGE_ARENA
    NgageArena,
    // GAME_SERVICES
    GameServices,
    // GAME_SERVICES_PROMPT
    GameServicesPrompt,
    // CONNECTING
    Connecting,
    // REGISTERING_NEW_ACCOUNT
    RegisteringNewAccount,
    // REGISTRATION_FAILED
    RegistrationFailed,
    // REGISTRATION_FAILED_GENERIC
    RegistrationFailedGeneric,
    // REGISTRATION_SUCCESSFUL
    RegistrationSuccessful,
    // LOGIN_FAILED
    LoginFailed,
    // LOGIN_SUCCESSFUL
    LoginSuccessful,
    // SEND_COMPLETE
    SendComplete,
    // DOWNLOADING
    Downloading,
    // OPTIONS_LANGUAGE
    OptionsLanguage,
    // RESTART
    Restart,
    // RESTART_CONFIRM
    RestartConfirm,
    // CONTROLS_MOVE_FORWARD
    ControlsMoveForward,
    // CONTROLS_TURN_LEFT
    ControlsTurnLeft,
    // CONTROLS_TURN_RIGHT
    ControlsTurnRight,
    // CONTROLS_MOVE_BACKWARD
    ControlsMoveBackward,
    // CONTROLS_GHOSTVISION
    ControlsGhostvision,
    // CONTROLS_LOOK_DOWN
    ControlsLookDown,
    // CONTROLS_LOOK_UP
    ControlsLookUp,
    // CONTROLS_STRAFE_LEFT
    ControlsStrafeLeft,
    // CONTROLS_FIRE
    ControlsFire,
    // CONTROLS_STRAFE_RIGHT
    ControlsStrafeRight,
    // CONTROLS_JUMP
    ControlsJump,
    // CONTROLS_NEXT_WEAPON
    ControlsNextWeapon,
    // CONTROLS_PREV_WEAPON
    ControlsPrevWeapon,
    // CONTROLS_OBJECTIVES
    ControlsObjectives,
    // CONTROLS_RELOAD
    ControlsReload,
    // CONTROLS_TURN_STRAFE_LEFT
    ControlsTurnStrafeLeft,
    // CONTROLS_TURN_STRAFE_RIGHT
    ControlsTurnStrafeRight,
    // CONTROLS_HOLD_STRAFE
    ControlsHoldStrafe,
    // OPTIONS_BLUETOOTH
    OptionsBluetooth,
    // BLUETOOTH_ENABLED
    BluetoothEnabled,
    // BLUETOOTH_NAME
    BluetoothName,
    // CHAPTER_HIGH_SCORE
    ChapterHighScore,
    // CHAPTER_SELECT
    ChapterSelect,
    // DATE_OF_BIRTH
    DateOfBirth,
    // DATE_DD
    DateDay,
    // DATE_MM
    DateMonth,
    // DATE_YYYY
    DateYear,
    // MISSING_FIELD
    MissingField,
    // PRESS_ANY_KEY
    PressAnyKey,
    // DELETE_ALL_DATA
    DeleteAllData,
    // DELETE_ALL_DATA_CONFIRM
    DeleteAllDataConfirm,
    // DELETING
    Deleting,
    // DELETE_SAVED_GAMES
    DeleteSavedGames,
    // DELETE_SAVED_GAMES_CONFIRM
    DeleteSavedGamesConfirm,
    // DELETE_SAVED_GAME_CONFIRM
    DeleteSavedGameConfirm,
    // DELETE_SAVED_OPTIONS
    DeleteSavedOptions,
    // DELETE_SAVED_OPTIONS_CONFIRM
    DeleteSavedOptionsConfirm,
    // MEMORY_FULL
    MemoryFull,
    // SAVE_FAILED
    SaveFailed,
    // SAVE_OVERWRITE
    SaveOverwrite,
    // DATA_CORRUPTED
    DataCorrupted,
    // OUT_OF_MEMORY
    OutOfMemory,
    // MESSAGE_RECEIVED
    MessageReceived,
    // MULTIPLAYER_BLUETOOTH_CONNECTION_BUSY
    MultiplayerBluetoothConnectionBusy,
    // MULTIPLAYER_JOIN_REJECTED
    MultiplayerJoinRejected,
    // MULTIPLAYER_JOIN_REJECTED_TOO_MANY
    MultiplayerJoinRejectedTooMany,
    // MULTIPLAYER_HAS_QUIT
    MultiplayerHasQuit,
    // MULTIPLAYER_HAS_PAUSED
    MultiplayerHasPaused,
    // MULTIPLAYER_GAME_TERMINATED_BY_HOST
    MultiplayerGameTerminatedByHost,
    // MULTIPLAYER_GAME_CANCELLED
    MultiplayerGameCancelled,
    // MULTIPLAYER_CONNECTION_LOST
    MultiplayerConnectionLost,
    // MULTIPLAYER_ANY_CONNECTION_LOST
    MultiplayerAnyConnectionLost,
    // MULTIPLAYER_CONNECTION
    MultiplayerConnection,
    // MULTIPLAYER_ACCEPT
    MultiplayerAccept,
    // MULTIPLAYER_REJECT
    MultiplayerReject,
    // ARENA_LEGAL_SCREEN
    ArenaLegalScreen,
    // ARENA_INVALID_USERNAME_PASSWORD
    ArenaInvalidUsernamePassword,
    // STRINGID_COUNT
    StringIdCount
}

pub struct StringTable {

}

impl Asset for StringTable {
    fn extension() -> &'static str {
        "json"
    }
}

impl Into<Vec<u8>> for StringTable {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

