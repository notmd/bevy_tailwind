#![allow(dead_code)]
#![allow(unused_variables)]
use bevy::prelude::*;
use bevy_tailwind::tw;

fn simple(mut query: Query<EntityMut>) {
    let mut entity = query.single_mut().unwrap();
    tw!("flex");
    tw!(@ &mut entity, "flex");
    tw!("block",{
        "flex": true
    });
    tw!(@ &mut entity, "block",{
        "flex": true
    });
    tw!(@ &mut entity, {
        "flex": true
    });
    tw!(@ &mut entity, {
        "flex": true,
        "block": true
    });
}

fn nested_with_priority(mut query: Query<EntityMut>) {
    let mut entity = query.single_mut().unwrap();
    let prio_1lv = tw!("pt-0 pr-1 pb-2 pl-3");
    let prio_1lv_cod = tw!({
        "pt-0": true,
        "pl-1": true,
        "pr-2": true,
        "pb-3": true
    });
    let prio_2lv = tw!("px-1 pl-2");
    let prio_2lv_cond = tw!({
        "px-1": true,
        "pr-2": true
    });
    let prio_2lv_cond_complex = tw!("px-1 pl-2 pl-3", {
        "pr-4 pl-5 pl-6": true,
        "pl-7": false,
        "px-8": false
    });
    tw!(@ &mut entity, "pt-1");
    tw!(@ &mut entity, "pl-1", {
        "pl-2": true
    });
    tw!(@ &mut entity, "p-1", {
        "pl-2": true,
        "px-3 pl-4": true
    });
    tw!(@ &mut entity, "p-1", {
        "pl-2": true,
        "px-3 pl-4": true,
        "p-5": true
    });
}

fn mutate_component() {
    let node = Node::default();

    tw!(node, "w-full");
}

fn picking_style(mut query: Query<EntityMut>) {
    let entity = query.single_mut().unwrap();
    tw!("bg-red-500 hover:bg-blue-500 focus:bg-green-500", {
        "hover:bg-yellow-500 hover:pt-1": true
    });
    // tw!(&mut entity, "bg-transparent hover:bg-black focus:bg-white", {
    //     "bg-red-500": true,
    //     "hover:bg-blue-500": true,
    //     "focus:bg-green-500": true
    // });
}

fn test_all() {
    // aspect-ratio
    tw!("aspect-auto hover:aspect-square focus:aspect-video");
    tw!("aspect-square");
    tw!("aspect-video");
    tw!("aspect-3/2");
    tw!("aspect": Some(0.5));
    // box-sizing
    // tw!("box-border");
    // tw!("box-content");

    // display
    tw!("flex hover:grid focus:block");
    tw!("grid");
    tw!("block");
    tw!("hidden");
    tw!("d": Display::Flex);

    // overflow
    tw!("overflow-hidden");
    tw!("overflow-clip");
    tw!("overflow-visible");
    tw!("overflow-scroll");
    tw!("overflow": OverflowAxis::Hidden);
    tw!("overflow-x-hidden hover:overflow-x-clip focus:overflow-x-visible");
    tw!("overflow-x-clip");
    tw!("overflow-x-visible");
    tw!("overflow-x-scroll");
    tw!("overflow-x": OverflowAxis::Hidden);
    tw!("overflow-y-hidden hover:overflow-y-clip focus:overflow-y-visible");
    tw!("overflow-y-clip");
    tw!("overflow-y-visible");
    tw!("overflow-y-scroll");
    tw!("overflow-y": OverflowAxis::Hidden);

    // position
    tw!("relative hover:absolute focus:absolute");
    tw!("absolute");
    tw!("position": PositionType::Relative);

    // top / right / bottom / left
    tw!("top-0 hover:top-1 focus:top-2");
    tw!("-top-1");
    tw!("top": Val::Px(10.));
    tw!("right-0 hover:right-1 focus:right-2");
    tw!("-right-1");
    tw!("right": Val::Px(10.));
    tw!("bottom-0 hover:bottom-1 focus:bottom-2");
    tw!("-bottom-1");
    tw!("bottom": Val::Px(10.));
    tw!("left-0 hover:left-1 focus:left-2");
    tw!("-left-1");
    tw!("left": Val::Px(10.));
    // tw!("inset-x-0");
    // tw!("inset-y-0");

    // z-index
    tw!("z-10 hover:z-20 focus:z-30");
    tw!("-z-10");
    tw!("z": 100);

    // flex basis
    tw!("basis-0 hover:basis-1 focus:basis-2");
    tw!("basis-1/2");
    tw!("basis": Val::Px(10.));

    // flex direction
    tw!("flex-row hover:flex-col focus:flex-col");
    tw!("flex-row-reverse");
    tw!("flex-col");
    tw!("flex-col-reverse");
    tw!("flex-dir": FlexDirection::Row);

    // flex wrap
    tw!("flex-wrap hover:flex-wrap-reverse focus:flex-nowrap");
    tw!("flex-wrap-reverse");
    tw!("flex-nowrap");
    tw!("flex-wrap": FlexWrap::Wrap);

    // flex
    tw!("flex-1");
    tw!("flex-auto");
    tw!("flex-initial");
    tw!("flex-none");

    // flex grow
    tw!("grow hover:flex-grow-0 focus:flex-grow");
    tw!("flex-grow");
    tw!("grow-0");
    tw!("flex-grow-0");
    tw!("flex-grow": 1.0);
    tw!("grow": 1.0);

    // flex shrink
    tw!("shrink hover:flex-shrink-0 focus:flex-shrink");
    tw!("flex-shrink");
    tw!("shrink-0");
    tw!("flex-shrink-0");
    tw!("flex-shrink": 1.0);
    tw!("shrink": 1.0);

    // order

    // grid template columns
    tw!("grid-cols-1 hover:grid-cols-2 focus:grid-cols-3");
    tw!("grid-cols-none");
    tw!("grid-cols": GridTrack::fr(1.0));

    // grid template columns start/end
    tw!("col-auto");
    tw!("col-span-1");
    tw!("col-span-full");
    tw!("col-start-1");
    tw!("col-start-auto");
    tw!("col-end-1");
    tw!("col-end-auto");

    // grid template rows
    tw!("grid-rows-1 hover:grid-rows-2 focus:grid-rows-3");
    tw!("grid-rows-none");
    tw!("grid-rows": GridTrack::fr(1.0));

    // grid template rows start/end
    tw!("row-auto");
    tw!("row-span-1");
    tw!("row-span-full");
    tw!("row-start-1");
    tw!("row-start-auto");
    tw!("row-end-1");
    tw!("row-end-auto");

    // grid auto flow
    tw!("grid-flow-row hover:grid-flow-col focus:grid-flow-row-dense");
    tw!("grid-flow-col");
    tw!("grid-flow-row-dense");
    tw!("grid-flow-col-dense");
    tw!("grid-flow": GridAutoFlow::Row);

    // grid auto columns
    tw!("auto-cols-auto hover:auto-cols-min focus:auto-cols-max");
    tw!("auto-cols-min");
    tw!("auto-cols-max");
    tw!("auto-cols-fr");
    tw!("auto-cols": GridTrack::fr(1.0));

    // grid auto rows
    tw!("auto-rows-auto hover:auto-rows-min focus:auto-rows-max");
    tw!("auto-rows-min");
    tw!("auto-rows-max");
    tw!("auto-rows-fr");
    tw!("auto-rows": GridTrack::fr(1.0));

    // gap
    tw!("gap-0");
    tw!("gap": Val::Px(10.));
    tw!("gap-x-0 hover:gap-x-1 focus:gap-x-2");
    tw!("gap-x": Val::Px(10.));
    tw!("gap-y-0 hover:gap-y-1 focus:gap-y-2");
    tw!("gap-y": Val::Px(10.));
    tw!("gap-px");
    tw!("gap-x-px");
    tw!("gap-y-px");
    tw!("gap-1.5");
    tw!("gap-x-1.5");
    tw!("gap-y-1.5");

    // justify content
    tw!("justify-normal hover:justify-start focus:justify-end");
    tw!("justify-start");
    tw!("justify-end");
    tw!("justify-center");
    tw!("justify-between");
    tw!("justify-around");
    tw!("justify-evenly");
    tw!("justify-stretch");
    tw!("justity": JustifyContent::Center);

    // justify items
    tw!("justify-items-start hover:justify-items-end focus:justify-items-center");
    tw!("justify-items-end");
    tw!("justify-items-center");
    tw!("justify-items-stretch");
    tw!("justify-items": JustifyItems::Center);

    // justify self
    tw!("justify-self-auto hover:justify-self-start focus:justify-self-end");
    tw!("justify-self-start");
    tw!("justify-self-end");
    tw!("justify-self-center");
    tw!("justify-self": JustifySelf::Center);

    // align content
    tw!("content-normal hover:content-center focus:content-start");
    tw!("content-center");
    tw!("content-start");
    tw!("content-end");
    tw!("content-between");
    tw!("content-around");
    tw!("content-evenly");
    tw!("content-stretch");
    tw!("content": AlignContent::Center);

    // align items
    tw!("items-start hover:items-end focus:items-center");
    tw!("items-end");
    tw!("items-center");
    tw!("items-baseline");
    tw!("items-stretch");
    tw!("items": AlignItems::Center);

    // align self
    tw!("self-auto hover:self-start focus:self-end");
    tw!("self-start");
    tw!("self-end");
    tw!("self-center");
    tw!("self-stretch");
    tw!("self-baseline");
    tw!("self": AlignSelf::Center);

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
    tw!("p": Val::Px(10.));
    tw!("px-0");
    tw!("px": Val::Px(10.));
    tw!("py-0");
    tw!("py": Val::Px(10.));
    tw!("pt-0 hover:pt-1 focus:pt-2");
    tw!("pt": Val::Px(10.), "hover:pt": Val::Px(20.), "focus:pt": Val::Px(30.));
    tw!("pr-0 hover:pr-1 focus:pr-2");
    tw!("pr": Val::Px(10.), "hover:pr": Val::Px(20.), "focus:pr": Val::Px(30.));
    tw!("pb-0 hover:pb-1 focus:pb-2");
    tw!("pb": Val::Px(10.), "hover:pb": Val::Px(20.), "focus:pb": Val::Px(30.));
    tw!("pl-0 hover:pl-1 focus:pl-2");
    tw!("pl": Val::Px(10.), "hover:pl": Val::Px(20.), "focus:pl": Val::Px(30.));
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
    tw!("m": Val::Px(10.));
    tw!("mx-0");
    tw!("mx": Val::Px(10.));
    tw!("my-0");
    tw!("my": Val::Px(10.));
    tw!("mt-0 hover:mt-1 focus:mt-2");
    tw!("mt": Val::Px(10.), "hover:mt": Val::Px(20.), "focus:mt": Val::Px(30.));
    tw!("mr-0 hover:mr-1 focus:mr-2");
    tw!("mr": Val::Px(10.), "hover:mr": Val::Px(20.), "focus:mr": Val::Px(30.));
    tw!("mb-0 hover:mb-1 focus:mb-2");
    tw!("mb": Val::Px(10.), "hover:mb": Val::Px(20.), "focus:mb": Val::Px(30.));
    tw!("ml-0 hover:ml-1 focus:ml-2");
    tw!("ml": Val::Px(10.), "hover:ml": Val::Px(20.), "focus:ml": Val::Px(30.));
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
    tw!("w-auto hover:w-0 focus:w-0.5");
    tw!("w-0");
    tw!("w-0.5");
    tw!("w-px");
    tw!("w-1/2");
    tw!("w-full");
    tw!("w-screen");
    tw!("w-svw");
    tw!("w-lvw");
    tw!("w": Val::Px(10.));

    // min width
    tw!("min-w-0 hover:min-w-0.5 focus:min-w-px");
    tw!("min-w-0.5");
    tw!("min-w-px");
    tw!("min-w-1/2");
    tw!("min-w-full");
    tw!("min-w": Val::Px(10.));

    // max width
    tw!("max-w-0 hover:max-w-0.5 focus:max-w-px");
    tw!("max-w-0.5");
    tw!("max-w-px");
    tw!("max-w-1/2");
    tw!("max-w-full");
    tw!("max-w": Val::Px(10.));

    // height
    tw!("h-auto hover:h-0 focus:h-0.5");
    tw!("h-0");
    tw!("h-0.5");
    tw!("h-px");
    tw!("h-1/2");
    tw!("h-full");
    tw!("h-screen");
    tw!("h-svh");
    tw!("h-lvh");
    tw!("h": Val::Px(10.));

    // min height
    tw!("min-h-0 hover:min-h-0.5 focus:min-h-px");
    tw!("min-h-0.5");
    tw!("min-h-px");
    tw!("min-h-1/2");
    tw!("min-h-full");
    tw!("min-h": Val::Px(10.));

    // max height
    tw!("max-h-0 hover:max-h-0.5 focus:max-h-px");
    tw!("max-h-0.5");
    tw!("max-h-px");
    tw!("max-h-1/2");
    tw!("max-h-full");
    tw!("max-h": Val::Px(10.));

    // size
    tw!("size-0");
    tw!("size-0.5");
    tw!("size-px");
    tw!("size-1/2");
    tw!("size-full");
    tw!("size": Val::Px(10.));

    // font size
    tw!("text-xs hover:text-sm focus:text-base");
    tw!("text-sm");
    tw!("text-base");
    tw!("text-lg");
    tw!("text-xl");
    tw!("text-2xl");
    tw!("text-3xl");
    tw!("text-4xl");
    tw!("text-5xl");
    tw!("text-6xl");
    tw!("text-7xl");
    tw!("text-8xl");
    tw!("text-9xl");
    tw!("text": 10.);

    // font smoothing
    tw!("antialiased");

    // text align
    tw!("text-left hover:text-center focus:text-right");
    tw!("text-center");
    tw!("text-right");
    tw!("text-justify");
    tw!("text-align": JustifyText::Center);

    // text color
    tw!("text-transparent hover:text-transparent/50 focus:text-black");
    tw!("text-transparent/50");
    tw!("text-black");
    tw!("text-black/50");
    tw!("text-white");
    tw!("text-white/50");
    tw!("text-gray-100");
    tw!("text-gray-100/50");
    tw!("text-color": Color::WHITE);

    // word break
    tw!("break-words hover:break-all focus:break-all");
    tw!("break-all");
    tw!("break": LineBreak::NoWrap);

    // background color
    tw!("bg-transparent hover:bg-black focus:bg-white");
    tw!("bg-transparent/50");
    tw!("bg-black");
    tw!("bg-black/50");
    tw!("bg-white");
    tw!("bg-white/50");
    tw!("bg-gray-100");
    tw!("bg-gray-100/50");
    tw!("bg": Color::WHITE);

    // border radius
    tw!("rounded-none");
    tw!("rounded-sm");
    tw!("rounded");
    tw!("rounded": Val::Px(10.));
    tw!("rounded-full");
    tw!("rounded-t-none");
    tw!("rounded-t-sm");
    tw!("rounded-t");
    tw!("rounded-t-full");
    tw!("rounded-t": Val::Px(10.));
    tw!("rounded-r-none");
    tw!("rounded-r-sm");
    tw!("rounded-r");
    tw!("rounded-r-full");
    tw!("rounded-r": Val::Px(10.));
    tw!("rounded-b-none");
    tw!("rounded-b-sm");
    tw!("rounded-b");
    tw!("rounded-b-full");
    tw!("rounded-b": Val::Px(10.));
    tw!("rounded-l-none");
    tw!("rounded-l-sm");
    tw!("rounded-l");
    tw!("rounded-l-full");
    tw!("rounded-l": Val::Px(10.));
    tw!("rounded-tl-none hover:rounded-tl-sm focus:rounded-tl");
    tw!("rounded-tl-sm");
    tw!("rounded-tl");
    tw!("rounded-tl-full");
    tw!("rounded-tl": Val::Px(10.));
    tw!("rounded-tr-none");
    tw!("rounded-tr-sm");
    tw!("rounded-tr");
    tw!("rounded-tr-full");
    tw!("rounded-tr": Val::Px(10.));
    tw!("rounded-br-none");
    tw!("rounded-br-sm");
    tw!("rounded-br");
    tw!("rounded-br-full");
    tw!("rounded-br": Val::Px(10.));
    tw!("rounded-bl-none");
    tw!("rounded-bl-sm");
    tw!("rounded-bl");
    tw!("rounded-bl-full");
    tw!("rounded-bl": Val::Px(10.));

    // border width
    tw!("border-0");
    tw!("border");
    tw!("border": Val::Px(10.));
    tw!("border-x-0");
    tw!("border-x");
    tw!("border-x": Val::Px(10.));
    tw!("border-y-0");
    tw!("border-y");
    tw!("border-y": Val::Px(10.));
    tw!("border-t-0");
    tw!("border-t hover:border-t-2 focus:border-t-3");
    tw!("border-t": Val::Px(10.));
    tw!("border-r-0");
    tw!("border-r");
    tw!("border-r": Val::Px(10.));
    tw!("border-b-0");
    tw!("border-b");
    tw!("border-b": Val::Px(10.));
    tw!("border-l-0");
    tw!("border-l");
    tw!("border-l": Val::Px(10.));

    // border color
    tw!("border-transparent hover:border-black focus:border-white");
    tw!("border-transparent/50");
    tw!("border-black");
    tw!("border-black/50");
    tw!("border-white");
    tw!("border-white/50");
    tw!("border-gray-100");
    tw!("border-gray-100/50");
    tw!("border-color": Color::WHITE);

    // outline width
    tw!("outline-0 hover:outline-1 focus:outline-2");
    tw!("outline": Val::Px(10.));

    // outline color
    tw!("outline-transparent hover:outline-black focus:outline-white");
    tw!("outline-transparent/50");
    tw!("outline-black");
    tw!("outline-black/50");
    tw!("outline-white");
    tw!("outline-white/50");
    tw!("outline-gray-100/50");
    tw!("outline-color": Color::WHITE);

    // outline offset
    tw!("outline-offset-0 hover:outline-offset-1 focus:outline-offset-2");
    tw!("outline-offset": Val::Px(10.));
}
