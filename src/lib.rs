cfg_if::cfg_if! {
    if #[cfg(feature = "legacy")]{
        pub use solana_program::program_error::PrintProgramError;
        pub use solana_program::decode_error::DecodeError;
    } else {
        // TODO: Redefine types so the crate can be minimal dependency.
    }
}
