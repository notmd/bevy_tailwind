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
    tw!("col-auto");
    tw!("col-span-1");
    tw!("col-span-full");
    tw!("col-start-1");
    tw!("col-start-auto");
    tw!("col-end-1");
    tw!("col-end-auto");

    // grid template rows
    tw!("grid-rows-1");
    tw!("grid-rows-none");

    // grid template rows start/end
    tw!("row-auto");
    tw!("row-span-1");
    tw!("row-span-full");
    tw!("row-start-1");
    tw!("row-start-auto");
    tw!("row-end-1");
    tw!("row-end-auto");

    // grid auto flow
    tw!("grid-flow-row");
    tw!("grid-flow-col");
    tw!("grid-flow-row-dense");
    tw!("grid-flow-col-dense");

    // grid auto columns
    tw!("auto-cols-auto");
    tw!("auto-cols-min");
    tw!("auto-cols-max");
    tw!("auto-cols-fr");

    // grid auto rows
    tw!("auto-rows-auto");
    tw!("auto-rows-min");
    tw!("auto-rows-max");
    tw!("auto-rows-fr");

    // gap
    tw!("gap-0");
    tw!("gap-x-0");
    tw!("gap-y-0");
    tw!("gap-px");
    tw!("gap-x-px");
    tw!("gap-y-px");
    tw!("gap-1.5");
    tw!("gap-x-1.5");
    tw!("gap-y-1.5");

    // justify content
    tw!("justify-normal");
    tw!("justify-start");
    tw!("justify-end");
    tw!("justify-center");
    tw!("justify-between");
    tw!("justify-around");
    tw!("justify-evenly");
    tw!("justify-stretch");

    // justify items
    tw!("justify-items-start");
    tw!("justify-items-end");
    tw!("justify-items-center");
    tw!("justify-items-stretch");

    // justify self
    tw!("justify-self-auto");
    tw!("justify-self-start");
    tw!("justify-self-end");
    tw!("justify-self-center");

    // align content
    tw!("content-normal");
    tw!("content-center");
    tw!("content-start");
    tw!("content-end");
    tw!("content-between");
    tw!("content-around");
    tw!("content-evenly");
    tw!("content-stretch");

    // align items
    tw!("items-start");
    tw!("items-end");
    tw!("items-center");
    tw!("items-baseline");
    tw!("items-stretch");

    // align self
    tw!("self-auto");
    tw!("self-start");
    tw!("self-end");
    tw!("self-center");
    tw!("self-stretch");
    tw!("self-baseline");

    // place content
    tw!("place-content-center");
    tw!("place-content-start");
    tw!("place-content-end");
    tw!("place-content-between");
    tw!("place-content-around");
    tw!("place-content-evenly");
    tw!("place-content-baseline");
    tw!("place-content-stretch");

    // place items
    tw!("place-items-start");
    tw!("place-items-end");
    tw!("place-items-center");
    tw!("place-items-baseline");
    tw!("place-items-stretch");

    // place self
    tw!("place-self-auto");
    tw!("place-self-start");
    tw!("place-self-end");
    tw!("place-self-center");
    tw!("place-self-stretch");

    // padding
    tw!("p-0");
    tw!("px-0");
    tw!("py-0");
    tw!("pt-0");
    tw!("pr-0");
    tw!("pb-0");
    tw!("pl-0");
    tw!("p-px");
    tw!("px-px");
    tw!("py-px");
    tw!("pt-px");
    tw!("pr-px");
    tw!("pb-px");
    tw!("pl-px");
    tw!("p-1.5");
    tw!("px-1.5");
    tw!("py-1.5");
    tw!("pt-1.5");
    tw!("pr-1.5");
    tw!("pb-1.5");
    tw!("pl-1.5");

    // margin
    tw!("m-0");
    tw!("mx-0");
    tw!("my-0");
    tw!("mt-0");
    tw!("mr-0");
    tw!("mb-0");
    tw!("ml-0");
    tw!("m-px");
    tw!("mx-px");
    tw!("my-px");
    tw!("mt-px");
    tw!("mr-px");
    tw!("mb-px");
    tw!("ml-px");
    tw!("m-1.5");
    tw!("mx-1.5");
    tw!("my-1.5");
    tw!("mt-1.5");
    tw!("mr-1.5");
    tw!("mb-1.5");
    tw!("ml-1.5");

    // width
    tw!("w-0");
    tw!("w-0.5");
    tw!("w-px");
    tw!("w-1/2");
    tw!("w-full");
    tw!("w-screen");
    tw!("w-svw");
    tw!("w-lvw");

    // min width
    tw!("min-w-0");
    tw!("min-w-0.5");
    tw!("min-w-px");
    tw!("min-w-1/2");
    tw!("min-w-full");
    tw!("min-w-screen");

    // max width
    tw!("max-w-0");
    tw!("max-w-0.5");
    tw!("max-w-px");
    tw!("max-w-1/2");
    tw!("max-w-full");
    tw!("max-w-screen");

    // height

    // min height

    // max height

    // size

    // font size

    // font weight

    //
}
