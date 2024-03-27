use crate::__steps__::Context;
use crate::__steps__::Project;

pub fn given_there_is_an_empty_assembly(context: &mut Context) {
    let path = Project::new("empty_assembly")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .build();

    context.assembly_set(path);
}
