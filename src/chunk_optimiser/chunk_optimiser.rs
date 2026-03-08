use tiktoken_rs::{CoreBPE, get_bpe_from_model, get_bpe_from_tokenizer, model};
use crate::chunk_optimiser::cheapest_path_indices;

#[derive(Clone, Copy)]
pub enum Granularity {
    Characters,
    Tokens,
}

pub struct ChunkOptimiser<'a> {
    text: &'a str,  // needed incase we tokenise
    punishments: Vec<usize>,
    token_start_char_idx: Vec<usize>  // maps characters in tokens to the original indices
}

impl<'a> ChunkOptimiser<'a> {
    /// create the token to text mapping
    pub fn new(text: &'a str, punishments: Vec<usize>, model: &str) -> Self {
        let token_start_char_idx = Self::build_token_alignment(text, model);

        Self {
            text,
            punishments,
            token_start_char_idx,
        }
    }

    /// here we either run the optimiser on text, or we map the punishments down to a reduced
    /// token vector, and run against that before mapping back up.
    /// my hope is this is the grooviest option - but idk tbh
    /// do we need to manually support many tokenisers???
    pub fn optimise_chunks(
        &self,
        max_chunk_size: usize,
        granularity: Granularity,
    ) -> Vec<usize> {
        match granularity {
            Granularity::Characters => {
                cheapest_path_indices(&self.punishments, max_chunk_size)
            }
            Granularity::Tokens => {
                let reduced = self.build_reduced_vector();
                let token_path = cheapest_path_indices(&reduced, max_chunk_size);

                // map the reduced indices back to their original positions
                token_path
                    .into_iter()
                    .map(|tok_idx| self.token_start_char_idx[tok_idx])
                    .collect()

            }
        }
    }

    /// build a mapping from the last element of a token to its index in
    /// the original string. 
    fn build_token_alignment(text: &str, model: &str) -> Vec<usize> {
        let bpe = Self::get_bpe(model);
        let tokens = bpe.encode_ordinary(text);

        let mut start_indices = Vec::with_capacity(tokens.len());
        let mut cursor = 0usize;

        for tok in tokens {
            start_indices.push(cursor);  // Store START position
            let s = bpe.decode(vec![tok]).unwrap();
            let len = s.chars().count();
            cursor += len;
        }

        start_indices
    }

    /// generic tokeniser interface
    fn get_bpe(model: &str) -> CoreBPE {
        get_bpe_from_model(model)
            .expect("invalid model or tokenizer selected")
    }

    /// reduce a vector to only the end characters of the tokens in it
    fn build_reduced_vector(&self) -> Vec<usize> {
        self.token_start_char_idx
            .iter()
            .map(|&i| self.punishments[i])
            .collect()
    }

    /// given a character index corresponding to the start of a token boundary,
    /// return the character index that is `offset_tokens` tokens further along.
    pub fn char_index_after_token_offset(&self, start_char_idx: usize, offset_tokens: usize) -> usize {
        let token_idx = match self.token_start_char_idx.binary_search(&start_char_idx) {
            Ok(i) => i,
            Err(i) => i, // first token whose start is > start_char_idx
        };

        let target = token_idx.saturating_add(offset_tokens);
        if target >= self.token_start_char_idx.len() {
            self.text.len()
        } else {
            self.token_start_char_idx[target]
        }
    }
}
