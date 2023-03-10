macro_rules! keywords {
    ($(
        $ident : ident $(= $string_keyword : expr)?
    ), *
    ) => {
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum KeyWord{
            NoKeyWord,
            $($ident), *
        }

        pub const ALL_KEYWORDS_INDEX : &[KeyWord] = &[
            $(KeyWord::$ident), *
        ];

        // Define all keywords as global variables, 
        // assign the specified value to the specified value, 
        // and use the built-in macro stringify for conversion 
        // if there is no specified value.
        $(kw_def!($ident $(= $string_keyword)?);)*
        pub const ALL_KEYWORDS : &[&str] = &[
            $($ident), *
        ];
    };
}

/// use macro define enum's const.
/// it include two module.
macro_rules! kw_def {
    ($ident : ident = $string_keyword : expr
    ) => {
        pub const $ident : &'static str = $string_keyword;
    };
    ($ident : ident) => {
        kw_def!($ident = stringify!($ident));
    }

}

// sorted for binary search.
keywords!(
    CREATE,
    SELECT
);

pub const RESERVED_FOR_TABLE_ALIAS : &[KeyWord] = &[
    KeyWord::SELECT,
];

pub const RESERVED_FOR_COLUMN_ALIAS : &[KeyWord] = &[
    KeyWord::SELECT,
];