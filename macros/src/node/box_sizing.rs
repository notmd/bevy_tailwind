use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_box_sizing(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "box-border" => {
            ctx.node_props.insert(NodeProp::BoxSizing, quote! {
                bevy::ui::BoxSizing::BorderBox
            });
            Ok(true)
        }
        "box-content" => {
            ctx.node_props.insert(NodeProp::BoxSizing, quote! {
                bevy::ui::BoxSizing::ContentBox
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_box_sizing() {
        let mut ctx = ParseCtx::default();
        assert_eq!(parse_box_sizing(&mut ctx, "box-border"), Ok(true));

        assert_eq!(
            ctx.node_props
                .get(&NodeProp::BoxSizing)
                .unwrap()
                .to_string(),
            "bevy :: ui :: BoxSizinng :: BorderBox"
        );

        let mut ctx = ParseCtx::default();
        assert_eq!(parse_box_sizing(&mut ctx, "box-content"), Ok(true));

        assert_eq!(
            ctx.node_props
                .get(&NodeProp::BoxSizing)
                .unwrap()
                .to_string(),
            "bevy :: ui :: BoxSizinng :: ContentBox"
        );
    }
}
