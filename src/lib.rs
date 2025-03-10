use solana_program::entrypoint;

pub mod errors;
pub mod handlers;
pub mod instruction;
pub mod processor;
pub mod state;

use processor::process_instruction;

entrypoint!(process_instruction);

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_adapp_reviews() {}
}
