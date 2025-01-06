use bevy_tailwind_macro::tw;

#[allow(dead_code)]
fn test() {
    // aspect-ratio

    // box-sizing
    // tw!("box-border");
    // tw!("box-content");

    // display
    tw!("flex");
    tw!("grid");
    tw!("block");
    tw!("hidden");

    // overflow

    // position
    tw!("relative");
    tw!("absolute");
    // top / right / bottom / left
    tw!("top-0");
    tw!("right-0");
    tw!("bottom-0");
    tw!("left-0");
    // tw!("inset-x-0");
    // tw!("inset-y-0");

    // z-index
    tw!("z-10");
    tw!("-z-10");

    // flex basis
    tw!("basis-0");
    tw!("basis-1/2");

    // flex direction
    tw!("flex-row");
    tw!("flex-row-reverse");
    tw!("flex-col");
    tw!("flex-col-reverse");

    // flex wrap
    tw!("flex-wrap");
    tw!("flex-wrap-reverse");
    tw!("flex-nowrap");

    // flex
    tw!("flex-1");
    tw!("flex-auto");
    tw!("flex-initial");
    tw!("flex-none");

    // flex grow
    tw!("grow");
    tw!("flex-grow");
    tw!("grow-0");
    tw!("flex-grow-0");

    // flex shrink
    tw!("shrink");
    tw!("flex-shrink");
    tw!("shrink-0");
    tw!("flex-shrink-0");
    // order

    // grid template columns
    tw!("grid-cols-1");
    tw!("grid-cols-none");

    // grid template columns start/end

    // grid template rows

    // grid template rows start/end

    // grid auto flow

    // grid auto columns

    // grid auto rows

    // gap

    // justify content

    // justify items

    // justify self

    // align content

    // align items

    // align self

    // place content

    // place items

    // place self

    // padding
    tw!("pt-1 pr-px pb-1 pl-px");
    // margin

    // width

    // min width

    // max width

    // height

    // min height

    // max height

    // size

    // font size

    // font weight

    //
}
