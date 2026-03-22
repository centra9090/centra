pub mod buffer;
pub mod csm;
pub mod csm_v2;
pub mod bitpack;
pub mod decode;

pub enum Symbol<'a> {
    Word(&'a str),
}