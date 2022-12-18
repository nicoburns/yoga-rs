/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// @generated by gentest/gentest.rb from gentest/fixtures/YGAndroidNewsFeed.html

extern crate ordered_float;
extern crate yoga;

use yoga::*;

#[test]
fn test_android_news_feed() {
    let mut config = Config::new();

    let mut root = Node::new_with_config(&mut config);
    root.set_align_content(Align::Stretch);
    root.set_width(StyleUnit::Point(1080_f32.into()));

    let mut root_child0 = Node::new_with_config(&mut config);
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0.set_align_content(Align::Stretch);
    root_child0.insert_child(&mut root_child0_child0, 0);

    let mut root_child0_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

    let mut root_child0_child0_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0.set_align_items(Align::FlexStart);
    root_child0_child0_child0_child0.set_margin(Edge::Start, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child0_child0.set_margin(Edge::Top, StyleUnit::Point(24_f32.into()));
    root_child0_child0_child0.insert_child(&mut root_child0_child0_child0_child0, 0);

    let mut root_child0_child0_child0_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child0_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0.insert_child(&mut root_child0_child0_child0_child0_child0, 0);

    let mut root_child0_child0_child0_child0_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0_child0_child0.set_width(StyleUnit::Point(120_f32.into()));
    root_child0_child0_child0_child0_child0_child0.set_height(StyleUnit::Point(120_f32.into()));
    root_child0_child0_child0_child0_child0
        .insert_child(&mut root_child0_child0_child0_child0_child0_child0, 0);

    let mut root_child0_child0_child0_child0_child1 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0_child1.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0_child1.set_flex_shrink(1_f32);
    root_child0_child0_child0_child0_child1
        .set_margin(Edge::Right, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child0_child0_child1
        .set_padding(Edge::Left, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child0_child0_child1.set_padding(Edge::Top, StyleUnit::Point(21_f32.into()));
    root_child0_child0_child0_child0_child1
        .set_padding(Edge::Right, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child0_child0_child1
        .set_padding(Edge::Bottom, StyleUnit::Point(18_f32.into()));
    root_child0_child0_child0_child0.insert_child(&mut root_child0_child0_child0_child0_child1, 1);

    let mut root_child0_child0_child0_child0_child1_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0_child1_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child0_child0_child1_child0.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0_child1_child0.set_flex_shrink(1_f32);
    root_child0_child0_child0_child0_child1
        .insert_child(&mut root_child0_child0_child0_child0_child1_child0, 0);

    let mut root_child0_child0_child0_child0_child1_child1 = Node::new_with_config(&mut config);
    root_child0_child0_child0_child0_child1_child1.set_align_content(Align::Stretch);
    root_child0_child0_child0_child0_child1_child1.set_flex_shrink(1_f32);
    root_child0_child0_child0_child0_child1
        .insert_child(&mut root_child0_child0_child0_child0_child1_child1, 1);

    let mut root_child0_child0_child1 = Node::new_with_config(&mut config);
    root_child0_child0_child1.set_align_content(Align::Stretch);
    root_child0_child0.insert_child(&mut root_child0_child0_child1, 1);

    let mut root_child0_child0_child1_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child1_child0.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0.set_align_items(Align::FlexStart);
    root_child0_child0_child1_child0.set_margin(Edge::Start, StyleUnit::Point(174_f32.into()));
    root_child0_child0_child1_child0.set_margin(Edge::Top, StyleUnit::Point(24_f32.into()));
    root_child0_child0_child1.insert_child(&mut root_child0_child0_child1_child0, 0);

    let mut root_child0_child0_child1_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child1_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0.insert_child(&mut root_child0_child0_child1_child0_child0, 0);

    let mut root_child0_child0_child1_child0_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0_child0_child0.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0_child0_child0.set_width(StyleUnit::Point(72_f32.into()));
    root_child0_child0_child1_child0_child0_child0.set_height(StyleUnit::Point(72_f32.into()));
    root_child0_child0_child1_child0_child0
        .insert_child(&mut root_child0_child0_child1_child0_child0_child0, 0);

    let mut root_child0_child0_child1_child0_child1 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0_child1.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0_child1.set_flex_shrink(1_f32);
    root_child0_child0_child1_child0_child1
        .set_margin(Edge::Right, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child1_child0_child1
        .set_padding(Edge::Left, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child1_child0_child1.set_padding(Edge::Top, StyleUnit::Point(21_f32.into()));
    root_child0_child0_child1_child0_child1
        .set_padding(Edge::Right, StyleUnit::Point(36_f32.into()));
    root_child0_child0_child1_child0_child1
        .set_padding(Edge::Bottom, StyleUnit::Point(18_f32.into()));
    root_child0_child0_child1_child0.insert_child(&mut root_child0_child0_child1_child0_child1, 1);

    let mut root_child0_child0_child1_child0_child1_child0 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0_child1_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0_child1_child0_child1_child0.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0_child1_child0.set_flex_shrink(1_f32);
    root_child0_child0_child1_child0_child1
        .insert_child(&mut root_child0_child0_child1_child0_child1_child0, 0);

    let mut root_child0_child0_child1_child0_child1_child1 = Node::new_with_config(&mut config);
    root_child0_child0_child1_child0_child1_child1.set_align_content(Align::Stretch);
    root_child0_child0_child1_child0_child1_child1.set_flex_shrink(1_f32);
    root_child0_child0_child1_child0_child1
        .insert_child(&mut root_child0_child0_child1_child0_child1_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0_f32, root.get_layout_left());
    assert_eq!(0_f32, root.get_layout_top());
    assert_eq!(1080_f32, root.get_layout_width());
    assert_eq!(240_f32, root.get_layout_height());

    assert_eq!(0_f32, root_child0.get_layout_left());
    assert_eq!(0_f32, root_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0.get_layout_width());
    assert_eq!(240_f32, root_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0.get_layout_width());
    assert_eq!(240_f32, root_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0_child0.get_layout_width());
    assert_eq!(144_f32, root_child0_child0_child0.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child0_child0.get_layout_left());
    assert_eq!(24_f32, root_child0_child0_child0_child0.get_layout_top());
    assert_eq!(1044_f32, root_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child0.get_layout_top());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child0_child0.get_layout_top());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0_child0.get_layout_height());

    assert_eq!(120_f32, root_child0_child0_child0_child0_child1.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child0_child0_child1.get_layout_width());
    assert_eq!(39_f32, root_child0_child0_child0_child0_child1.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child0_child0_child1_child0.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child0_child0_child1_child0.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child0.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child0.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child0_child0_child1_child1.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child0_child0_child1_child1.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child1.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child1.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1.get_layout_left());
    assert_eq!(144_f32, root_child0_child0_child1.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0_child1.get_layout_width());
    assert_eq!(96_f32, root_child0_child0_child1.get_layout_height());

    assert_eq!(174_f32, root_child0_child0_child1_child0.get_layout_left());
    assert_eq!(24_f32, root_child0_child0_child1_child0.get_layout_top());
    assert_eq!(906_f32, root_child0_child0_child1_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child0.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child0_child0.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0_child0.get_layout_height());

    assert_eq!(72_f32, root_child0_child0_child1_child0_child1.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child1.get_layout_width());
    assert_eq!(39_f32, root_child0_child0_child1_child0_child1.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child1_child0_child1_child0.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child1_child0_child1_child0.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child0.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child0.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child1_child0_child1_child1.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child1_child0_child1_child1.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child1.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child1.get_layout_height());

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0_f32, root.get_layout_left());
    assert_eq!(0_f32, root.get_layout_top());
    assert_eq!(1080_f32, root.get_layout_width());
    assert_eq!(240_f32, root.get_layout_height());

    assert_eq!(0_f32, root_child0.get_layout_left());
    assert_eq!(0_f32, root_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0.get_layout_width());
    assert_eq!(240_f32, root_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0.get_layout_width());
    assert_eq!(240_f32, root_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0_child0.get_layout_width());
    assert_eq!(144_f32, root_child0_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0_child0.get_layout_left());
    assert_eq!(24_f32, root_child0_child0_child0_child0.get_layout_top());
    assert_eq!(1044_f32, root_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0.get_layout_height());

    assert_eq!(924_f32, root_child0_child0_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child0.get_layout_top());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child0_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child0_child0.get_layout_top());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0_child0.get_layout_width());
    assert_eq!(120_f32, root_child0_child0_child0_child0_child0_child0.get_layout_height());

    assert_eq!(816_f32, root_child0_child0_child0_child0_child1.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child0_child0_child1.get_layout_width());
    assert_eq!(39_f32, root_child0_child0_child0_child0_child1.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child0_child0_child1_child0.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child0_child0_child1_child0.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child0.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child0.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child0_child0_child1_child1.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child0_child0_child1_child1.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child1.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child0_child0_child1_child1.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1.get_layout_left());
    assert_eq!(144_f32, root_child0_child0_child1.get_layout_top());
    assert_eq!(1080_f32, root_child0_child0_child1.get_layout_width());
    assert_eq!(96_f32, root_child0_child0_child1.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1_child0.get_layout_left());
    assert_eq!(24_f32, root_child0_child0_child1_child0.get_layout_top());
    assert_eq!(906_f32, root_child0_child0_child1_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0.get_layout_height());

    assert_eq!(834_f32, root_child0_child0_child1_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child0.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0.get_layout_height());

    assert_eq!(0_f32, root_child0_child0_child1_child0_child0_child0.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child0_child0.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0_child0.get_layout_width());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child0_child0.get_layout_height());

    assert_eq!(726_f32, root_child0_child0_child1_child0_child1.get_layout_left());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1.get_layout_top());
    assert_eq!(72_f32, root_child0_child0_child1_child0_child1.get_layout_width());
    assert_eq!(39_f32, root_child0_child0_child1_child0_child1.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child1_child0_child1_child0.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child1_child0_child1_child0.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child0.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child0.get_layout_height());

    assert_eq!(36_f32, root_child0_child0_child1_child0_child1_child1.get_layout_left());
    assert_eq!(21_f32, root_child0_child0_child1_child0_child1_child1.get_layout_top());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child1.get_layout_width());
    assert_eq!(0_f32, root_child0_child0_child1_child0_child1_child1.get_layout_height());
}
