use crate::__steps__::assembly::given_there_is_an_assembly_with;
use crate::__steps__::Context;

pub fn given_there_is_an_assembly_without_anything(context: &mut Context) {
    given_there_is_an_assembly_with(context, "");
}
