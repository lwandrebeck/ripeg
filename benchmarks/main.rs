macro_rules! mods {
    ($(mod $mod:ident;)+) => {
        $(mod $mod;)+
        ::criterion::criterion_main!($($mod::benches),+);
    }
}

mods![
    mod charset;
    mod isa;
];
